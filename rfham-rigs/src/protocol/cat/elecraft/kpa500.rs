//!
//! CAT commands for the Elecraft KPA500 solid-state HF amplifier.
//!
//! All KPA500 commands and responses use a leading caret (`^`), for example `^BN05;`. The GET
//! form of a command is the bare command letters with no data, e.g. `^BN;`.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//!
//! 1. [Elecraft KPA500 Programmer's Reference, rev A2](https://ftp.elecraft.com/KPA/Manuals%20Downloads/KPA500%20Programmers%20Ref.pdf)., Jul 2011
//!

use crate::{
    error::RigError,
    protocol::cat::common::{
        bytes_to_vec, format_uint_ascii, u8_from_ascii, u16_from_ascii, u32_from_ascii,
    },
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetAlcThreshold, SetAlcThreshold
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the ALC threshold for the current band.

The ALC value is saved per band; the response reflects the currently selected band.

# Command format

> `^AL;`

# Response format

> `^AL{nnn};`

Where *nnn* is the ALC threshold, between `000` and `210`." =>
    GetAlcThreshold
);

define_cat_command!("Set the ALC threshold for the current band.

# Command format

> `^AL{nnn};`

Where *nnn* is the ALC threshold, between `000` and `210`." =>
    SetAlcThreshold {
        value: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetAttenuatorReleaseTime, SetAttenuatorReleaseTime
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the attenuator fault release time.

# Command format

> `^AR;`

# Response format

> `^AR{nnnn};`

Where *nnnn* is the attenuator fault release time, in milliseconds, between `1400` and `5000`." =>
    GetAttenuatorReleaseTime
);

define_cat_command!("Set the attenuator fault release time.

# Command format

> `^AR{nnnn};`

Where *nnnn* is the attenuator fault release time, in milliseconds, between `1400` and `5000`." =>
    SetAttenuatorReleaseTime {
        ms: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetStandbyOnBandChange, SetStandbyOnBandChange
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether the amplifier always returns to standby on a band change.

# Command format

> `^BC;`

# Response format

> `^BC{n};`

Where `n` is the boolean state `0` (return to the prior operate/standby state) or `1` (always go
to standby after a band change)." =>
    GetStandbyOnBandChange
);

define_cat_command!("Set whether the amplifier always returns to standby on a band change.

# Command format

> `^BC{n};`

Where `n` is the boolean state `0` (return to the prior operate/standby state) or `1` (always go
to standby after a band change)." =>
    SetStandbyOnBandChange {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandSelection, SetBandSelection
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the currently selected band.

# Command format

> `^BN;`

# Response format

> `^BN{nn};`

Where *nn* is the band code, one of:

* `00`; 160 m.
* `01`; 80 m.
* `02`; 60 m.
* `03`; 40 m.
* `04`; 30 m.
* `05`; 20 m.
* `06`; 17 m.
* `07`; 15 m.
* `08`; 12 m.
* `09`; 10 m.
* `10`; 6 m." =>
    GetBandSelection
);

define_cat_command!("Set the currently selected band.

# Command format

> `^BN{nn};`

Where *nn* is the band code, one of:

* `00`; 160 m.
* `01`; 80 m.
* `02`; 60 m.
* `03`; 40 m.
* `04`; 30 m.
* `05`; 20 m.
* `06`; 17 m.
* `07`; 15 m.
* `08`; 12 m.
* `09`; 10 m.
* `10`; 6 m." =>
    SetBandSelection {
        band: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPcBaudRate, SetPcBaudRate
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PC RS-232 port data rate.

# Command format

> `^BRP;`

# Response format

> `^BRP{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    GetPcBaudRate
);

define_cat_command!("Set the PC RS-232 port data rate.

# Command format

> `^BRP{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    SetPcBaudRate {
        rate: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetXcvrBaudRate, SetXcvrBaudRate
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the transceiver RS-232 port data rate.

# Command format

> `^BRX;`

# Response format

> `^BRX{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    GetXcvrBaudRate
);

define_cat_command!("Set the transceiver RS-232 port data rate.

# Command format

> `^BRX{n};`

Where *n* is one of:

* `0`; 4800 baud.
* `1`; 9600 baud.
* `2`; 19200 baud.
* `3`; 38400 baud." =>
    SetXcvrBaudRate {
        rate: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetDemoMode, SetDemoMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the demo mode state.

# Command format

> `^DMO;`

# Response format

> `^DMO{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetDemoMode
);

define_cat_command!("Set the demo mode state.

# Command format

> `^DMO{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetDemoMode {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFanMinimumSpeed, SetFanMinimumSpeed
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the fan minimum control level.

# Command format

> `^FC;`

# Response format

> `^FC{n};`

Where *n* is the fan minimum speed, between `0` (off) and `6` (high)." =>
    GetFanMinimumSpeed
);

define_cat_command!("Set the fan minimum control level.

# Command format

> `^FC{n};`

Where *n* is the fan minimum speed, between `0` (off) and `6` (high)." =>
    SetFanMinimumSpeed {
        level: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultCode, ClearFault
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current fault code.

# Command format

> `^FL;`

# Response format

> `^FL{nn};`

Where *nn* is the 2-digit fault identifier; `00` indicates no active fault." =>
    GetFaultCode
);

define_cat_command!("Clear the current fault condition.

There is no query form; use [`GetFaultCode`] to check the fault state.

# Command format

> `^FLC;`" =>
    ClearFault
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetInhibitInput, SetInhibitInput
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether the INHIBIT# input pin is enabled.

# Command format

> `^NH;`

# Response format

> `^NH{n};`

Where `n` is the boolean state `0` (disabled) or `1` (enabled)." =>
    GetInhibitInput
);

define_cat_command!("Set whether the INHIBIT# input pin is enabled.

# Command format

> `^NH{n};`

Where `n` is the boolean state `0` (disabled) or `1` (enabled)." =>
    SetInhibitInput {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerStatus, TurnPowerOff
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get whether the KPA500 is powered on.

**Protocol quirk:** if the amplifier is currently powered off, it does not send any response to
this command at all; it does not reply with an explicit `^ON0;` off status, it simply stays
silent. Callers should treat a read timeout, or an empty response, to this command as equivalent
to the amplifier being powered off. The response parser here only handles the case where a
genuine 1-byte reply is actually received, and does not itself special-case a missing response.

# Command format

> `^ON;`

# Response format

> `^ON{n};`

Where `n` is the boolean state `1` (on). No response is sent when the amplifier is off." =>
    GetPowerStatus
);

define_cat_command!("Turn the KPA500 off.

Powering the amplifier back on cannot be done with a CAT command; that requires the KPA500's
boot-loader `P` command sequence instead.

# Command format

> `^ON0;`" =>
    TurnPowerOff
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOperateMode, SetOperateMode
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the current operate/standby mode.

# Command format

> `^OS;`

# Response format

> `^OS{n};`

Where `n` is the boolean state `0` (standby) or `1` (operate)." =>
    GetOperateMode
);

define_cat_command!("Set the current operate/standby mode.

# Command format

> `^OS{n};`

Where `n` is the boolean state `0` (standby) or `1` (operate)." =>
    SetOperateMode {
        operate: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerAdjustment, SetPowerAdjustment
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the power adjustment setting for the current band.

The value is saved per band; the response reflects the currently selected band.

# Command format

> `^PJ;`

# Response format

> `^PJ{nnn};`

Where *nnn* is the power adjustment, as a percentage of rated output, between `080` and `120`." =>
    GetPowerAdjustment
);

define_cat_command!("Set the power adjustment setting for the current band.

# Command format

> `^PJ{nnn};`

Where *nnn* is the power adjustment, as a percentage of rated output, between `080` and `120`." =>
    SetPowerAdjustment {
        value: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFirmwareVersion
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the firmware version string.

# Command format

> `^RVM;`

# Response format

> `^RVM{text};`

The response is a fixed-format `nn.nn` version string, e.g. `01.13`, returned as raw bytes." =>
    GetFirmwareVersion
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetSerialNumber
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the KPA500 serial number.

# Command format

> `^SN;`

# Response format

> `^SN{nnnnn};`

Where *nnnnn* is the 5-digit serial number." =>
    GetSerialNumber
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetFaultSpeaker, SetFaultSpeaker
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the fault speaker on/off state.

# Command format

> `^SP;`

# Response format

> `^SP{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetFaultSpeaker
);

define_cat_command!("Set the fault speaker on/off state.

# Command format

> `^SP{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetFaultSpeaker { state }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPaTemperature
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PA temperature.

# Command format

> `^TM;`

# Response format

> `^TM{nnn};`

Where *nnn* is the PA temperature, in degrees Celsius, between `000` and `150`." =>
    GetPaTemperature
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTrDelay, SetTrDelay
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the T/R (transmit-to-receive) delay time.

# Command format

> `^TR;`

# Response format

> `^TR{nn};`

Where *nn* is the T/R delay, in milliseconds, between `00` and `50`." =>
    GetTrDelay
);

define_cat_command!("Set the T/R (transmit-to-receive) delay time.

# Command format

> `^TR{nn};`

Where *nn* is the T/R delay, in milliseconds, between `00` and `50`." =>
    SetTrDelay {
        ms: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPaVoltageCurrent, PaVoltageCurrent
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the PA voltage and current.

# Command format

> `^VI;`

# Response format

> `^VI{vvv} {iii};`

Where *vvv* is the PA voltage, in tenths of a volt, and *iii* is the PA current, in tenths of an
ampere; the two fields are separated by a single space." =>
    GetPaVoltageCurrent
);

define_command_struct!(
    "PA voltage and current reading returned by [`GetPaVoltageCurrent`]." =>
    PaVoltageCurrent {
        "PA voltage, in tenths of a volt (e.g. `485` = 48.5 V)." => voltage_dv: u16,
        "PA current, in tenths of an ampere (e.g. `123` = 12.3 A)." => current_da: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetPowerAndSwr, PowerAndSwr
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the output power and SWR.

Returns `000 000` when the amplifier is not transmitting.

# Command format

> `^WS;`

# Response format

> `^WS{ppp} {sss};`

Where *ppp* is the output power, in watts, and *sss* is SWR × 10 (e.g. `15` represents an SWR of
1.5:1); the two fields are separated by a single space." =>
    GetPowerAndSwr
);

define_command_struct!(
    "Output power and SWR reading returned by [`GetPowerAndSwr`]." =>
    PowerAndSwr {
        "Output power, in watts (`000`\u{2013}`999`)." => power_w: u16,
        "SWR × 10 (e.g. `15` = 1.5:1). Zero when not transmitting." => swr_d: u16
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetRadioInterface, SetRadioInterface, RadioInterface
// ------------------------------------------------------------------------------------------------

define_cat_command!("Get the radio interface type.

# Command format

> `^XI;`

# Response format

> `^XI{nn}{o};`

Where *nn* is the interface type, one of:

* `00`; K3 (always returns *o* = `1` on firmware V1.04+).
* `01`; BCD.
* `02`; Analog (Icom voltage levels).
* `03`; Elecraft/Kenwood serial I/O.

and *o* is the option bit; for interface type `03` this enables radio frequency polling." =>
    GetRadioInterface
);

define_cat_command!("Set the radio interface type and option bit.

# Command format

> `^XI{nn}{o};`

Where *nn* is the interface type, one of:

* `00`; K3 (always returns *o* = `1` on firmware V1.04+).
* `01`; BCD.
* `02`; Analog (Icom voltage levels).
* `03`; Elecraft/Kenwood serial I/O.

and *o* is the option bit; for interface type `03` this enables radio frequency polling." =>
    SetRadioInterface {
        interface_type: u8,
        option: u8
    }
);

define_command_struct!(
    "The radio interface type and option bit returned by [`GetRadioInterface`]." =>
    RadioInterface {
        "The interface type: `0` = K3, `1` = BCD, `2` = Analog (Icom voltage levels), `3` = Elecraft/Kenwood serial I/O." =>
        interface_type: u8,
        "The option bit; meaning depends on `interface_type`, e.g. enables radio frequency polling for Elecraft/Kenwood serial I/O." =>
        option: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAlcThreshold => b"^AL");
impl_cat_command_with_response!(GetAlcThreshold => 3, u16_from_ascii => u16);

impl_cat_command!(SetAlcThreshold => b"^AL" with Some |cmd: &SetAlcThreshold| {
    format!("{:03}", cmd.value).into_bytes()
}, if |cmd: &SetAlcThreshold| {
    if cmd.value <= 210 {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "value",
            type_name: "u16",
            value: cmd.value.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetAttenuatorReleaseTime => b"^AR");
impl_cat_command_with_response!(GetAttenuatorReleaseTime => 4, u16_from_ascii => u16);

impl_cat_command!(SetAttenuatorReleaseTime => b"^AR" with Some |cmd: &SetAttenuatorReleaseTime| {
    format!("{:04}", cmd.ms).into_bytes()
}, if |cmd: &SetAttenuatorReleaseTime| {
    if (1400..=5000).contains(&cmd.ms) {
        Ok(())
    } else {
        Err(RigError::InvalidArgumentValue {
            argument_name: "ms",
            type_name: "u16",
            value: cmd.ms.to_string(),
        })
    }
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetStandbyOnBandChange => b"^BC");
impl_cat_command_with_response!(GetStandbyOnBandChange => boolean);

impl_cat_command!(SetStandbyOnBandChange => b"^BC" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetBandSelection => b"^BN");
impl_cat_command_with_response!(GetBandSelection => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetBandSelection => b"^BN"
    format band uint 2,
    if |cmd: &SetBandSelection| {
        if cmd.band <= 10 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "band",
                type_name: "u8",
                value: cmd.band.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPcBaudRate => b"^BRP");
impl_cat_command_with_response!(GetPcBaudRate => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetPcBaudRate => b"^BRP"
    format rate uint 1,
    if |cmd: &SetPcBaudRate| {
        if cmd.rate <= 3 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "rate",
                type_name: "u8",
                value: cmd.rate.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetXcvrBaudRate => b"^BRX");
impl_cat_command_with_response!(GetXcvrBaudRate => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetXcvrBaudRate => b"^BRX"
    format rate uint 1,
    if |cmd: &SetXcvrBaudRate| {
        if cmd.rate <= 3 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "rate",
                type_name: "u8",
                value: cmd.rate.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetDemoMode => b"^DMO");
impl_cat_command_with_response!(GetDemoMode => boolean);

impl_cat_command!(SetDemoMode => b"^DMO" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFanMinimumSpeed => b"^FC");
impl_cat_command_with_response!(GetFanMinimumSpeed => 1, u8_from_ascii => u8);

impl_cat_command!(
    SetFanMinimumSpeed => b"^FC"
    format level uint 1,
    if |cmd: &SetFanMinimumSpeed| {
        if cmd.level <= 6 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "level",
                type_name: "u8",
                value: cmd.level.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFaultCode => b"^FL");
impl_cat_command_with_response!(GetFaultCode => 2, u8_from_ascii => u8);

impl_cat_command!(ClearFault => b"^FL" with Some |_| {
    vec![b'C']
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetInhibitInput => b"^NH");
impl_cat_command_with_response!(GetInhibitInput => boolean);

impl_cat_command!(SetInhibitInput => b"^NH" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPowerStatus => b"^ON");
impl_cat_command_with_response!(GetPowerStatus => boolean);

impl_cat_command!(TurnPowerOff => b"^ON" with Some |_| {
    vec![b'0']
});

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetOperateMode => b"^OS");
impl_cat_command_with_response!(GetOperateMode => boolean);

impl_cat_command!(SetOperateMode => b"^OS" for boolean operate);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPowerAdjustment => b"^PJ");
impl_cat_command_with_response!(GetPowerAdjustment => 3, u8_from_ascii => u8);

impl_cat_command!(
    SetPowerAdjustment => b"^PJ"
    format value uint 3,
    if |cmd: &SetPowerAdjustment| {
        if (80..=120).contains(&cmd.value) {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "value",
                type_name: "u8",
                value: cmd.value.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFirmwareVersion => b"^RVM");
impl_cat_command_with_response!(GetFirmwareVersion => 0, bytes_to_vec => Vec<u8>);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetSerialNumber => b"^SN");
impl_cat_command_with_response!(GetSerialNumber => 5, u32_from_ascii => u32);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetFaultSpeaker => b"^SP");
impl_cat_command_with_response!(GetFaultSpeaker => boolean);

impl_cat_command!(SetFaultSpeaker => b"^SP" for state);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPaTemperature => b"^TM");
impl_cat_command_with_response!(GetPaTemperature => 3, u8_from_ascii => u8);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetTrDelay => b"^TR");
impl_cat_command_with_response!(GetTrDelay => 2, u8_from_ascii => u8);

impl_cat_command!(
    SetTrDelay => b"^TR"
    format ms uint 2,
    if |cmd: &SetTrDelay| {
        if cmd.ms <= 50 {
            Ok(())
        } else {
            Err(RigError::InvalidArgumentValue {
                argument_name: "ms",
                type_name: "u8",
                value: cmd.ms.to_string(),
            })
        }
    }
);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPaVoltageCurrent => b"^VI");
impl_cat_command_with_response!(GetPaVoltageCurrent => 7, |bytes: &[u8]| {
    Ok(PaVoltageCurrent {
        voltage_dv: u16_from_ascii(&bytes[0..3])?,
        current_da: u16_from_ascii(&bytes[4..7])?,
    })
} => PaVoltageCurrent);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetPowerAndSwr => b"^WS");
impl_cat_command_with_response!(GetPowerAndSwr => 7, |bytes: &[u8]| {
    Ok(PowerAndSwr {
        power_w: u16_from_ascii(&bytes[0..3])?,
        swr_d: u16_from_ascii(&bytes[4..7])?,
    })
} => PowerAndSwr);

// ------------------------------------------------------------------------------------------------

impl_cat_command!(GetRadioInterface => b"^XI");
impl_cat_command_with_response!(GetRadioInterface => 3, |bytes: &[u8]| {
    Ok(RadioInterface {
        interface_type: u8_from_ascii(&bytes[0..2])?,
        option: bytes[2] - b'0',
    })
} => RadioInterface);

impl_cat_command!(SetRadioInterface => b"^XI" with Some |cmd: &SetRadioInterface| {
    let mut v = format_uint_ascii(cmd.interface_type, 2);
    v.push(b'0' + cmd.option);
    v
}, if |cmd: &SetRadioInterface| {
    if cmd.interface_type > 3 {
        Err(RigError::InvalidArgumentValue {
            argument_name: "interface_type",
            type_name: "u8",
            value: cmd.interface_type.to_string(),
        })
    } else if cmd.option > 1 {
        Err(RigError::InvalidArgumentValue {
            argument_name: "option",
            type_name: "u8",
            value: cmd.option.to_string(),
        })
    } else {
        Ok(())
    }
});
