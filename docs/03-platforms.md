# 03 — Platforms

This document lists the platforms Grimoire supports, the order work moves through them, what each one needs, and what is tested on every one. The release targets are defined in code, in `xtask/src/release/targets.rs`; this document describes that table and the CI matrix in `.github/workflows/ci.yml`.

## Targets

A target is a platform the binary is built for, named by its Rust target triple. Each release builds the CLI and every asset binary for six targets, each on a native GitHub-hosted runner, so no cross-compiling toolchain is needed and the smoke test (a quick `--version` run) runs on the real operating system ([ADR-0009](adr/0009-release-pipeline.md), the release pipeline).

| Target | Build runner | In pull-request dry-runs | Image platform | Notes |
|---|---|---|---|---|
| `x86_64-unknown-linux-musl` | `ubuntu-24.04` | yes | `linux/amd64` | Static, so it runs on any Linux. Pure-Rust dependencies only; the CLI has no TLS |
| `aarch64-unknown-linux-musl` | `ubuntu-24.04-arm` | yes | `linux/arm64` | Static. Native arm64 runner, free for public repositories |
| `x86_64-apple-darwin` | `macos-15-intel` | no | | GitHub plans to drop Intel macOS "after the macOS 15 image is retired in Fall 2027". The fallback is cross-compiling from arm64 (backlog in [`roadmap.md`](roadmap.md)) |
| `aarch64-apple-darwin` | `macos-15` | yes | | |
| `x86_64-pc-windows-msvc` | `windows-2025` | yes | | The C runtime is linked statically (`-Ctarget-feature=+crt-static`), so no Visual C++ runtime is needed ([build profiles](05-release.md#build-profiles)) |
| `aarch64-pc-windows-msvc` | `windows-11-arm` | no | | Native arm64 runner |

Runner labels are pinned explicitly, never `ubuntu-latest`, because a `-latest` label moves to a new image on GitHub's schedule (`RUNNERS` in [`specs.md`](specs.md)). The two Linux targets are also the container image's two platforms ([the container image spec](subsystems/container-image.md)).

## The order work moves through them

1. **Linux first.** Local work and agent sessions run on Linux, and every check in "Everyday commands" in `AGENTS.md` runs there.
2. **Then macOS and Windows, in CI.** They run only after a push: `ci.yml`'s `test` job runs the whole test suite on `ubuntu-24.04`, `macos-15` and `windows-2025`. The MSRV build (`cargo +1.96.0 check`) runs on `ubuntu-24.04`.
3. **Then GitHub itself.** The workflows, the ruleset, the Release dry-run (four of the six targets, the image on both platforms) and the signing jobs run only on GitHub.

A task that still needs a later environment carries a `⏳ needs …` line in [`roadmap.md`](roadmap.md) until someone runs that check.

## What each platform needs

- **Linux:** the musl targets give static binaries. The install path's directory handle uses `openat2(RESOLVE_BENEATH)`, a system call that refuses to resolve a path outside the directory ([threat F4](01-architecture.md#threats-and-mitigations)). On a filesystem that supports it, temporary files are unnamed until the final rename, so a crash rarely leaves one behind ([known gaps](01-architecture.md#known-gaps-and-residual-risks)).
- **macOS and Windows:** their file systems ignore case by default, so two paths that differ only in case are refused before any write ([threat F12](01-architecture.md#threats-and-mitigations)).
- **Windows:**
  - Device names such as `CON` and `COM1`, drive and UNC prefixes, and `:` are refused in every path ([threats F1 and F2](01-architecture.md#threats-and-mitigations)).
  - cap-std resolves each path one component at a time, but performs the last step of creating a directory, a rename or a removal by path ([known gaps](01-architecture.md#known-gaps-and-residual-risks)).
  - A file Grimoire removes is closed before its directory is pruned, because cap-std opens Windows directories without `FILE_SHARE_DELETE`.
  - Windows ACLs (access control lists, Windows' file permissions) are not copied when a file is replaced; Unix mode bits are ([threat F17](01-architecture.md#threats-and-mitigations)).
- **Line endings:** a checkout with `core.autocrlf` (the Git setting that turns `\n` into `\r\n` on Windows) must not read as drift. Text hashes fold `\r\n` to `\n` ([threat F13](01-architecture.md#threats-and-mitigations)).

## Invariants on every platform

- Every test binary runs on Linux, macOS and Windows in CI: `cargo test --workspace --locked --no-fail-fast`, so one failing binary can't hide failures in the others.
- The tests of symbolic links, Unix permission bits and FIFOs (named pipes) are Unix-only (`#[cfg(unix)]`): in `agent-grimoire-install`'s `fs.rs`, `agent-grimoire-catalog`'s `tests/catalog.rs`, the CLI's `tests/cli.rs`, `xtask`'s `marketplace.rs`, `secret-guard`'s `scan.rs` and `project-docs`'s `docs.rs`. Symbolic-link detection has no Windows test (backlog in [`roadmap.md`](roadmap.md)).
- Every release target builds its binaries with the same flags and the `dist` profile, and `cargo xtask release verify` checks that every channel carries the same binary ([the release spec](subsystems/release.md)).
