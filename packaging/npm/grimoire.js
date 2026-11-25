#!/usr/bin/env node
// Launcher for the agent-grimoire npm package.
//
// `cargo xtask release channels` copies this file to bin/grimoire.js in the
// launcher package. The grimoire binary itself ships in one optional
// dependency per platform, @agent-grimoire/<os>-<cpu>, and npm installs only
// the one whose os and cpu match. This script finds that binary and runs it,
// passing arguments, stdio and the exit status through. No dependencies, no
// install scripts, no network. See docs/05-release.md.

"use strict";

const { spawn } = require("child_process");
const os = require("os");

// process.platform and process.arch -> platform package.
const PACKAGES = {
  "linux x64": "@agent-grimoire/linux-x64",
  "linux arm64": "@agent-grimoire/linux-arm64",
  "darwin x64": "@agent-grimoire/darwin-x64",
  "darwin arm64": "@agent-grimoire/darwin-arm64",
  "win32 x64": "@agent-grimoire/win32-x64",
  "win32 arm64": "@agent-grimoire/win32-arm64",
};

// Signals the launcher passes on to grimoire, so that stopping the launcher (a CI
// timeout, `kill`, a closed terminal) stops grimoire too instead of orphaning it.
const FORWARDED = ["SIGINT", "SIGTERM", "SIGHUP"];

function fail(message) {
  process.stderr.write(`agent-grimoire: error: ${message}\n`);
  process.exitCode = 1;
}

function main() {
  const pkg = PACKAGES[`${process.platform} ${process.arch}`];
  if (!pkg) {
    fail(
      `there is no grimoire binary for ${process.platform}-${process.arch}. ` +
        "Supported: linux, darwin and win32 on x64 and arm64.",
    );
    return;
  }

  const exe = process.platform === "win32" ? "grimoire.exe" : "grimoire";
  let bin;
  try {
    bin = require.resolve(`${pkg}/bin/${exe}`);
  } catch {
    fail(
      `the package ${pkg} is not installed. It is an optional dependency ` +
        "of agent-grimoire, so reinstall agent-grimoire without --omit=optional.",
    );
    return;
  }

  const child = spawn(bin, process.argv.slice(2), {
    stdio: "inherit",
    shell: false,
  });
  const forward = (signal) => {
    // A terminal's Ctrl-C reaches grimoire directly too; sending it again is harmless.
    child.kill(signal);
  };
  for (const signal of FORWARDED) {
    process.on(signal, forward);
  }
  child.on("error", (error) => {
    fail(`could not run ${bin}: ${error.message}`);
  });
  child.on("exit", (code, signal) => {
    for (const s of FORWARDED) {
      process.removeListener(s, forward);
    }
    // Shell convention: death by signal N exits with 128 + N.
    process.exitCode = signal ? 128 + (os.constants.signals[signal] || 0) : (code ?? 1);
  });
}

// Setting exitCode (rather than calling process.exit) lets stderr flush.
main();
