#!/bin/sh
# Installs the grimoire command-line tool on Linux and macOS.
#
# This file is generated for each release by `cargo xtask release channels`
# from the template packaging/install/install.sh, which fills in the version
# and the SHA-256 of each archive. See docs/05-release.md and
# docs/adr/0010-language-policy-exceptions.md.
#
# It downloads one archive over HTTPS, checks its SHA-256 against the value
# embedded below before extracting it, and puts `grimoire` into one directory.
# It never edits shell startup files or PATH. It sends nothing anywhere: no
# telemetry, no receipts, no tokens, no updater.
#
# Usage: sh install.sh [--to DIR]   (see --help)

set -eu
LC_ALL=C
export LC_ALL
unset CDPATH

# Filled in by `cargo xtask release channels`.
VERSION='@@VERSION@@'

REPO_URL=https://github.com/jabbott-iii/Grimoire

usage() {
  cat <<EOF
Install grimoire $VERSION.

Usage: sh install.sh [--to DIR]

Options:
  --to DIR    Install into DIR, creating it if needed.
  -h, --help  Show this help.

Environment:
  GRIMOIRE_INSTALL_DIR   Install directory when --to is not given.
                         Default: \$HOME/.local/bin
  GRIMOIRE_DOWNLOAD_URL  Base URL to download the archive from. Default:
                         $REPO_URL/releases/download/v$VERSION
                         It must start with https://. Plain http:// is allowed
                         only for 127.0.0.1 and localhost, for testing.

The archive's SHA-256 is checked against the value embedded in this script
before anything is extracted. Shell startup files and PATH are never changed.
EOF
}

say() { printf '%s\n' "$*"; }
die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}
have() { command -v "$1" >/dev/null 2>&1; }

# Temporary files, removed on any exit.
tmp=
staged=
cleanup() {
  if [ -n "$staged" ]; then rm -f -- "$staged"; fi
  if [ -n "$tmp" ]; then rm -rf -- "$tmp"; fi
}
trap cleanup EXIT
trap 'exit 1' HUP INT TERM

is_sha256() {
  case $1 in
    *[!0-9a-f]*) return 1 ;;
  esac
  [ ${#1} -eq 64 ]
}

# Sets `triple` (the Rust target to install) and `expected` (its SHA-256).
detect_target() {
  os=$(uname -s)
  arch=$(uname -m)
  case $os in
    Linux)
      # The Linux builds are static musl binaries, so any libc will do.
      case $arch in
        x86_64 | amd64) triple=x86_64-unknown-linux-musl ;;
        aarch64 | arm64) triple=aarch64-unknown-linux-musl ;;
        *) die "there is no grimoire build for Linux on $arch" ;;
      esac
      ;;
    Darwin)
      case $arch in
        arm64) triple=aarch64-apple-darwin ;;
        x86_64)
          # Under Rosetta 2 on Apple silicon, uname reports x86_64. Apple
          # documents sysctl.proc_translated as 1 for a translated process
          # and 0 for a native one (what Intel Macs print is not documented).
          # So anything but exactly 1 means an Intel Mac.
          if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null || true)" = 1 ]; then
            triple=aarch64-apple-darwin
          else
            triple=x86_64-apple-darwin
          fi
          ;;
        *) die "there is no grimoire build for macOS on $arch" ;;
      esac
      ;;
    MINGW* | MSYS* | CYGWIN*) die "on Windows, use install.ps1 instead" ;;
    *) die "there is no grimoire build for $os on $arch" ;;
  esac

  # Filled in by `cargo xtask release channels`. Empty means "not built".
  case $triple in
    x86_64-unknown-linux-musl) expected='@@SHA256:x86_64-unknown-linux-musl@@' ;;
    aarch64-unknown-linux-musl) expected='@@SHA256:aarch64-unknown-linux-musl@@' ;;
    x86_64-apple-darwin) expected='@@SHA256:x86_64-apple-darwin@@' ;;
    aarch64-apple-darwin) expected='@@SHA256:aarch64-apple-darwin@@' ;;
  esac
}

# Sets `proto` for curl. Accepts https://, and plain http:// only for the
# loopback host. The character set leaves out `@`, so no user info can hide
# the real host.
check_download_url() {
  case $1 in
    *[!A-Za-z0-9._~:/%+-]*)
      die "GRIMOIRE_DOWNLOAD_URL may contain only letters, digits and ._~:/%+-"
      ;;
  esac
  case $1 in
    https://?*)
      proto='=https'
      return 0
      ;;
    http://127.0.0.1 | http://127.0.0.1/* | http://localhost | http://localhost/*)
      proto='=http'
      return 0
      ;;
    http://127.0.0.1:* | http://localhost:*)
      port=${1#http://*:}
      port=${port%%/*}
      case $port in
        '' | *[!0-9]*) ;;
        *)
          proto='=http'
          return 0
          ;;
      esac
      ;;
  esac
  die "GRIMOIRE_DOWNLOAD_URL must start with https:// (plain http:// is allowed only for 127.0.0.1 and localhost)"
}

# download URL FILE
# curl only: wget can't refuse a redirect from https:// to http:// (--https-only applies
# to recursive downloads), and BusyBox wget has no --https-only at all.
download() {
  have curl || die "curl is needed to download grimoire (on Alpine: apk add curl)"
  curl --proto "$proto" --tlsv1.2 --fail --location --silent --show-error \
    --output "$2" "$1"
}

# Sets `actual` to the SHA-256 of FILE. Fails closed without a tool.
sha256_of() {
  if have sha256sum; then
    out=$(sha256sum <"$1") || die "sha256sum failed"
  elif have shasum; then
    out=$(shasum -a 256 <"$1") || die "shasum failed"
  elif have openssl; then
    out=$(openssl dgst -sha256 <"$1") || die "openssl failed"
    # Prints "SHA2-256(stdin)= <hash>" or similar: keep the last field.
    out=${out##* }
  else
    die "no SHA-256 tool found (sha256sum, shasum or openssl), so the download cannot be verified"
  fi
  actual=${out%% *}
}

dir=
while [ $# -gt 0 ]; do
  case $1 in
    --to)
      if [ $# -lt 2 ] || [ -z "$2" ]; then die "--to needs a directory"; fi
      dir=$2
      shift 2
      ;;
    --to=*)
      dir=${1#--to=}
      if [ -z "$dir" ]; then die "--to needs a directory"; fi
      shift
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *) die "unknown argument: $1 (see --help)" ;;
  esac
done

case $VERSION in
  *@*) die "this is the unrendered template; use the install.sh attached to a Grimoire release" ;;
esac

if [ -z "$dir" ]; then dir=${GRIMOIRE_INSTALL_DIR:-}; fi
if [ -z "$dir" ]; then
  if [ -z "${HOME:-}" ]; then
    die "HOME is not set; pass --to DIR or set GRIMOIRE_INSTALL_DIR"
  fi
  dir=$HOME/.local/bin
fi

for tool in uname mktemp tar; do
  have "$tool" || die "$tool is needed to install grimoire"
done

detect_target
if [ -z "$expected" ]; then
  die "this script has no SHA-256 for $triple, so it cannot verify the download"
fi
is_sha256 "$expected" || die "the SHA-256 embedded for $triple is malformed"

base=${GRIMOIRE_DOWNLOAD_URL:-$REPO_URL/releases/download/v$VERSION}
check_download_url "$base"
name=agent-grimoire-$VERSION-$triple.tar.gz
url=${base%/}/$name

tmp=$(mktemp -d "${TMPDIR:-/tmp}/grimoire-install.XXXXXX") ||
  die "could not create a temporary directory"

say "downloading $url"
download "$url" "$tmp/$name" || die "could not download $url"

sha256_of "$tmp/$name"
if ! is_sha256 "$actual" || [ "$actual" != "$expected" ]; then
  die "SHA-256 mismatch for $name: expected $expected, got $actual"
fi
say "verified SHA-256 $actual"

# Only now, with the archive verified, extract it. The archive is flat.
mkdir -- "$tmp/x"
tar -xzf "$tmp/$name" -C "$tmp/x" || die "could not extract $name"
bin=$tmp/x/grimoire
if [ ! -f "$bin" ] || [ -h "$bin" ]; then die "$name has no grimoire binary"; fi

mkdir -p -- "$dir" || die "could not create $dir"
dir=$(cd -- "$dir" && pwd) || die "could not enter $dir"
dest=${dir%/}/grimoire
if [ -d "$dest" ]; then die "$dest is a directory"; fi

# Stage a copy next to the destination, check that it runs, then rename it
# over the old binary, so `grimoire` is never missing or half-written.
staged=$(mktemp "${dir%/}/.grimoire.XXXXXX") || die "could not write to $dir"
cp -- "$bin" "$staged" || die "could not write to $dir"
chmod 755 "$staged" || die "could not make $staged executable"
if ! "$staged" --version >/dev/null 2>&1; then
  die "the downloaded grimoire does not run on this machine; nothing was replaced"
fi
mv -f -- "$staged" "$dest" || die "could not replace $dest"
staged=

say "installed grimoire $VERSION to $dest"
case ":${PATH:-}:" in
  *":$dir:"* | *":$dir/:"*) ;;
  *)
    say "$dir is not on your PATH. To run grimoire by name, add it in your shell's startup file, for example:"
    say "  export PATH=\"$dir:\$PATH\""
    ;;
esac
