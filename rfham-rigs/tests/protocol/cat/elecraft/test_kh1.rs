//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::kh1`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    error::RigError,
    protocol::{
        Command,
        cat::elecraft::kh1::{
            DumpLog, EmulateButtonHold, EmulateButtonTap, EmulateEncoderRotation,
            EmulateHandKeyPress, Encoder, EncoderDirection, GetDisplayText, GetFirmwareRevision,
            GetHelpInformation, GetMenuParameter, GetTransceiverId, GetTransceiverSerialNumber,
            GetTransceiverStatus, GetTransmitLowerLimit, GetTransmitUpperLimit, HandKeyState,
            LoadFirmware, LogAction, OperatingMode, SelectMenuItem, SetAfGain, SetDisplayText,
            SetMenuParameter, SetOperatingFrequency, SetOperatingMode, SetVfoOffset, TransmitBand,
        },
    },
};
use strum::IntoEnumIterator;

#[test]
fn set_speaker_gain_encodes() {
    let cmd = SetAfGain { level: 25 };
    assert_eq!(cmd.to_message().unwrap(), b"AG25;".to_vec());
}

#[test]
fn set_speaker_gain_accepts_boundary_values() {
    assert!(SetAfGain { level: 0 }.validate().is_ok());
    assert!(SetAfGain { level: 30 }.validate().is_ok());
}

#[test]
fn set_speaker_gain_rejects_out_of_range() {
    assert!(matches!(
        SetAfGain { level: 61 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn get_display_text_encodes() {
    assert_eq!(
        GetDisplayText::for_top_line().to_message().unwrap(),
        b"DS1;".to_vec()
    );
    assert_eq!(
        GetDisplayText::for_bottom_line().to_message().unwrap(),
        b"DS2;".to_vec()
    );
}

#[test]
fn set_display_text_encodes_and_pads_to_16_characters() {
    let cmd = SetDisplayText::for_top_line(b"HI".to_vec());
    // "DS" + line digit + space + 16-byte, space-padded text field + ';'.
    let mut expected = b"DS1 HI".to_vec();
    expected.extend(std::iter::repeat_n(b' ', 14));
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn set_display_text_truncates_text_longer_than_16_characters() {
    let cmd = SetDisplayText::for_bottom_line(b"0123456789ABCDEFGHIJ".to_vec());
    let mut expected = b"DS2 ".to_vec();
    expected.extend_from_slice(b"0123456789ABCDEF");
    expected.push(b';');
    assert_eq!(cmd.to_message().unwrap(), expected);
}

#[test]
fn set_front_panel_encodes() {
    assert_eq!(
        EmulateEncoderRotation {
            encoder: Encoder::AfGain,
            direction: EncoderDirection::Up
        }
        .to_message()
        .unwrap(),
        b"ENAU;".to_vec()
    );
    assert_eq!(
        EmulateEncoderRotation {
            encoder: Encoder::Vfo,
            direction: EncoderDirection::Down
        }
        .to_message()
        .unwrap(),
        b"ENVD;".to_vec()
    );
}

#[test]
fn set_operating_frequency_encodes() {
    // NOTE: the source doc string's example arithmetic (`14074000 x 10 Hz = 140,740,000 Hz =
    // 14.074 MHz`) is internally inconsistent -- 140,740,000 Hz is 140.74 MHz, not 14.074 MHz.
    // This does not affect wire encoding, which is a plain zero-padded-to-8-digits `u32`.
    let cmd = SetOperatingFrequency { freq_10hz: 1234 };
    assert_eq!(cmd.to_message().unwrap(), b"FA00001234;".to_vec());
}

#[test]
fn set_vfo_offset_encodes() {
    let cmd = SetVfoOffset { offset_hz: 42 };
    assert_eq!(cmd.to_message().unwrap(), b"FO42;".to_vec());
}

#[test]
fn set_filter_offset_accepts_boundary_values() {
    assert!(SetVfoOffset { offset_hz: 0 }.validate().is_ok());
    assert!(SetVfoOffset { offset_hz: 99 }.validate().is_ok());
}

#[test]
fn set_filter_offset_rejects_out_of_range() {
    assert!(matches!(
        SetVfoOffset { offset_hz: 100 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn get_power_status_encodes() {
    assert_eq!(GetHelpInformation.to_message().unwrap(), b"H;".to_vec());
}

#[test]
fn emulate_hand_key_press_encodes() {
    let cmd = EmulateHandKeyPress {
        state: HandKeyState::KeyDown,
    };
    assert_eq!(cmd.to_message().unwrap(), b"HK1;".to_vec());
}

#[test]
fn emulate_hand_key_press_accepts_all_values() {
    for variant in HandKeyState::iter() {
        assert!(
            EmulateHandKeyPress { state: variant }.validate().is_ok(),
            "HandKeyState::{variant:?} should be valid"
        );
    }
}

#[test]
fn get_transceiver_info_encodes() {
    assert_eq!(GetTransceiverId.to_message().unwrap(), b"I;".to_vec());
}

#[test]
fn set_led_brightness_encodes() {
    let cmd = LoadFirmware;
    assert_eq!(cmd.to_message().unwrap(), b"LD;".to_vec());
}

#[test]
fn send_cw_message_encodes() {
    let cmd = DumpLog {
        action: LogAction::Stop,
    };
    assert_eq!(cmd.to_message().unwrap(), b"LG1;".to_vec());
}

#[test]
fn send_cw_message_accepts_all_values() {
    for variant in LogAction::iter() {
        assert!(
            DumpLog { action: variant }.validate().is_ok(),
            "LogAction::{variant:?} should be valid"
        );
    }
}

#[test]
fn set_operating_mode_encodes() {
    assert_eq!(
        SetOperatingMode {
            mode: OperatingMode::LowerSideBand
        }
        .to_message()
        .unwrap(),
        b"MD0;".to_vec()
    );
    assert_eq!(
        SetOperatingMode {
            mode: OperatingMode::Data
        }
        .to_message()
        .unwrap(),
        b"MD4;".to_vec()
    );
}

#[test]
fn set_operating_mode_accepts_all_values() {
    for variant in OperatingMode::iter() {
        assert!(SetOperatingMode { mode: variant }.validate().is_ok());
    }
}

#[test]
fn select_menu_item_encodes() {
    let cmd = SelectMenuItem {
        item_id: [b'K', b' ', b'S'],
    };
    assert_eq!(cmd.to_message().unwrap(), b"MNK S;".to_vec());
}

#[test]
fn get_menu_parameter_encodes() {
    assert_eq!(GetMenuParameter.to_message().unwrap(), b"MP;".to_vec());
}

#[test]
fn set_menu_parameter_encodes() {
    let cmd = SetMenuParameter { value: 7 };
    assert_eq!(cmd.to_message().unwrap(), b"MP007;".to_vec());
}

#[test]
fn get_firmware_revision_encodes() {
    assert_eq!(GetFirmwareRevision.to_message().unwrap(), b"RV;".to_vec());
}

#[test]
fn get_serial_number_encodes() {
    assert_eq!(
        GetTransceiverSerialNumber.to_message().unwrap(),
        b"SN;".to_vec()
    );
}

#[test]
fn get_status_encodes() {
    assert_eq!(GetTransceiverStatus.to_message().unwrap(), b"ST;".to_vec());
}

#[test]
fn emulate_button_tap_encodes() {
    let cmd = EmulateButtonTap { button: 3 };
    assert_eq!(cmd.to_message().unwrap(), b"SW3T;".to_vec());
}

#[test]
fn emulate_button_tap_accepts_boundary_values() {
    assert!(EmulateButtonTap { button: 1 }.validate().is_ok());
    assert!(EmulateButtonTap { button: 6 }.validate().is_ok());
}

#[test]
fn emulate_button_tap_rejects_out_of_range() {
    assert!(matches!(
        EmulateButtonTap { button: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        EmulateButtonTap { button: 7 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn emulate_button_hold_encodes() {
    let cmd = EmulateButtonHold { button: 3 };
    assert_eq!(cmd.to_message().unwrap(), b"SW3H;".to_vec());
}

#[test]
fn emulate_button_hold_accepts_boundary_values() {
    assert!(EmulateButtonHold { button: 1 }.validate().is_ok());
    assert!(EmulateButtonHold { button: 6 }.validate().is_ok());
}

#[test]
fn emulate_button_hold_rejects_out_of_range() {
    assert!(matches!(
        EmulateButtonHold { button: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        EmulateButtonHold { button: 7 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

#[test]
fn get_tx_low_limit_encodes() {
    assert_eq!(
        GetTransmitLowerLimit {
            band: TransmitBand::Band20m
        }
        .to_message()
        .unwrap(),
        b"TXL2;".to_vec()
    );
}

#[test]
fn get_tx_high_limit_encodes() {
    assert_eq!(
        GetTransmitUpperLimit {
            band: TransmitBand::Band17m
        }
        .to_message()
        .unwrap(),
        b"TXH3;".to_vec()
    );
}
