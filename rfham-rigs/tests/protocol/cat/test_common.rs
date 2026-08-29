//! Encoding tests for `rfham_rigs::protocol::cat::common`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    Level,
    error::RigError,
    protocol::{
        Command, Frequency,
        cat::common::{
            GetCurrentAntenna, GetTransceiverId, GetVfoAAfGain, GetVfoAFrequency, GetVfoBAfGain,
            GetVfoBFrequency, SetCurrentAntenna, SetVfoAAfGain, SetVfoAFrequency, SetVfoBAfGain,
            SetVfoBFrequency,
        },
    },
};

#[test]
fn get_transceiver_id_encodes() {
    assert_eq!(GetTransceiverId.to_message().unwrap(), b"ID;".to_vec());
}

#[test]
fn get_vfo_a_frequency_encodes() {
    assert_eq!(GetVfoAFrequency.to_message().unwrap(), b"FA;".to_vec());
}

#[test]
fn set_vfo_a_frequency_encodes() {
    let cmd = SetVfoAFrequency {
        frequency: Frequency::from(14_074_000u64),
    };
    assert_eq!(cmd.to_message().unwrap(), b"FA00014074000;".to_vec());
}

#[test]
fn get_vfo_b_frequency_encodes() {
    assert_eq!(GetVfoBFrequency.to_message().unwrap(), b"FB;".to_vec());
}

#[test]
fn set_vfo_b_frequency_encodes() {
    let cmd = SetVfoBFrequency {
        frequency: Frequency::from(7_074_000u64),
    };
    assert_eq!(cmd.to_message().unwrap(), b"FB00007074000;".to_vec());
}

#[test]
fn get_vfo_a_af_gain_encodes() {
    assert_eq!(GetVfoAAfGain.to_message().unwrap(), b"AG0;".to_vec());
}

#[test]
fn set_vfo_a_af_gain_encodes() {
    // NOTE: SetVfoAAfGain/SetVfoBAfGain send `Level`'s raw byte value verbatim (not an ASCII
    // digit) — this documents current behavior, which is itself flagged "Unverified" in the
    // source doc string pending confirmation against a real radio's programmer's reference.
    let cmd = SetVfoAAfGain {
        level: Level::from(5u8),
    };
    assert_eq!(cmd.to_message().unwrap(), vec![b'A', b'G', b'0', 5, b';']);
}

#[test]
fn get_vfo_b_af_gain_encodes() {
    assert_eq!(GetVfoBAfGain.to_message().unwrap(), b"AG1;".to_vec());
}

#[test]
fn set_vfo_b_af_gain_encodes() {
    let cmd = SetVfoBAfGain {
        level: Level::from(9u8),
    };
    assert_eq!(cmd.to_message().unwrap(), vec![b'A', b'G', b'1', 9, b';']);
}

#[test]
fn get_current_antenna_encodes() {
    assert_eq!(GetCurrentAntenna.to_message().unwrap(), b"AN;".to_vec());
}

#[test]
fn set_current_antenna_encodes() {
    let cmd = SetCurrentAntenna { antenna: 2 };
    assert_eq!(cmd.to_message().unwrap(), b"AN2;".to_vec());
}

#[test]
fn set_current_antenna_accepts_boundary_values() {
    assert!(SetCurrentAntenna { antenna: 1 }.validate().is_ok());
    assert!(SetCurrentAntenna { antenna: 3 }.validate().is_ok());
}

#[test]
fn set_current_antenna_rejects_out_of_range() {
    assert!(matches!(
        SetCurrentAntenna { antenna: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetCurrentAntenna { antenna: 4 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}
