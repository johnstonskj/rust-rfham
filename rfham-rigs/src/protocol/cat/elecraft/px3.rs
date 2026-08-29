//!
//! CAT commands for the Elecraft PX3 panadapter.
//!
//! The PX3 shares the full P3 command set with the [`super::p3`] module; this module documents
//! only the PX3-specific additions. All PX3-only commands use the `#` prefix.
//!
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//!
//! 1. [Elecraft PX3 Programmer's Reference, rev. A6](https://ftp.elecraft.com/PX3/Manuals%20Downloads/PX3_Pgmrs_Ref_A6.pdf), Feb 2017.
//!

use crate::{
    error::RigError,
    protocol::cat::{
        Command, CommandWithResponse,
        common::{u8_from_ascii, validate_response},
    },
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandscopeChannelIndicatorState, SetBandscopeChannelIndicatorState
// ------------------------------------------------------------------------------------------------

define_command!("Get the bandscope channel indicator on/off state.

# Command format

> `#BCI;`

# Response format

> `#BCI{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetBandscopeChannelIndicatorState
);

define_command!("Set the bandscope channel indicator on/off state.

# Command format

> `#BCI{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetBandscopeChannelIndicatorState {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandscopeChannelList, SetBandscopeChannelList
// ------------------------------------------------------------------------------------------------

define_command!("Get the bandscope channel list overlay on/off state.

# Command format

> `#BCL;`

# Response format

> `#BCL{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetBandscopeChannelList
);

define_command!("Set the bandscope channel list overlay on/off state.

# Command format

> `#BCL{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetBandscopeChannelList {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetBandscopeChannelName, SetBandscopeChannelName
// ------------------------------------------------------------------------------------------------

define_command!("Get the bandscope channel name overlay on/off state.

# Command format

> `#BCN;`

# Response format

> `#BCN{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetBandscopeChannelName
);

define_command!("Set the bandscope channel name overlay on/off state.

# Command format

> `#BCN{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetBandscopeChannelName {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetCalibSignal, SetCalibSignal
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the internal calibration signal is active.

# Command format

> `#CAL;`

# Response format

> `#CAL{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetCalibSignal
);

define_command!("Set whether the internal calibration signal is active.

# Command format

> `#CAL{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetCalibSignal {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMemoryAntennaA
// ------------------------------------------------------------------------------------------------

define_command!("Get the antenna band-map stored in memory slot A.

# Command format

> `#MAA;`

# Response format

> `#MAA...;`

The response is a variable-length, model-specific antenna band-map, returned as raw bytes." =>
    GetMemoryAntennaA
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetMemoryAntennaB
// ------------------------------------------------------------------------------------------------

define_command!("Get the antenna band-map stored in memory slot B.

# Command format

> `#MBA;`

# Response format

> `#MBA...;`

The response is a variable-length, model-specific antenna band-map, returned as raw bytes." =>
    GetMemoryAntennaB
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOffscreenBandscopePosition, SetOffscreenBandscopePosition
// ------------------------------------------------------------------------------------------------

define_command!("Get the off-screen bandscope vertical position.

# Command format

> `#OSBP;`

# Response format

> `#OSBP{nn};`

Where *nn* is the vertical position, in pixels, from the top of the screen." =>
    GetOffscreenBandscopePosition
);

define_command!("Set the off-screen bandscope vertical position.

# Command format

> `#OSBP{nn};`

Where *nn* is the vertical position, in pixels, from the top of the screen." =>
    SetOffscreenBandscopePosition {
        position: u8
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetOffscreenBandscopeActive, SetOffscreenBandscopeActive
// ------------------------------------------------------------------------------------------------

define_command!("Get whether the off-screen bandscope is active.

# Command format

> `#OSBA;`

# Response format

> `#OSBA{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetOffscreenBandscopeActive
);

define_command!("Set whether the off-screen bandscope is active.

# Command format

> `#OSBA{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetOffscreenBandscopeActive {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTxHold, SetTxHold
// ------------------------------------------------------------------------------------------------

define_command!("Get the TX hold display on/off state.

# Command format

> `#TXH;`

# Response format

> `#TXH{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetTxHold
);

define_command!("Set the TX hold display on/off state.

# Command format

> `#TXH{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetTxHold {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetTxMarker, SetTxMarker
// ------------------------------------------------------------------------------------------------

define_command!("Get the TX frequency marker display on/off state.

# Command format

> `#TXM;`

# Response format

> `#TXM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetTxMarker
);

define_command!("Set the TX frequency marker display on/off state.

# Command format

> `#TXM{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetTxMarker {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetUsbAudioEnable, SetUsbAudioEnable
// ------------------------------------------------------------------------------------------------

define_command!("Get whether USB audio output is enabled.

# Command format

> `#USB;`

# Response format

> `#USB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    GetUsbAudioEnable
);

define_command!("Set whether USB audio output is enabled.

# Command format

> `#USB{n};`

Where `n` is the boolean state `0` (off) or `1` (on)." =>
    SetUsbAudioEnable {
        enabled: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_command!(GetBandscopeChannelIndicatorState => b"#BCI");
impl_command_with_response!(GetBandscopeChannelIndicatorState => boolean);

impl_command!(SetBandscopeChannelIndicatorState => b"#BCI" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetBandscopeChannelList => b"#BCL");
impl_command_with_response!(GetBandscopeChannelList => boolean);

impl_command!(SetBandscopeChannelList => b"#BCL" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetBandscopeChannelName => b"#BCN");
impl_command_with_response!(GetBandscopeChannelName => boolean);

impl_command!(SetBandscopeChannelName => b"#BCN" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetCalibSignal => b"#CAL");
impl_command_with_response!(GetCalibSignal => boolean);

impl_command!(SetCalibSignal => b"#CAL" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetMemoryAntennaA => b"#MAA");

impl CommandWithResponse for GetMemoryAntennaA {
    type Response = Vec<u8>;

    fn expected_response_length(&self) -> usize {
        0
    }

    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"#MAA", 0)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetMemoryAntennaB => b"#MBA");

impl CommandWithResponse for GetMemoryAntennaB {
    type Response = Vec<u8>;

    fn expected_response_length(&self) -> usize {
        0
    }

    fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>, RigError> {
        let d = validate_response(bytes, b"#MBA", 0)?;
        Ok(d.to_vec())
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetOffscreenBandscopePosition => b"#OSBP");
impl_command_with_response!(GetOffscreenBandscopePosition => 2, u8_from_ascii => u8);

impl_command!(SetOffscreenBandscopePosition => b"#OSBP" format position uint 2);

// ------------------------------------------------------------------------------------------------

impl_command!(GetOffscreenBandscopeActive => b"#OSBA");
impl_command_with_response!(GetOffscreenBandscopeActive => boolean);

impl_command!(SetOffscreenBandscopeActive => b"#OSBA" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTxHold => b"#TXH");
impl_command_with_response!(GetTxHold => boolean);

impl_command!(SetTxHold => b"#TXH" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetTxMarker => b"#TXM");
impl_command_with_response!(GetTxMarker => boolean);

impl_command!(SetTxMarker => b"#TXM" for boolean enabled);

// ------------------------------------------------------------------------------------------------

impl_command!(GetUsbAudioEnable => b"#USB");
impl_command_with_response!(GetUsbAudioEnable => boolean);

impl_command!(SetUsbAudioEnable => b"#USB" for boolean enabled);
