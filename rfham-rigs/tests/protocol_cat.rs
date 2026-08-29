//! Integration test entry point for `rfham_rigs::protocol::cat`.
//!
//! Cargo only auto-discovers `.rs` files that are direct children of `tests/` as separate test
//! binaries, so this file exists purely to pull in the mirrored `protocol/cat/...` test tree
//! below via `#[path]`. The actual test modules live in that tree, one file per source module,
//! each named `test_<module>`, mirroring `src/protocol/cat/...` exactly.

#[path = "protocol/cat/mod.rs"]
mod cat;
