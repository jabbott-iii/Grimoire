#!/usr/bin/env bash
# The container image's registry steps (ADR-0011): push by digest, tag the version, move
# `latest`. The release runs them (release.yml publish-image, then sign-image.yml), and every
# pull request runs them against a local registry (release.yml verify-image), so the code that
# guards the release tags never runs for the first time in a release.
#
#   image-tags.sh push   IMAGE DIGEST VERSION LAYOUT   push LAYOUT by digest, untagged
#   image-tags.sh tag    IMAGE DIGEST VERSION          tag VERSION
#   image-tags.sh latest IMAGE DIGEST VERSION -- VERIFY...
#                                                       move latest to DIGEST, unless it names
#                                                       a newer release: then leave it if VERIFY
#                                                       accepts it, and stop if not
#
# VERIFY is a command that exits 0 when the image reference appended to it carries the release
# signature: cosign verify with the image's signing identity (sign-image.yml's), or a throwaway
# key in dry-runs.
#
# Rules: a VERSION tag that names another digest is an error, never overwritten. A tag
# lookup that fails for any reason other than the registry's "unknown" answer stops the
# script, so a tag it couldn't read is never treated as absent. Needs crane and jq.
set -euo pipefail
# Command substitutions keep set -e, so a failed lookup inside one still stops the script.
shopt -s inherit_errexit

die() {
  echo "::error::$*" >&2
  exit 1
}

# Prints the digest REF names, or nothing when the registry says it doesn't exist.
lookup() {
  local ref="$1" err out
  err="$(mktemp)"
  if out="$(crane digest "${ref}" 2>"${err}")"; then
    rm -f "${err}"
    printf '%s' "${out}"
  elif grep -qE 'MANIFEST_UNKNOWN|NAME_UNKNOWN' "${err}"; then
    rm -f "${err}"
  else
    cat "${err}" >&2
    rm -f "${err}"
    die "cannot tell whether ${ref} exists"
  fi
}

# Fails unless VERSION isn't tagged yet or already names DIGEST.
version_free() {
  local image="$1" digest="$2" version="$3" current
  current="$(lookup "${image}:${version}")" || exit 1
  if [ -n "${current}" ] && [ "${current}" != "${digest}" ]; then
    die "${image}:${version} already points at ${current}, not at ${digest}"
  fi
  printf '%s' "${current}"
}

semver() {
  [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]
}

cmd="${1:-}"
[ "$#" -ge 4 ] || die "usage: image-tags.sh push|tag|latest IMAGE DIGEST VERSION ..."
image="$2"
digest="$3"
version="$4"
[[ "${digest}" =~ ^sha256:[0-9a-f]{64}$ ]] || die "'${digest}' is not a sha256 digest"
semver "${version}" || die "'${version}' is not a release version"

case "${cmd}" in
  push)
    [ "$#" -eq 5 ] || die "usage: image-tags.sh push IMAGE DIGEST VERSION LAYOUT"
    version_free "${image}" "${digest}" "${version}" > /dev/null
    pushed="$(mktemp)"
    crane push --image-refs "${pushed}" "$5" "${image}@${digest}"
    if [ "$(cat "${pushed}")" != "${image}@${digest}" ]; then
      die "crane pushed $(cat "${pushed}"), not ${image}@${digest}"
    fi
    rm -f "${pushed}"
    echo "pushed ${image}@${digest}"
    ;;
  tag)
    [ "$#" -eq 4 ] || die "usage: image-tags.sh tag IMAGE DIGEST VERSION"
    current="$(version_free "${image}" "${digest}" "${version}")" || exit 1
    if [ -n "${current}" ]; then
      echo "${image}:${version} already points at ${digest}"
    else
      crane tag "${image}@${digest}" "${version}"
      echo "tagged ${image}:${version}"
    fi
    ;;
  latest)
    [ "$#" -ge 6 ] && [ "$5" = "--" ] || die "usage: image-tags.sh latest IMAGE DIGEST VERSION -- VERIFY..."
    shift 5
    current="$(lookup "${image}:latest")" || exit 1
    if [ -n "${current}" ] && [ "${current}" != "${digest}" ]; then
      named="$(crane manifest "${image}@${current}" | jq -r '.annotations["org.opencontainers.image.version"] // empty')" || exit 1
      if semver "${named}" && [ "$(printf '%s\n' "${named}" "${version}" | sort -V | tail -n 1)" != "${version}" ]; then
        # latest claims a newer release. Believe it only if it carries the release signature:
        # anyone who can write to the package could claim a high version. A claim that doesn't
        # verify (a forged image, or a verification that failed) stops here, so latest is never
        # moved back to an older release without someone looking.
        if "$@" "${image}@${current}" > /dev/null 2>&1; then
          echo "latest is ${named}, newer than ${version}; leaving it"
          exit 0
        fi
        die "latest (${current}) claims ${named}, newer than ${version}, but doesn't verify with the release signature: investigate before moving it"
      fi
      if [ -n "${named}" ]; then
        echo "latest was ${named} (${current}); moving it to ${version}"
      else
        echo "::warning::latest named ${current}, which has no release version; moving it"
      fi
    fi
    crane tag "${image}@${digest}" latest
    echo "latest is now ${version} (${digest})"
    ;;
  *)
    die "usage: image-tags.sh push|tag|latest IMAGE DIGEST VERSION ..."
    ;;
esac
