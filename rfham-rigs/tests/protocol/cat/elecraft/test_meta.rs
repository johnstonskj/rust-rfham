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

#[test]
fn get_k2_command_mode_encodes() {
    assert_eq!(GetK2CommandMode.to_message().unwrap(), b"K2;".to_vec());
}

#[test]
fn set_k2_command_mode_encodes_normal() {
    let cmd = SetK2CommandMode {
        mode: K2CommandMode {
            extended: false,
            rtty_off: false,
        },
    };
    assert_eq!(cmd.to_message().unwrap(), b"K20;".to_vec());
}

#[test]
fn set_k2_command_mode_encodes_extended_rtty_off() {
    let cmd = SetK2CommandMode {
        mode: K2CommandMode {
            extended: true,
            rtty_off: true,
        },
    };
    assert_eq!(cmd.to_message().unwrap(), b"K23;".to_vec());
}

#[test]
fn get_k3_command_mode_encodes() {
    assert_eq!(GetK3CommandMode.to_message().unwrap(), b"K3;".to_vec());
}

#[test]
fn set_k3_command_mode_encodes_normal() {
    let cmd = SetK3CommandMode {
        mode: K3CommandMode { extended: false },
    };
    assert_eq!(cmd.to_message().unwrap(), b"K30;".to_vec());
}

#[test]
fn set_k3_command_mode_encodes_extended() {
    let cmd = SetK3CommandMode {
        mode: K3CommandMode { extended: true },
    };
    assert_eq!(cmd.to_message().unwrap(), b"K31;".to_vec());
}

#[test]
fn get_k4_command_mode_encodes() {
    assert_eq!(GetK4CommandMode.to_message().unwrap(), b"K4;".to_vec());
}

#[test]
fn set_k4_command_mode_encodes_normal() {
    let cmd = SetK4CommandMode {
        mode: K4CommandMode { advanced: false },
    };
    assert_eq!(cmd.to_message().unwrap(), b"K40;".to_vec());
}

#[test]
fn set_k4_command_mode_encodes_advanced() {
    let cmd = SetK4CommandMode {
        mode: K4CommandMode { advanced: true },
    };
    assert_eq!(cmd.to_message().unwrap(), b"K41;".to_vec());
}
