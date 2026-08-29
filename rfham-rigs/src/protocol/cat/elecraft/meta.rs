//!
//! Meta-commands common to all Elecraft transceivers.
//!
//! Covers: K2/K3/K4 command-mode selection
//!
//! # References
//!
//! 1. [ElecraftK3S/K3/KX3/KX2 Programmer's Reference, rev. G5](https://ftp.elecraft.com/K3S/Manuals%20Downloads/K3S&K3&KX3&KX2%20Pgmrs%20Ref,%20G5.pdf), Feb 2019.
//! 2. [K4 Programmer's Reference, rev. D11](https://ftp.elecraft.com/K4/Manuals%20Downloads/K4%20Programmer's%20Reference,%20rev.%20D12.pdf), May 2026
//! 3. [Elecraft KIO2 Programmer's Reference](https://ftp.elecraft.com/K2/Manuals%20Downloads/KIO2%20Pgmrs%20Ref%20rev%20E.pdf), Feb 2004.
//!    * Complete programmer's command reference for RS-232 computer control of the K2 with the KIO2 or KPA100.
//! 4. [Elecraft KH1 Programmer's Reference, rev. B2](https://ftp.elecraft.com/KH1/Manuals%20Downloads/Elecraft%20KH1%20Programmer's%20Ref,%20rev%20B2.pdf), Jan 2026.

use crate::{
    error::{RigError, invalid_response_length},
    protocol::cat::{Command, common::validate_response},
};
use std::fmt::Display;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types: GetK2CommandMode, SetK2CommandMode, K2CommandMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the K2 command mode.

The K3/K3S also accepts this command and behaves identically.

# Command format

> `K2;`

# Response format

> `K2{n};`

Where *n* is:

* `0`; Normal RSP, RTTY decode on
* `1`; Normal RSP, RTTY decode off
* `2`; Extended RSP, RTTY decode on
* `3`; Extended RSP, RTTY decode off" =>
    GetK2CommandMode
);

define_command!("Set the K2 command mode.

The K3/K3S also accepts this command and behaves identically.

# Command format

> `K2{n};`

Where *n* is:

* `0`; Normal RSP, RTTY decode on
* `1`; Normal RSP, RTTY decode off
* `2`; Extended RSP, RTTY decode on
* `3`; Extended RSP, RTTY decode off" =>
    SetK2CommandMode {
        mode: K2CommandMode
    }
);

define_command_struct!(
    "Represents the parsed K2 command-mode response." =>
    K2CommandMode {
        extended: bool,
        rtty_off: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetK3CommandMode, SetK3CommandMode, K3CommandMode
// ------------------------------------------------------------------------------------------------

define_command!("Get the K3 meta-command mode.

Software should set K31 immediately after connecting to enable full functionality.

# Command format

> `K3;`

# Response format

> `K3{n};`

Where *n* is:

* `0`; Normal RSP format, the default after power-on
* `1`; Extended RSP format, K31 mode, enables extra fields in IF, etc." =>
    GetK3CommandMode
);

define_command!("Set the K3 meta-command mode.

Software should set K31 immediately after connecting to enable full functionality.

# Command format

> `K3{n};`

Where *n* is:

* `0`; Normal RSP format, the default after power-on
* `1`; Extended RSP format, K31 mode, enables extra fields in IF, etc." =>
    SetK3CommandMode {
        mode: K3CommandMode
    }
);

define_command_struct!(
    "Represents the parsed K3 command-mode response." =>
    K3CommandMode {
        "`true` for extended RSP format, `false` for normal RSP format." =>
        extended: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Public Types: GetK4CommandMode, SetK4CommandMode, K4CommandMode
// ------------------------------------------------------------------------------------------------

define_command!("Get K4 meta-command mode.

K41 mode is required to use many K4-specific commands and response fields.

# Command format

> `K4;`

# Response format

> `K4{n};`

Where *n* is:

* `0`; Normal, K3-compatible RSP format; default after power-on
* `1`; Advanced, K4 extended RSP format — K41 mode" => 
    GetK4CommandMode
);

define_command!("Set K4 meta-command mode.

K41 mode is required to use many K4-specific commands and response fields.

# Command format

> `K4{n};`

Where *n* is:

* `0`; Normal, K3-compatible RSP format; default after power-on
* `1`; Advanced, K4 extended RSP format — K41 mode" => 
    SetK4CommandMode {
        mode: K4CommandMode
    }
);

define_command_struct!(
    "Represents the parsed K4 command-mode response." =>
    K4CommandMode {
        "`true` for advanced, `false` for normal." =>
        advanced: bool
    }
);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl_command!(GetK2CommandMode => b"K2");
impl_command_with_response!(GetK2CommandMode => try_from 1 K2CommandMode);

impl_command!(SetK2CommandMode => b"K2" with Some |cmd: &SetK2CommandMode| {
    vec![
            b'0' + ((cmd.mode.extended as u8) << 1) + (cmd.mode.rtty_off as u8),
        ]
});

impl Display for K2CommandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.extended, self.rtty_off) {
            (false, false) => "K2 Normal Mode",
            (false, true) => "K2 Normal/RTTY off",
            (true, false) => "K2 Extended Mode",
            (true, true) => "K2 Extended/RTTY off",
        }
        .fmt(f)
    }
}

impl TryFrom<&[u8]> for K2CommandMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("K2CommandMode: expecting 1 byte, given {}", value.len());
            Err(invalid_response_length(1, value.len()))
        } else {
            Ok(Self {
                extended: value[0] & 0b0010 != 0,
                rtty_off: value[0] & 0b0001 != 0,
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetK3CommandMode => b"K3");
impl_command_with_response!(GetK3CommandMode => try_from 1 K3CommandMode);

impl_command!(SetK3CommandMode => b"K3" with Some |cmd: &SetK3CommandMode| {
    vec![if cmd.mode.extended { b'1' } else { b'0' }]
});

impl Display for K3CommandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.extended {
            "K3 Extended Mode"
        } else {
            "K3 Normal Mode"
        }
        .fmt(f)
    }
}

impl TryFrom<&[u8]> for K3CommandMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("K3CommandMode: expecting 1 byte, given {}", value.len());
            Err(invalid_response_length(1, value.len()))
        } else {
            Ok(Self {
                extended: value[0] == b'1',
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_command!(GetK4CommandMode => b"K4");
impl_command_with_response!(GetK4CommandMode => try_from 1 K4CommandMode);

impl_command!(SetK4CommandMode => b"K4" with Some |cmd: &SetK4CommandMode| {
    vec![if cmd.mode.advanced { b'1' } else { b'0' }]
});

impl Display for K4CommandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.advanced {
            "K4 Advanced Mode"
        } else {
            "K4 Normal Mode"
        }
        .fmt(f)
    }
}

impl TryFrom<&[u8]> for K4CommandMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            error!("K4CommandMode: expecting 1 byte, given {}", value.len());
            Err(invalid_response_length(1, value.len()))
        } else {
            Ok(Self {
                advanced: value[0] == b'1',
            })
        }
    }
}
