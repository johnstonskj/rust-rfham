//! Encoding tests for `rfham_rigs::protocol::cat::common`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    Level,
    protocol::{
        Command, Frequency,
        cat::elecraft::k2::{
            GetAntennaSelection, GetK2CommandMode, GetTransceiverId, GetVfoAAfGain,
            GetVfoAOperatingFrequency, GetVfoBAfGain, GetVfoBOperatingFrequency,
            GetVfoOperatingMode, K2CommandMode, OperatingMode, SelectedAntenna,
            SetAntennaSelection, SetK2CommandMode, SetVfoAAfGain, SetVfoAOperatingFrequency,
            SetVfoBAfGain, SetVfoBOperatingFrequency, SetVfoOperatingMode,
        },
    },
};
use strum::IntoEnumIterator;

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
fn get_transceiver_id_encodes() {
    assert_eq!(GetTransceiverId.to_message().unwrap(), b"ID;".to_vec());
}

#[test]
fn get_vfo_a_frequency_encodes() {
    assert_eq!(
        GetVfoAOperatingFrequency.to_message().unwrap(),
        b"FA;".to_vec()
    );
}

#[test]
fn set_vfo_a_frequency_encodes() {
    let cmd = SetVfoAOperatingFrequency {
        frequency: Frequency::from(14_074_000u64),
    };
    assert_eq!(cmd.to_message().unwrap(), b"FA00014074000;".to_vec());
}

#[test]
fn get_vfo_b_frequency_encodes() {
    assert_eq!(
        GetVfoBOperatingFrequency.to_message().unwrap(),
        b"FB;".to_vec()
    );
}

#[test]
fn set_vfo_b_frequency_encodes() {
    let cmd = SetVfoBOperatingFrequency {
        frequency: Frequency::from(7_074_000u64),
    };
    assert_eq!(cmd.to_message().unwrap(), b"FB00007074000;".to_vec());
}

#[test]
fn get_operating_mode_encodes() {
    assert_eq!(GetVfoOperatingMode.to_message().unwrap(), b"MD;".to_vec());
}

#[test]
fn set_operating_mode_encodes() {
    let cmd = SetVfoOperatingMode::to_cw();
    assert_eq!(cmd.to_message().unwrap(), b"MD3;".to_vec());
}

#[test]
fn set_operating_mode_assert_accepts_all_values() {
    for variant in OperatingMode::iter() {
        assert!(SetVfoOperatingMode { mode: variant }.validate().is_ok());
    }
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
    assert_eq!(GetAntennaSelection.to_message().unwrap(), b"AN;".to_vec());
}

#[test]
fn set_antenna_selection_encodes() {
    let cmd = SetAntennaSelection {
        antenna: SelectedAntenna::Antenna2,
    };
    assert_eq!(cmd.to_message().unwrap(), b"AN2;".to_vec());
}

#[test]
fn set_current_antenna_accepts_boundary_values() {
    assert!(
        SetAntennaSelection {
            antenna: SelectedAntenna::Antenna1
        }
        .validate()
        .is_ok()
    );
    assert!(
        SetAntennaSelection {
            antenna: SelectedAntenna::Antenna2
        }
        .validate()
        .is_ok()
    );
}
