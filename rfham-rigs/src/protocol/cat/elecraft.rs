//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

use crate::{
    AnalogVoiceMode, Frequency, MorseMode, OperatingMode,
    error::RigError,
    protocol::cat::{Command, Vfo, common::validate_response},
};
use std::fmt::Display;
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

command!(GetK2CommandMode);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K2CommandMode {
    extended: bool,
    rtty_off: bool,
}

command!(GetK3CommandMode);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K3CommandMode {
    extended: bool,
}

command!(GetK4CommandMode);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K4CommandMode {
    advanced: bool,
}

///
/// # Programmer Reference
///
/// RSP format: `OM APXSDFfLVR--;` where any of the characters **APXSDFfLVR**, if present, indicate installed
/// and detected option modules (see list below). The positions of the letters are fixed. If a module is
/// not present, its letter is replaced by a dash (`-`). For example, if only a PA and sub receiver were
/// installed, “`OM;`” would return “`OM -P-S--------;`”. Unused dashes are reserved for future module letters
/// and product ID.
///
/// **Option List**: The letters (and associated positions) in the OM string refer to the following
/// option modules:
///
/// * `A` = ATU (KAT3A)
/// * `P` = PA (KPA3A)
/// * `X` = XVTR and RX I/O (KXV3, KXV3A, or KXV3B)
/// * `S` = Sub Receiver (KRX3A)
/// * `D` = DVR (KDVR3)
/// * `F` = Band-Pass Filter module, main (KBPF3A)
/// * `f` = Band-Pass Filter module, sub (KBPF3A)
/// * `L` = Low-Noise Amplifier available on present band (preamp 2, only available on the KXV3B module)
/// * `V` = KSYN3A synthesizer (extends VFO tuning range; see note 2 below)
/// * `R` = K3S RF board.
///
/// **Note 1**: The presence of ‘R’ in the string (K3S RF board) is the preferred way to identify a K3S.
/// In this case: (1) Use the K3S format for the **RA** (receive attenuator) command; (2) poll for **OM**
/// after each band change to 12/10/6 meters to see if the LNA (preamp 2) is enabled. (See **PA** command
/// for information on preamp 2 use.)
///
/// **Note 2**: Presence of a KSYN3A (‘V’) extends VFO tuning range down to 100 kHz. However, to use
/// frequencies below 160 meters, a KBPF3 option module is required, and the receiving antenna should be
/// connected to RX ANT IN or XVTR IN on the KXV3B module (to bypass the high-pass filter in the T/R switch).
/// Low-level (0.5-1 mW) transmit below 160 meters is also possible via the XVTR OUT jack. Use of
/// frequencies below 600 meters (470 kHz) requires a KBPF3A, or a KBPF3 modified for LF use. See details
/// on the Elecraft web site.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K3InstalledOptions {
    pub has_kat3a_atu: bool,
    pub has_kpa3a_pa: bool,
    pub has_kxv3_xvtr: bool,
    pub has_krx3a_sub_receiver: bool,
    pub has_kdvr3_dvr: bool,
    pub has_kbpf3a_main_bpf: bool,
    pub has_kbpf3a_sub_bpf: bool,
    pub has_kxv3b_low_noise_amp: bool,
    pub has_ksyn3a: bool,
    pub has_k3s: bool,
}

///
/// # Programmer Reference
///
/// RESP format: `OM APXSHML14---;` where any of the option characters, when present,
/// indicate detected option modules (see list below). The positions of the letters are fixed. If a
/// module is not present, its letter is replaced by a dash (`-`). The letters (and associated
/// positions) in the OM string refer to the following option modules:
///
/// * `A` = ATU (KAT4)
/// * `P` = PA (KPA4)
/// * `X` = XVTR (additional fields will be used to ID different XVTR models)
/// * `S` = SUB RX (KRX4 + 2nd KDDC4, standard in K4D model)
/// * `H` = HDR MODULE (KHDR4 + KDDC4-2, standard in K4HD model)
/// * `M` = K40 (or "Mini")
/// * `L` = LINEAR AMP detected (generic, i.e. Elecraft KPA500 or KPA1500)
/// * `1` = KPA1500 amp detected
/// * `4` = Identifies the radio as a K4 (S & 4 = K4D; S, H, and 4 = K4HD)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K4InstalledOptions {
    pub has_kat4_atu: bool,
    pub has_kpa4_pa: bool,
    pub has_xvtr: bool,
    pub has_krx4_sub_receiver: bool,
    pub has_khdr4_hdr: bool,
    pub has_k40_mini: bool,
    pub has_kpa_linear_pa: bool,
    pub has_kpa1500_pa: bool,
}

///
/// # Programmer Reference
///
/// **RSP format**: `OM APF---TBXI0n;` where any of the characters **APFTBXI**, if present, indicate
/// installed and detected option modules (see list below), and `0n` (zero, not ‘O’) is the product
/// identifier (n=`1` for KX2, n=`2` for KX3). The positions of the letters are fixed. If a module is
/// not present, its letter is replaced by a dash (`-`). For example, if only KXAT3 antenna tuner and
/// KXFL3 roofing filter modules were installed, “`OM;`” would return “`OM A-F-------02;`”. Unused
/// dashes are reserved for future module letters.
///
/// **Option List**: The letters (and associated positions) in the OM string refer to the following
/// KX3 or KX2 option modules:
///
/// * `A` = ATU (KXAT3 or KXAT2)
/// * `P` = external 100-W PA (KXPA100)
/// * `F` = roofing filter (KXFL3)
/// * `T` = external 100-W ATU (KXAT100, a KXPA100 internal option)
/// * `B` = internal NiMH battery-charger/real-time clock (KXBC3)
/// * `X` = KX3-2M or KX3-4M transverter module
/// * `I` = KXIO2 RTC I/O module.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KXInstalledOptions {
    pub has_kxat_atu: bool,
    pub has_kxpa100_pa: bool,
    pub has_kxfl3_roofing_filter: bool,
    pub has_kxat100_external_atu: bool,
    pub has_kxbc3_realtime_clock: bool,
    pub has_kx3m_transverter: bool,
    pub has_kxio2_rtc_io: bool,
    pub is_kx3: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstalledOptions {
    K3(K3InstalledOptions),
    K4(K4InstalledOptions),
    KX(KXInstalledOptions),
}

command!(GetInstalledOptions);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct K3IconsAndStatus {
    preset_1: bool,
    preset_2: bool,
    band_sel: bool,
    msg_playing: bool,
    message_bank_1: bool,
    message_bank_2: bool,
    mw_power_level: bool,
    tx_test: bool,
    bset: bool,
    // --------
    sub_rx_on: bool,
    sub_rx_nb_on: bool,
    sub_rx_aux_bnc: bool,
    sub_ant_main: bool,
    sub_ant_aux: bool,
    diversity_mode: bool,
    vfo_ab_bands_independent: bool,
    vfo_ab_linked: bool,
    // --------
    text_to_terminal_on: bool,
    sync_data: bool,
    fsk_tx_polarity_normal: bool,
    fsk_dual_tone_filter_on: bool,
    vox_on_for_cw_fsk_psk: bool,
    dual_passband_cf_apf_on: bool,
    full_qsk: bool,
    // --------
    repeater_tx_offset_negative: bool,
    repeater_tx_offset_positive: bool,
    fm_pl_tone_on: bool,
    am_sync_rx: bool,
    noise_gate_on: bool,
    essb: bool,
    vox_on_for_voice_data_afsk: bool,
    // --------
    fast_play_in_effect: bool,
    ofs_led_is_on: bool,
    vfob_led_on: bool,
    sub_rx_nr_on: bool,
    sub_rx_squelched: bool,
    main_rx_squelched: bool,
    am_sync_usb: bool,
    am_sync_lsb: bool,
    shift_10_hz: bool,
    shift_50_hz: bool,
}

command!(GetK3IconsAndStatus);

///
/// # Programmer Reference
///
/// RSP format: IF[f]*****+yyyyrx*00tmvspbd1*; where the fields are defined as follows:
///
/// * `[f]` Operating frequency, excluding any RIT/XIT offset (11 digits; see FA command format)
/// * `*` represents a space (BLANK, or ASCII 0x20)
/// * `+` either "+" or "-" (sign of RIT/XIT offset)
/// * `yyyy` RIT/XIT offset in Hz (range is -9999 to +9999 Hz when computer-controlled)
/// * `r` 1 if RIT is on, 0 if off
/// * `x` 1 if XIT is on, 0 if off
/// * `t` 1 if the K3 is in transmit mode, 0 if receive
/// * `m` operating mode (see MD command)
/// * `v` receive-mode VFO selection, 0 for VFO A, 1 for VFO B
///   * **NOTE** on K4 this is only ever returned as 0.
/// * `s` 1 if scan is in progress, 0 otherwise
/// * `p` 1 if the transceiver is in split mode, 0 otherwise
/// * `b` Basic RSP format: always 0; K2 Extended RSP format (K22):
///   1 if present IF response is due to a band change; 0 otherwise
/// * `d` Basic RSP format: always 0; K3 Extended RSP format (K31):
///   DATA sub-mode, if applicable (0=DATA A, 1=AFSK A, 2= FSK D, 3=PSK D)
///
/// The fixed-value fields (space, 0, and 1) are provided for syntactic compatibility with existing software.
///
/// In the K4 programmer reference it is note that this command is *Provided for K3 compatibility
/// (K22/K31 meta-mode variant).*
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransceiverInformation {
    operating_frequency: Frequency,
    rit_xit_sign_negative: bool,
    rit_xit_offset_hz: u16,
    rit_on: bool,
    xit_on: bool,
    in_transmit_mode: bool,
    operating_mode: u8,
    receive_mode_vfo: Vfo,
    scan_in_progress: bool,
    in_split_mode: bool,
    data_sub_mode: Option<u8>,
}

command!(GetTransceiverInformation);

command! { GetOperatingMode =>
    vfo: Vfo
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

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
            error!(
                "invalid response length; expecting 1, given {}",
                value.len()
            );
            return Err(RigError::InvalidResponseLength {
                expecting: 1,
                given: value.len(),
            });
        }
        Ok(Self {
            extended: value[0] & 0b0010 == 1,
            rtty_off: value[0] & 0b0001 == 1,
        })
    }
}

impl_command!(GetK2CommandMode, b"K2");
impl_command_with_response!(GetK2CommandMode => try_from 1 K2CommandMode);

// ------------------------------------------------------------------------------------------------

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
            error!(
                "invalid response length; expecting 1, given {}",
                value.len()
            );
            return Err(RigError::InvalidResponseLength {
                expecting: 1,
                given: value.len(),
            });
        }
        Ok(Self {
            extended: value[0] & 0b0001 == 1,
        })
    }
}

impl_command!(GetK3CommandMode, b"K3");
impl_command_with_response!(GetK3CommandMode => try_from 1 K3CommandMode);

// ------------------------------------------------------------------------------------------------

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
            error!(
                "invalid response length; expecting 1, given {}",
                value.len()
            );
            return Err(RigError::InvalidResponseLength {
                expecting: 1,
                given: value.len(),
            });
        }
        Ok(Self {
            advanced: value[0] & 0b0001 == 1,
        })
    }
}

impl_command!(GetK4CommandMode, b"K4");
impl_command_with_response!(GetK4CommandMode => try_from 1 K4CommandMode);

// ------------------------------------------------------------------------------------------------

impl_command!(GetInstalledOptions, b"OM");
impl_command_with_response!(GetInstalledOptions => try_from 13 InstalledOptions);

// ------------------------------------------------------------------------------------------------

impl TryFrom<&[u8]> for InstalledOptions {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.starts_with(b" ") {
            if value.ends_with(b"4---") {
                Ok(Self::K4(K4InstalledOptions::try_from(&value[1..])?))
            } else if value.ends_with(b"--") {
                Ok(Self::K3(K3InstalledOptions::try_from(&value[1..])?))
            } else if value[11] == b'0' {
                Ok(Self::KX(KXInstalledOptions::try_from(&value[1..])?))
            } else {
                error!("could not map response data to rig model");
                Err(RigError::InvalidResponseData {
                    data: value.to_vec(),
                })
            }
        } else {
            error!("missing space separator");
            Err(RigError::InvalidResponseData {
                data: value.to_vec(),
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------

macro_rules! parse_option {
    ($name:ident => $bytes:ident [ $index:literal ] == $present:literal) => {
        match $bytes[$index] {
            $present => true,
            b'-' => false,
            _ => {
                ::tracing::error!(
                    "byte value {:02X?} not valid for option {}",
                    $bytes[$index],
                    stringify!($name)
                );
                return Err($crate::error::RigError::InvalidResponseData {
                    data: $bytes.to_vec(),
                });
            }
        }
    };
}

impl TryFrom<&[u8]> for K3InstalledOptions {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            has_kat3a_atu: parse_option!(has_kat3a_atu => value[0] == b'A'),
            has_kpa3a_pa: parse_option!(has_kpa3a_pa => value[1] == b'P'),
            has_kxv3_xvtr: parse_option!(has_kxv3_xvtr => value[2] == b'X'),
            has_krx3a_sub_receiver: parse_option!(has_krx3a_sub_receiver => value[3] == b'S'),
            has_kdvr3_dvr: parse_option!(has_kdvr3_dvr => value[4] == b'D'),
            has_kbpf3a_main_bpf: parse_option!(has_kbpf3a_main_bpf => value[5] == b'F'),
            has_kbpf3a_sub_bpf: parse_option!(has_kbpf3a_sub_bpf => value[6] == b'f'),
            has_kxv3b_low_noise_amp: parse_option!(has_kxv3b_low_noise_amp => value[7] == b'L'),
            has_ksyn3a: parse_option!(has_ksyn3a => value[8] == b'V'),
            has_k3s: parse_option!(has_k3s => value[9] == b'R'),
        })
    }
}

impl K3InstalledOptions {
    pub fn is_model_k3(&self) -> bool {
        !self.has_k3s
    }

    pub fn is_model_k3s(&self) -> bool {
        self.has_k3s
    }
}

// ------------------------------------------------------------------------------------------------

impl TryFrom<&[u8]> for K4InstalledOptions {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            has_kat4_atu: parse_option!(has_kat4_atu => value[0] == b'A'),
            has_kpa4_pa: parse_option!(has_kpa4_pa => value[1] == b'P'),
            has_xvtr: parse_option!(has_xvtr => value[2] == b'X'),
            has_krx4_sub_receiver: parse_option!(has_krx4_sub_receiver => value[3] == b'S'),
            has_khdr4_hdr: parse_option!(has_khdr4_hdr => value[4] == b'H'),
            has_k40_mini: parse_option!(has_k40_mini => value[5] == b'M'),
            has_kpa_linear_pa: parse_option!(has_kpa_linear_pa => value[6] == b'L'),
            has_kpa1500_pa: parse_option!(has_kpa1500_pa => value[7] == b'1'),
        })
    }
}

impl K4InstalledOptions {
    pub fn is_k4(&self) -> bool {
        !self.has_krx4_sub_receiver && !self.has_khdr4_hdr
    }

    pub fn is_k4d(&self) -> bool {
        self.has_krx4_sub_receiver && !self.has_khdr4_hdr
    }

    pub fn is_k4hd(&self) -> bool {
        self.has_krx4_sub_receiver && self.has_khdr4_hdr
    }
}

// ------------------------------------------------------------------------------------------------

impl TryFrom<&[u8]> for KXInstalledOptions {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            has_kxat_atu: parse_option!(has_kxat_atu => value[0] == b'A'),
            has_kxpa100_pa: parse_option!(has_kxpa100_pa => value[1] == b'P'),
            has_kxfl3_roofing_filter: parse_option!(has_kxfl3_roofing_filter => value[2] == b'F'),
            has_kxat100_external_atu: parse_option!(has_kxat100_external_atu => value[6] == b'T'),
            has_kxbc3_realtime_clock: parse_option!(has_kxbc3_realtime_clock => value[7] == b'B'),
            has_kx3m_transverter: parse_option!(has_kx3m_transverter => value[8] == b'X'),
            has_kxio2_rtc_io: parse_option!(has_kxio2_rtc_io => value[9] == b'I'),
            is_kx3: match value[11] {
                b'2' => true,
                b'1' => false,
                _ => {
                    ::tracing::error!("byte value {:02X?} not valid for option is_kx3", value[11],);
                    return Err(RigError::InvalidResponseData {
                        data: value.to_vec(),
                    });
                }
            },
        })
    }
}

impl KXInstalledOptions {
    pub fn is_model_kx2(&self) -> bool {
        !self.is_kx3
    }

    pub fn is_model_kx3(&self) -> bool {
        self.is_kx3
    }
}

// ------------------------------------------------------------------------------------------------

macro_rules! parse_flag {
    ($byte:literal : $bit:literal == $state:literal in $bytes:expr) => {
        $bytes[$byte] & 1 << $bit == $state
    };
    ($byte:literal : $bit:literal ON in $bytes:expr) => {
        parse_flag!($byte:$bit == 1 in $bytes)
    };
    ($byte:literal : $bit:literal OFF in $bytes:expr) => {
        parse_flag!($byte:$bit == 0 in $bytes)
    };
    ($byte:literal : $bit:literal in $bytes:expr) => {
        parse_flag!($byte:$bit ON in $bytes)
    };
}

impl TryFrom<&[u8]> for K3IconsAndStatus {
    type Error = RigError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(K3IconsAndStatus {
            preset_1: parse_flag!(0:0 OFF in bytes),
            preset_2: parse_flag!(0:0 ON in bytes),
            band_sel: parse_flag!(0:1 OFF in bytes),
            msg_playing: parse_flag!(0:2 in bytes),
            message_bank_1: parse_flag!(0:3 OFF in bytes),
            message_bank_2: parse_flag!(0:3 ON in bytes),
            mw_power_level: parse_flag!(0:4 in bytes),
            tx_test: parse_flag!(0:5 in bytes),
            bset: parse_flag!(0:6 in bytes),
            // --------
            sub_rx_on: parse_flag!(1:0 in bytes),
            sub_rx_nb_on: parse_flag!(1:1 in bytes),
            sub_rx_aux_bnc: parse_flag!(1:2 in bytes),
            sub_ant_main: parse_flag!(1:3 ON in bytes),
            sub_ant_aux: parse_flag!(1:3 OFF in bytes),

            diversity_mode: parse_flag!(1:4 in bytes),
            vfo_ab_bands_independent: parse_flag!(1:5 in bytes),
            vfo_ab_linked: parse_flag!(1:6 in bytes),
            // --------
            text_to_terminal_on: parse_flag!(2:0 in bytes),
            sync_data: parse_flag!(2:1 in bytes),
            fsk_tx_polarity_normal: parse_flag!(2:2 in bytes),
            fsk_dual_tone_filter_on: parse_flag!(2:3 in bytes),
            vox_on_for_cw_fsk_psk: parse_flag!(2:4 in bytes),
            dual_passband_cf_apf_on: parse_flag!(2:5 in bytes),
            full_qsk: parse_flag!(2:6 in bytes),
            // --------
            repeater_tx_offset_negative: parse_flag!(3:0 in bytes),
            repeater_tx_offset_positive: parse_flag!(3:1 in bytes),
            fm_pl_tone_on: parse_flag!(3:2 in bytes),
            am_sync_rx: parse_flag!(3:3 in bytes),
            noise_gate_on: parse_flag!(3:4 in bytes),
            essb: parse_flag!(3:5 in bytes),
            vox_on_for_voice_data_afsk: parse_flag!(3:6 in bytes),
            // --------
            fast_play_in_effect: parse_flag!(4:0 in bytes),
            ofs_led_is_on: parse_flag!(4:1 ON in bytes),
            vfob_led_on: parse_flag!(4:1 OFF in bytes),
            sub_rx_nr_on: parse_flag!(4:2 in bytes),
            sub_rx_squelched: parse_flag!(4:3 in bytes),
            main_rx_squelched: parse_flag!(4:4 in bytes),
            am_sync_usb: parse_flag!(4:5 ON in bytes),
            am_sync_lsb: parse_flag!(4:5 OFF in bytes),
            shift_10_hz: parse_flag!(4:6 ON in bytes),
            shift_50_hz: parse_flag!(4:6 OFF in bytes),
        })
    }
}

impl_command!(GetK3IconsAndStatus, b"IC");
impl_command_with_response!(GetK3IconsAndStatus => try_from 5 K3IconsAndStatus);

// ------------------------------------------------------------------------------------------------

impl TryFrom<&[u8]> for OperatingMode {
    type Error = RigError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            match value[0] {
                b'1' => Ok(OperatingMode::AnalogVoice(AnalogVoiceMode::LowerSideBand)),
                b'2' => Ok(OperatingMode::AnalogVoice(AnalogVoiceMode::UpperSideBand)),
                b'3' => Ok(OperatingMode::Morse(MorseMode::ContinuousWave)),
                b'4' => Ok(OperatingMode::AnalogVoice(
                    AnalogVoiceMode::FrequencyModulated,
                )),
                b'5' => Ok(OperatingMode::AnalogVoice(
                    AnalogVoiceMode::AmplitudeModulated,
                )),
                b'6' => Ok(OperatingMode::Data(crate::DataMode::Unspecified)),
                b'7' => Ok(OperatingMode::Morse(MorseMode::ContinuousWaveReverse)),
                b'9' => Ok(OperatingMode::Data(crate::DataMode::Unspecified)),
                _ => {
                    ::tracing::error!("unknown data mode value {:02X?}", value);
                    Err(RigError::InvalidResponseData {
                        data: value.to_vec(),
                    })
                }
            }
        } else {
            error!(
                "invalid response length; expecting 1, given {}",
                value.len()
            );
            Err(RigError::InvalidResponseLength {
                expecting: 1,
                given: value.len(),
            })
        }
    }
}

impl Command for GetOperatingMode {
    fn command_id(&self) -> &[u8] {
        match self.vfo {
            Vfo::A => b"MD",
            Vfo::B => b"MD$",
            _ => panic!(),
        }
    }
}

impl_command_with_response!(GetOperatingMode => try_from 1 OperatingMode);

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {}
