//! Encoding tests for `rfham_rigs::protocol::cat::elecraft::p3`.
//!
//! These check the bytes each command actually sends on the wire (`Command::to_message`), and,
//! where a command validates its arguments, that boundary/invalid values are rejected as
//! expected. Response *parsing* is intentionally out of scope here.

use pretty_assertions::assert_eq;
use rfham_rigs::{
    error::RigError,
    protocol::{
        Command, SignedFrequency,
        cat::elecraft::p3::{
            DisplayMode, ExecuteFunctionKey, FixedTuneAutoAdjustMode, FixedTuneOrTrackingMode,
            FontSize, GetBaudRate, GetCenterFrequency, GetDisplayAveragingTimeConstant,
            GetDisplayFontSize, GetDisplayMode, GetFirmwareRevision, GetFixedTuneAutoAdjustMode,
            GetFixedTuneOrTrackingMode, GetFpgaImageFirmwareRevision, GetFunctionKeyLabel,
            GetFunctionKeyLabelDisplayState, GetMarkerAFrequency, GetMarkerAState,
            GetMarkerBFrequency, GetMarkerBState, GetNoiseBlankerLevel, GetNoiseBlankerState,
            GetPeakModeState, GetPowerStatus, GetProductId, GetReferenceLevel,
            GetRelativeCenterFrequency, GetScale, GetSpan, GetSpanMode,
            GetSvgaDecodedDataDisplayState, GetSvgaDisplayResolution, GetSvgaDisplayState,
            GetSvgaFirmwareRevision, GetSvgaFontSize, GetSvgaSpectrumFillState,
            GetSvgaWaterfallBias, GetTransceiverConnected, GetVfoBCursorState,
            GetWaterfallAveragingState, GetWaterfallColor, GetWaterfallMarkersState, QsyAction,
            Reset, SetBaudRate, SetCenterFrequency, SetDisplayAveragingTimeConstant,
            SetDisplayFontSize, SetDisplayMode, SetFixedTuneAutoAdjustMode,
            SetFixedTuneOrTrackingMode, SetFunctionKeyLabelDisplayState, SetMarkerAFrequency,
            SetMarkerAState, SetMarkerBFrequency, SetMarkerBState, SetNoiseBlankerLevel,
            SetNoiseBlankerState, SetPassThroughModeState, SetPeakModeState, SetPowerStatus,
            SetQsyToMarker, SetReferenceLevel, SetRelativeCenterFrequency, SetScale, SetSpan,
            SetSpanMode, SetSvgaDecodedDataDisplayState, SetSvgaDisplayResolution,
            SetSvgaDisplayState, SetSvgaFontSize, SetSvgaSpectrumFillState, SetSvgaWaterfallBias,
            SetTransceiverConnected, SetVfoBCursorState, SetWaterfallAveragingState,
            SetWaterfallColor, SetWaterfallMarkersState, SpanMode, SvgaDisplayResolution,
            SvgaFontSize, UploadScreenshotBitmap, WaterfallColor,
        },
    },
    transport::BaudRate,
};

// ------------------------------------------------------------------------------------------------
// GetProductId (hand-rolled: no `#` prefix, no `;` terminator)
// ------------------------------------------------------------------------------------------------

#[test]
fn get_product_id_encodes() {
    // Unlike every other P3 command, this has no `#` prefix and no `;` terminator.
    assert_eq!(GetProductId.to_message().unwrap(), b"=".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetDisplayAveragingTimeConstant / SetDisplayAveragingTimeConstant
// ------------------------------------------------------------------------------------------------

#[test]
fn get_display_averaging_time_constant_encodes() {
    assert_eq!(
        GetDisplayAveragingTimeConstant.to_message().unwrap(),
        b"#AVG;".to_vec()
    );
}

#[test]
fn set_display_averaging_time_constant_encodes_off() {
    let cmd = SetDisplayAveragingTimeConstant { averaging_time: 0 };
    assert_eq!(cmd.to_message().unwrap(), b"#AVG00;".to_vec());
}

#[test]
fn set_display_averaging_time_constant_encodes_on() {
    let cmd = SetDisplayAveragingTimeConstant { averaging_time: 20 };
    assert_eq!(cmd.to_message().unwrap(), b"#AVG20;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// BitmapUpload (Command encoding is standard; only response parsing is hand-rolled)
// ------------------------------------------------------------------------------------------------

#[test]
fn bitmap_upload_encodes() {
    // NOTE: unlike GetProductId, BitmapUpload's *request* encoding uses the standard
    // `impl_command!` macro (only its response parsing is hand-rolled to skip the command-id
    // echo/terminator), so this ends in `;` like any other P3 command.
    assert_eq!(
        UploadScreenshotBitmap.to_message().unwrap(),
        b"#BMP;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetBaudRate / SetBaudRate
// ------------------------------------------------------------------------------------------------

#[test]
fn get_baud_rate_encodes() {
    assert_eq!(GetBaudRate.to_message().unwrap(), b"#BR;".to_vec());
}

#[test]
fn set_baud_rate_encodes_4800() {
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd4800,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#BR0;".to_vec());
}

#[test]
fn set_baud_rate_encodes_9600() {
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd9600,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#BR1;".to_vec());
}

#[test]
fn set_baud_rate_encodes_19200() {
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd19200,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#BR2;".to_vec());
}

#[test]
fn set_baud_rate_encodes_38400() {
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd38400,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#BR3;".to_vec());
}

#[test]
fn set_baud_rate_rejects_unsupported_rate() {
    // BaudRate has variants outside the 4 the P3 supports; `argument_bytes` (called from
    // `to_message`) returns an error for any of them rather than clamping or panicking.
    let cmd = SetBaudRate {
        baud_rate: BaudRate::Bd300,
    };
    assert!(matches!(
        cmd.to_message(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetCenterFrequency / SetCenterFrequency
// ------------------------------------------------------------------------------------------------

#[test]
fn get_center_frequency_encodes() {
    assert_eq!(GetCenterFrequency.to_message().unwrap(), b"#CTF;".to_vec());
}

#[test]
fn set_center_frequency_encodes_positive() {
    let cmd = SetCenterFrequency {
        center: SignedFrequency::from(14_060_000),
    };
    assert_eq!(cmd.to_message().unwrap(), b"#CTF+00014060000;".to_vec());
}

#[test]
fn set_center_frequency_encodes_negative() {
    let cmd = SetCenterFrequency {
        center: SignedFrequency::from(-500),
    };
    assert_eq!(cmd.to_message().unwrap(), b"#CTF-00000000500;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetDisplayMode / SetDisplayMode / DisplayMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_display_mode_encodes() {
    assert_eq!(GetDisplayMode.to_message().unwrap(), b"#DSM;".to_vec());
}

#[test]
fn set_display_mode_encodes_spectrum_only() {
    let cmd = SetDisplayMode {
        mode: DisplayMode::SpectrumOnly,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#DSM0;".to_vec());
}

#[test]
fn set_display_mode_encodes_spectrum_and_waterfall() {
    let cmd = SetDisplayMode {
        mode: DisplayMode::SpectrumAndWaterfall,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#DSM1;".to_vec());
}

#[test]
fn set_display_mode_encodes_spectrum_and_power_meters() {
    let cmd = SetDisplayMode {
        mode: DisplayMode::SpectrumAndPowerMeters,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#DSM2;".to_vec());
}

#[test]
fn set_display_mode_encodes_spectrum_and_waterfall_and_power_meters() {
    let cmd = SetDisplayMode {
        mode: DisplayMode::SpectrumAndWaterfallAndPowerMeters,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#DSM3;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetFunctionKeyLabel
// ------------------------------------------------------------------------------------------------

#[test]
fn get_function_key_label_encodes_key_one() {
    let cmd = GetFunctionKeyLabel { function_key: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"#FNL1;".to_vec());
}

#[test]
fn get_function_key_label_encodes_key_eight() {
    let cmd = GetFunctionKeyLabel { function_key: 8 };
    assert_eq!(cmd.to_message().unwrap(), b"#FNL8;".to_vec());
}

#[test]
fn get_function_key_label_rejects_out_of_range_key() {
    // The range check happens inline in `argument_bytes` rather than via a separate `validate`
    // override, so it surfaces through `to_message` directly.
    let cmd = GetFunctionKeyLabel { function_key: 9 };
    assert!(matches!(
        cmd.to_message(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    let cmd = GetFunctionKeyLabel { function_key: 0 };
    assert!(matches!(
        cmd.to_message(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetFontSize / SetFontSize / FontSize
// ------------------------------------------------------------------------------------------------

#[test]
fn get_font_size_encodes() {
    assert_eq!(GetDisplayFontSize.to_message().unwrap(), b"#FON;".to_vec());
}

#[test]
fn set_font_size_encodes_small() {
    let cmd = SetDisplayFontSize {
        size: FontSize::Small,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FON0;".to_vec());
}

#[test]
fn set_font_size_encodes_medium() {
    let cmd = SetDisplayFontSize {
        size: FontSize::Medium,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FON1;".to_vec());
}

#[test]
fn set_font_size_encodes_large() {
    let cmd = SetDisplayFontSize {
        size: FontSize::Large,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FON2;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// SetFunctionKeyExecute
// ------------------------------------------------------------------------------------------------

#[test]
fn set_function_key_execute_encodes_key_one() {
    let cmd = ExecuteFunctionKey { function_key: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"#FNX1;".to_vec());
}

#[test]
fn set_function_key_execute_encodes_key_eight() {
    let cmd = ExecuteFunctionKey { function_key: 8 };
    assert_eq!(cmd.to_message().unwrap(), b"#FNX8;".to_vec());
}

#[test]
fn set_function_key_execute_rejects_out_of_range_key() {
    let cmd = ExecuteFunctionKey { function_key: 9 };
    assert!(matches!(
        cmd.to_message(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetFixedTuneAutoAdjustMode / SetFixedTuneAutoAdjustMode / FixedTuneAutoAdjustMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_fixed_tune_auto_adjust_mode_encodes() {
    assert_eq!(
        GetFixedTuneAutoAdjustMode.to_message().unwrap(),
        b"#FXA;".to_vec()
    );
}

#[test]
fn set_fixed_tune_auto_adjust_mode_encodes_full_screen() {
    let cmd = SetFixedTuneAutoAdjustMode {
        mode: FixedTuneAutoAdjustMode::FullScreen,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FXA0;".to_vec());
}

#[test]
fn set_fixed_tune_auto_adjust_mode_encodes_half_screen() {
    let cmd = SetFixedTuneAutoAdjustMode {
        mode: FixedTuneAutoAdjustMode::HalfScreen,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FXA1;".to_vec());
}

#[test]
fn set_fixed_tune_auto_adjust_mode_encodes_slide() {
    let cmd = SetFixedTuneAutoAdjustMode {
        mode: FixedTuneAutoAdjustMode::Slide,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FXA2;".to_vec());
}

#[test]
fn set_fixed_tune_auto_adjust_mode_encodes_static() {
    let cmd = SetFixedTuneAutoAdjustMode {
        mode: FixedTuneAutoAdjustMode::Static,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FXA3;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetFixedTuneOrTrackingMode / SetFixedTuneOrTrackingMode / FixedTuneOrTrackingMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_fixed_tune_or_tracking_mode_encodes() {
    assert_eq!(
        GetFixedTuneOrTrackingMode.to_message().unwrap(),
        b"#FXT;".to_vec()
    );
}

#[test]
fn set_fixed_tune_or_tracking_mode_encodes_tracking() {
    let cmd = SetFixedTuneOrTrackingMode {
        mode: FixedTuneOrTrackingMode::Tracking,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FXT0;".to_vec());
}

#[test]
fn set_fixed_tune_or_tracking_mode_encodes_fixed_tune() {
    let cmd = SetFixedTuneOrTrackingMode {
        mode: FixedTuneOrTrackingMode::FixedTune,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#FXT1;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetFnLabelDisplay / SetFnLabelDisplay
// ------------------------------------------------------------------------------------------------

#[test]
fn get_fn_label_display_encodes() {
    assert_eq!(
        GetFunctionKeyLabelDisplayState.to_message().unwrap(),
        b"#LBL;".to_vec()
    );
}

#[test]
fn set_fn_label_display_encodes_on() {
    let cmd = SetFunctionKeyLabelDisplayState::turn_on();
    assert_eq!(cmd.to_message().unwrap(), b"#LBL1;".to_vec());
}

#[test]
fn set_fn_label_display_encodes_off() {
    let cmd = SetFunctionKeyLabelDisplayState::turn_off();
    assert_eq!(cmd.to_message().unwrap(), b"#LBL0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetMarkerAFrequency / SetMarkerAFrequency
// ------------------------------------------------------------------------------------------------

#[test]
fn get_marker_a_frequency_encodes() {
    assert_eq!(GetMarkerAFrequency.to_message().unwrap(), b"#MFA;".to_vec());
}

#[test]
fn set_marker_a_frequency_encodes_positive() {
    let cmd = SetMarkerAFrequency {
        marker: SignedFrequency::from(14_060_000),
    };
    assert_eq!(cmd.to_message().unwrap(), b"#MFA+00014060000;".to_vec());
}

#[test]
fn set_marker_a_frequency_encodes_negative() {
    let cmd = SetMarkerAFrequency {
        marker: SignedFrequency::from(-500),
    };
    assert_eq!(cmd.to_message().unwrap(), b"#MFA-00000000500;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetMarkerBFrequency / SetMarkerBFrequency
// ------------------------------------------------------------------------------------------------

#[test]
fn get_marker_b_frequency_encodes() {
    assert_eq!(GetMarkerBFrequency.to_message().unwrap(), b"#MFB;".to_vec());
}

#[test]
fn set_marker_b_frequency_encodes_positive() {
    let cmd = SetMarkerBFrequency {
        marker: SignedFrequency::from(7_074_000),
    };
    assert_eq!(cmd.to_message().unwrap(), b"#MFB+00007074000;".to_vec());
}

#[test]
fn set_marker_b_frequency_encodes_negative() {
    let cmd = SetMarkerBFrequency {
        marker: SignedFrequency::from(-1_000),
    };
    assert_eq!(cmd.to_message().unwrap(), b"#MFB-00000001000;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetMarkerAState / SetMarkerAState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_marker_a_state_encodes() {
    assert_eq!(GetMarkerAState.to_message().unwrap(), b"#MKA;".to_vec());
}

#[test]
fn set_marker_a_state_encodes_on() {
    let cmd = SetMarkerAState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#MKA1;".to_vec());
}

#[test]
fn set_marker_a_state_encodes_off() {
    let cmd = SetMarkerAState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#MKA0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetMarkerBState / SetMarkerBState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_marker_b_state_encodes() {
    assert_eq!(GetMarkerBState.to_message().unwrap(), b"#MKB;".to_vec());
}

#[test]
fn set_marker_b_state_encodes_on() {
    let cmd = SetMarkerBState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#MKB1;".to_vec());
}

#[test]
fn set_marker_b_state_encodes_off() {
    let cmd = SetMarkerBState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#MKB0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetNoiseBlankerState / SetNoiseBlankerState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_noise_blanker_state_encodes() {
    assert_eq!(GetNoiseBlankerState.to_message().unwrap(), b"#NB;".to_vec());
}

#[test]
fn set_noise_blanker_state_encodes_on() {
    let cmd = SetNoiseBlankerState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#NB1;".to_vec());
}

#[test]
fn set_noise_blanker_state_encodes_off() {
    let cmd = SetNoiseBlankerState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#NB0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetNoiseBlankerLevel / SetNoiseBlankerLevel
//
// NOTE: SetNoiseBlankerLevel had a backwards silent-clamp bug fixed into proper validation during
// the macro conversion, so this gets careful boundary testing on `validate()`.
// ------------------------------------------------------------------------------------------------

#[test]
fn get_noise_blanker_level_encodes() {
    assert_eq!(
        GetNoiseBlankerLevel.to_message().unwrap(),
        b"#NBL;".to_vec()
    );
}

#[test]
fn set_noise_blanker_level_encodes_min() {
    let cmd = SetNoiseBlankerLevel { level: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"#NBL01;".to_vec());
}

#[test]
fn set_noise_blanker_level_encodes_max() {
    let cmd = SetNoiseBlankerLevel { level: 15 };
    assert_eq!(cmd.to_message().unwrap(), b"#NBL15;".to_vec());
}

#[test]
fn set_noise_blanker_level_accepts_boundary_values() {
    assert!(SetNoiseBlankerLevel { level: 1 }.validate().is_ok());
    assert!(SetNoiseBlankerLevel { level: 15 }.validate().is_ok());
}

#[test]
fn set_noise_blanker_level_rejects_out_of_range() {
    assert!(matches!(
        SetNoiseBlankerLevel { level: 0 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetNoiseBlankerLevel { level: 16 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// GetPeakModeState / SetPeakModeState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_peak_mode_state_encodes() {
    assert_eq!(GetPeakModeState.to_message().unwrap(), b"#PKM;".to_vec());
}

#[test]
fn set_peak_mode_state_encodes_on() {
    let cmd = SetPeakModeState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#PKM1;".to_vec());
}

#[test]
fn set_peak_mode_state_encodes_off() {
    let cmd = SetPeakModeState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#PKM0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetPowerStatus / SetPowerStatus
// ------------------------------------------------------------------------------------------------

#[test]
fn get_power_status_encodes() {
    assert_eq!(GetPowerStatus.to_message().unwrap(), b"#PS;".to_vec());
}

#[test]
fn set_power_status_encodes_on() {
    let cmd = SetPowerStatus { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#PS1;".to_vec());
}

#[test]
fn set_power_status_encodes_off() {
    let cmd = SetPowerStatus { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#PS0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// SetPassThroughModeState
// ------------------------------------------------------------------------------------------------

#[test]
fn set_pass_through_mode_state_encodes() {
    assert_eq!(
        SetPassThroughModeState.to_message().unwrap(),
        b"#PT;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// SetQsyToMarker / QsyAction
// ------------------------------------------------------------------------------------------------

#[test]
fn set_qsy_to_marker_encodes_qsy() {
    let cmd = SetQsyToMarker {
        action: QsyAction::Qsy,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#QSY1;".to_vec());
}

#[test]
fn set_qsy_to_marker_encodes_undo_qsy() {
    let cmd = SetQsyToMarker {
        action: QsyAction::UndoQsy,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#QSY0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetRelativeCenterFrequency / SetRelativeCenterFrequency
//
// NOTE: `#RCF` uses a bespoke 6-digit signed-offset encoding (a plain decimal offset in Hz,
// unlike the 11-digit `Frequency`-based encoding used by `#CTF`/`#MFA`/`#MFB`). This was flagged
// during the macro conversion as unverified against an external programmer's-reference copy, so
// these tests document current code behavior rather than confirmed-correct wire format.
// ------------------------------------------------------------------------------------------------

#[test]
fn get_relative_center_frequency_encodes() {
    assert_eq!(
        GetRelativeCenterFrequency.to_message().unwrap(),
        b"#RCF;".to_vec()
    );
}

#[test]
fn set_relative_center_frequency_encodes_positive() {
    // Matches the doc's own worked example: `#RCF+025000;`.
    let cmd = SetRelativeCenterFrequency {
        offset: SignedFrequency::from(25_000),
    };
    assert_eq!(cmd.to_message().unwrap(), b"#RCF+025000;".to_vec());
}

#[test]
fn set_relative_center_frequency_encodes_negative() {
    let cmd = SetRelativeCenterFrequency {
        offset: SignedFrequency::from(-1_000),
    };
    assert_eq!(cmd.to_message().unwrap(), b"#RCF-001000;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetReferenceLevel / SetReferenceLevel
//
// NOTE: SetReferenceLevel had a backwards silent-clamp bug fixed into proper validation during
// the macro conversion, so this gets careful boundary testing on `validate()`.
// ------------------------------------------------------------------------------------------------

#[test]
fn get_reference_level_encodes() {
    assert_eq!(GetReferenceLevel.to_message().unwrap(), b"#REF;".to_vec());
}

#[test]
fn set_reference_level_encodes_example_from_doc() {
    // Matches the doc's own worked example: `#REF-120;`.
    let cmd = SetReferenceLevel { dbm: -120 };
    assert_eq!(cmd.to_message().unwrap(), b"#REF-120;".to_vec());
}

#[test]
fn set_reference_level_encodes_min_boundary() {
    let cmd = SetReferenceLevel { dbm: -170 };
    assert_eq!(cmd.to_message().unwrap(), b"#REF-170;".to_vec());
}

#[test]
fn set_reference_level_encodes_max_boundary() {
    let cmd = SetReferenceLevel { dbm: 10 };
    assert_eq!(cmd.to_message().unwrap(), b"#REF+010;".to_vec());
}

#[test]
fn set_reference_level_accepts_boundary_values() {
    assert!(SetReferenceLevel { dbm: -170 }.validate().is_ok());
    assert!(SetReferenceLevel { dbm: 10 }.validate().is_ok());
}

#[test]
fn set_reference_level_rejects_out_of_range() {
    assert!(matches!(
        SetReferenceLevel { dbm: -171 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
    assert!(matches!(
        SetReferenceLevel { dbm: 11 }.validate(),
        Err(RigError::InvalidArgumentValue { .. })
    ));
}

// ------------------------------------------------------------------------------------------------
// Reset
// ------------------------------------------------------------------------------------------------

#[test]
fn reset_encodes() {
    assert_eq!(Reset.to_message().unwrap(), b"#RST;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetFpgaImageFirmwareRevision
// ------------------------------------------------------------------------------------------------

#[test]
fn get_fpga_image_firmware_revision_encodes() {
    assert_eq!(
        GetFpgaImageFirmwareRevision.to_message().unwrap(),
        b"#RVF;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetMainFirmware
// ------------------------------------------------------------------------------------------------

#[test]
fn get_main_firmware_encodes() {
    assert_eq!(GetFirmwareRevision.to_message().unwrap(), b"#RVM;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSvgaFirmwareRevision
// ------------------------------------------------------------------------------------------------

#[test]
fn get_svga_firmware_revision_encodes() {
    assert_eq!(
        GetSvgaFirmwareRevision.to_message().unwrap(),
        b"#RVS;".to_vec()
    );
}

// ------------------------------------------------------------------------------------------------
// GetScale / SetScale
// ------------------------------------------------------------------------------------------------

#[test]
fn get_scale_encodes() {
    assert_eq!(GetScale.to_message().unwrap(), b"#SCL;".to_vec());
}

#[test]
fn set_scale_encodes_min() {
    let cmd = SetScale { db: 10 };
    assert_eq!(cmd.to_message().unwrap(), b"#SCL010;".to_vec());
}

#[test]
fn set_scale_encodes_max() {
    // Matches the doc's own worked example: `#SCL080;`.
    let cmd = SetScale { db: 80 };
    assert_eq!(cmd.to_message().unwrap(), b"#SCL080;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSpanMode / SetSpanMode / SpanMode
// ------------------------------------------------------------------------------------------------

#[test]
fn get_span_mode_encodes() {
    assert_eq!(GetSpanMode.to_message().unwrap(), b"#SPM;".to_vec());
}

#[test]
fn set_span_mode_encodes_continuous() {
    let cmd = SetSpanMode {
        mode: SpanMode::Continuous,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SPM0;".to_vec());
}

#[test]
fn set_span_mode_encodes_stepped() {
    let cmd = SetSpanMode {
        mode: SpanMode::Stepped,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SPM1;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSpan / SetSpan
//
// NOTE: SetSpan also implements `CommandWithResponse` (its own response is parsed), but per this
// task's scope only the `Command`/encoding half is tested here.
// ------------------------------------------------------------------------------------------------

#[test]
fn get_span_encodes() {
    assert_eq!(GetSpan.to_message().unwrap(), b"#SPN;".to_vec());
}

#[test]
fn set_span_encodes() {
    // Matches the doc's own worked example: `#SPN000500;` (50 kHz in 100 Hz units).
    let cmd = SetSpan {
        span_hundred_hz: 500,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SPN000500;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSvgaDecodedDataDisplayState / SetSvgaDecodedDataDisplayState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_svga_decoded_data_display_state_encodes() {
    assert_eq!(
        GetSvgaDecodedDataDisplayState.to_message().unwrap(),
        b"#SVDT;".to_vec()
    );
}

#[test]
fn set_svga_decoded_data_display_state_encodes_on() {
    let cmd = SetSvgaDecodedDataDisplayState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#SVDT1;".to_vec());
}

#[test]
fn set_svga_decoded_data_display_state_encodes_off() {
    let cmd = SetSvgaDecodedDataDisplayState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#SVDT0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSvgaDisplayState / SetSvgaDisplayState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_svga_display_state_encodes() {
    assert_eq!(
        GetSvgaDisplayState.to_message().unwrap(),
        b"#SVEN;".to_vec()
    );
}

#[test]
fn set_svga_display_state_encodes_on() {
    let cmd = SetSvgaDisplayState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#SVEN1;".to_vec());
}

#[test]
fn set_svga_display_state_encodes_off() {
    let cmd = SetSvgaDisplayState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#SVEN0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSvgaSpectrumFillState / SetSvgaSpectrumFillState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_svga_spectrum_fill_state_encodes() {
    assert_eq!(
        GetSvgaSpectrumFillState.to_message().unwrap(),
        b"#SVFL;".to_vec()
    );
}

#[test]
fn set_svga_spectrum_fill_state_encodes_on() {
    let cmd = SetSvgaSpectrumFillState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#SVFL1;".to_vec());
}

#[test]
fn set_svga_spectrum_fill_state_encodes_off() {
    let cmd = SetSvgaSpectrumFillState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#SVFL0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSvgaFontSize / SetSvgaFontSize / SvgaFontSize
// ------------------------------------------------------------------------------------------------

#[test]
fn get_svga_font_size_encodes() {
    assert_eq!(GetSvgaFontSize.to_message().unwrap(), b"#SVFN;".to_vec());
}

#[test]
fn set_svga_font_size_encodes_small() {
    let cmd = SetSvgaFontSize {
        size: SvgaFontSize::Small,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SVFN0;".to_vec());
}

#[test]
fn set_svga_font_size_encodes_medium() {
    let cmd = SetSvgaFontSize {
        size: SvgaFontSize::Medium,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SVFN1;".to_vec());
}

#[test]
fn set_svga_font_size_encodes_large() {
    let cmd = SetSvgaFontSize {
        size: SvgaFontSize::Large,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SVFN2;".to_vec());
}

#[test]
fn set_svga_font_size_encodes_larger() {
    let cmd = SetSvgaFontSize {
        size: SvgaFontSize::Larger,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SVFN3;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSvgaDisplayResolution / SetSvgaDisplayResolution / SvgaDisplayResolution
// ------------------------------------------------------------------------------------------------

#[test]
fn get_svga_display_resolution_encodes() {
    assert_eq!(
        GetSvgaDisplayResolution.to_message().unwrap(),
        b"#SVRS;".to_vec()
    );
}

#[test]
fn set_svga_display_resolution_encodes_xga() {
    let cmd = SetSvgaDisplayResolution {
        resolution: SvgaDisplayResolution::Xga,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SVRS0;".to_vec());
}

#[test]
fn set_svga_display_resolution_encodes_wxga_plus() {
    let cmd = SetSvgaDisplayResolution {
        resolution: SvgaDisplayResolution::WxgaPlus,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SVRS2;".to_vec());
}

#[test]
fn set_svga_display_resolution_encodes_fhd_alt() {
    let cmd = SetSvgaDisplayResolution {
        resolution: SvgaDisplayResolution::FHdAlt,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#SVRS4;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetSvgaWaterfallBias / SetSvgaWaterfallBias
// ------------------------------------------------------------------------------------------------

#[test]
fn get_svga_waterfall_bias_encodes() {
    assert_eq!(
        GetSvgaWaterfallBias.to_message().unwrap(),
        b"#SVWB;".to_vec()
    );
}

#[test]
fn set_svga_waterfall_bias_encodes_min() {
    let cmd = SetSvgaWaterfallBias { bias: 1 };
    assert_eq!(cmd.to_message().unwrap(), b"#SVWB01;".to_vec());
}

#[test]
fn set_svga_waterfall_bias_encodes_max() {
    let cmd = SetSvgaWaterfallBias { bias: 99 };
    assert_eq!(cmd.to_message().unwrap(), b"#SVWB99;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetVfoBCursorState / SetVfoBCursorState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_vfo_b_cursor_state_encodes() {
    assert_eq!(GetVfoBCursorState.to_message().unwrap(), b"#VFB;".to_vec());
}

#[test]
fn set_vfo_b_cursor_state_encodes_on() {
    let cmd = SetVfoBCursorState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#VFB1;".to_vec());
}

#[test]
fn set_vfo_b_cursor_state_encodes_off() {
    let cmd = SetVfoBCursorState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#VFB0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetWaterfallAveragingState / SetWaterfallAveragingState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_waterfall_averaging_state_encodes() {
    assert_eq!(
        GetWaterfallAveragingState.to_message().unwrap(),
        b"#WFA;".to_vec()
    );
}

#[test]
fn set_waterfall_averaging_state_encodes_on() {
    let cmd = SetWaterfallAveragingState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#WFA1;".to_vec());
}

#[test]
fn set_waterfall_averaging_state_encodes_off() {
    let cmd = SetWaterfallAveragingState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#WFA0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetWaterfallColor / SetWaterfallColor / WaterfallColor
// ------------------------------------------------------------------------------------------------

#[test]
fn get_waterfall_color_encodes() {
    assert_eq!(GetWaterfallColor.to_message().unwrap(), b"#WFC;".to_vec());
}

#[test]
fn set_waterfall_color_encodes_gray_scale() {
    let cmd = SetWaterfallColor {
        color: WaterfallColor::GrayScale,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#WFC0;".to_vec());
}

#[test]
fn set_waterfall_color_encodes_colored() {
    let cmd = SetWaterfallColor {
        color: WaterfallColor::Colored,
    };
    assert_eq!(cmd.to_message().unwrap(), b"#WFC1;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetWaterfallMarkersState / SetWaterfallMarkersState
// ------------------------------------------------------------------------------------------------

#[test]
fn get_waterfall_markers_state_encodes() {
    assert_eq!(
        GetWaterfallMarkersState.to_message().unwrap(),
        b"#WFM;".to_vec()
    );
}

#[test]
fn set_waterfall_markers_state_encodes_on() {
    let cmd = SetWaterfallMarkersState { on: true };
    assert_eq!(cmd.to_message().unwrap(), b"#WFM1;".to_vec());
}

#[test]
fn set_waterfall_markers_state_encodes_off() {
    let cmd = SetWaterfallMarkersState { on: false };
    assert_eq!(cmd.to_message().unwrap(), b"#WFM0;".to_vec());
}

// ------------------------------------------------------------------------------------------------
// GetTransceiverConnected / SetTransceiverConnected
// ------------------------------------------------------------------------------------------------

#[test]
fn get_transceiver_connected_encodes() {
    assert_eq!(
        GetTransceiverConnected.to_message().unwrap(),
        b"#XCV;".to_vec()
    );
}

#[test]
fn set_transceiver_connected_encodes_k3() {
    let cmd = SetTransceiverConnected { transceiver: 0 };
    assert_eq!(cmd.to_message().unwrap(), b"#XCV00;".to_vec());
}

#[test]
fn set_transceiver_connected_encodes_other() {
    let cmd = SetTransceiverConnected { transceiver: 5 };
    assert_eq!(cmd.to_message().unwrap(), b"#XCV05;".to_vec());
}
