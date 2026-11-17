//! Root-confined, atomic installation of Grimoire outputs (`architecture.md` §7).
//!
//! - [`block`]: managed blocks in shared Markdown files.
//! - [`entry`]: owned entries in JSON, JSONC and TOML config files.
//! - [`fs`]: root-confined, symlink-refusing, atomic file access.
//! - [`plan`]: the pure planner that compares desired outputs, the lock and disk.
//! - [`apply`]: applying a plan, building the next lock, and verifying a lock.

pub mod apply;
pub mod block;
pub mod entry;
pub mod fs;
pub mod hash;
pub mod plan;
