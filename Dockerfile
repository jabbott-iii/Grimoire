# check=error=true
#
# The Grimoire container image (ADR-0011, docs/container.md): the release's static musl
# binaries on distroless static. Only .github/workflows/release.yml builds it, from a
# context that `cargo xtask release image context` unpacks from the release archives:
#
#   linux/<arch>/cli/grimoire                  -> /usr/local/bin/grimoire (the entrypoint)
#   linux/<arch>/assets/grimoire-{mcp,tool}-*  -> /opt/grimoire/bin/, off PATH, so
#                                                 `grimoire doctor` in a container never
#                                                 counts them as installed on the host
#
# Rules, which `cargo xtask release image context` enforces:
# - The first line stays `# check=error=true`, so a failed build check fails the build.
# - COPY only: no RUN or ADD, so nothing runs during the build and no platform needs
#   emulation.
# - One FROM, pinned by its index digest on the same line, which Dependabot updates. A
#   new digest must be at least 7 days old (CI-PINS), and the workflow verifies its
#   signature before every build (BASE-IMAGE-P6).
# - No syntax or escape parser directive, and no line continuations: the frontend is the
#   one in the digest-pinned BuildKit image, so nothing is pulled by tag (BUILD-P6), and
#   every instruction is on one line, as the check reads it.
FROM gcr.io/distroless/static-debian13:nonroot@sha256:25ac8f0c3b2334a9eb05452266bd4697864a8c8ef0ef71e5e6480d7f92868329

ARG TARGETOS
ARG TARGETARCH
COPY --chmod=0755 ${TARGETOS}/${TARGETARCH}/cli/ /usr/local/bin/
COPY --chmod=0755 ${TARGETOS}/${TARGETARCH}/assets/ /opt/grimoire/bin/
# Mount the repository here: docker run --mount type=bind,src="$PWD",dst=/repo ...
WORKDIR /repo
# distroless's nonroot user, stated explicitly.
USER 65532:65532
# Links the GHCR package to the repository on its first push (REGISTRY-P6).
LABEL org.opencontainers.image.source="https://github.com/jabbott-iii/Grimoire"
ENTRYPOINT ["/usr/local/bin/grimoire"]
CMD ["--help"]
