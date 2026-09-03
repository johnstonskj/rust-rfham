//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::meta`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`).
//! Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::protocol::{
    Command,
    cat::elecraft::meta::{
        GetK2CommandMode, GetK3CommandMode, GetK4CommandMode, K2CommandMode, K3CommandMode,
        K4CommandMode, SetK2CommandMode, SetK3CommandMode, SetK4CommandMode,
    },
};
