//!
//! Provides commands for Elecraft products, covering the K and KX series transceivers, KPA and KXPA
//! amplifiers, and KP and KXP panadapters.
//!
//! # Transceivers
//!
//! | Command                           | ID       | K2    | K3    | K3S   | K4    | KX2   | KX3   | KH1   |
//! |-----------------------------------|----------|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
//! | CaptureScreenshot                 | `SS`     |       |       |       | **Y** |       |       |       |
//! | CenterPanadapterOnVfoA            | `FC`     |       |       |       | **Y** |       |       |       |
//! | CenterPanadapterOnVfoB            | `FC$`    |       |       |       | **Y** |       |       |       |
//! | ClearRitOffset                    |          |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | CopyVfoAtoVfoB                    | `AB0`    |       |       |       | **Y** |       |       |       |
//! | DumpLog                           | `LG`     |       |       |       |       |       |       | **Y** |
//! | EmulateButtonHold                 | \[2]     |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | EmulateButtonTap                  | \[3]     |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | EmulateEncodeRotation             | `EN`     |       |       |       |       |       |       | **Y** |
//! | EmulateHandKeyPress               | `HK`     |       |       |       |       |       |       | **Y** |
//! | GetActiveSoftwareReleaseChannel   | `RL`     |       |       |       | **Y** |       |       |       |
//! | GetActualPowerOutput              |          |       |       |       |       | **Y** | **Y** |       |
//! | GetAgcTimeConstant                | `GT`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetAtuNetworkValues               | `AK`     |       |       |       |       | **Y** | **Y** |       |
//! | GetAudioPeakingFilterState        | `AP`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetAutoInfoMode                   | `AI`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetBargraphValue                  | `BG`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetCwSidetonePitch                | `CW`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetDataSubMode                    | `DT`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetDisplayText                    | `DS`     |       |       |       |       |       |       | **Y** |
//! | GetDiversityMode                  | `DV`     |       | **Y** | **Y** |       |       |       |       |
//! | GetEssbMode                       | `ES`     |       | **Y** | **Y** |       |       |       |       |
//! | GetFirmwareRevision               | `RV`     |       |       |       |       |       |       | **Y** |
//! | GetHelpInformation                | `H`      |       |       |       |       |       |       | **Y** |
//! | GetIfCenterFrequency              | `FI`     |       | **Y** |       |       |       |       |       |
//! | GetInstalledOptions               | `OM`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetK3IconsAndStatus               | `IC`     |       | **Y** | **Y** |       |       |       |       |
//! | GetKeyerSpeed                     | `KS`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMemoryChannel                  | `MC`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMenuParameter                  | `MP` \[4] |      | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | GetMenuParameter16                | `MQ`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMicGain                        | `MG`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMonitorLevel                   | `ML`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetReceiveAntenna                 | `AR`     |       | **Y** | **Y** |       |       |       |       |
//! | GetReceiveVfo                     | `FR`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetSpeechCompression              | `CP`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransceiverId                  | `I`      |       |       |       |       |       |       | **Y** |
//! | GetTransceiverInformation         | `IF`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransceiverSerialNumber        | `SN`     |       |       |       |       |       |       | **Y** |
//! | GetTransceiverStatus              | `ST`     |       |       |       |       |       |       | **Y** |
//! | GetTransmitLowerLimit             | `TXL`    |       |       |       |       |       |       | **Y** |
//! | GetTransmitUpperLimit             | `TXH`    |       |       |       |       |       |       | **Y** |
//! | GetTransmitVfoSplitModeState      | `FT`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAAfGain                     | `AG`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBAfGain                     | `AG$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoABandNumber                 | `BN`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBBandNumber                 | `BN$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoADisplayAndIcons            | `DS`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBDisplayText                | `DB`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAFilterBandwidth            | `BW`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBFilterBandwidth            | `BW$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAIfShift                    | `IS`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBIfShift                    | `IS$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoALegacyFilterBandwidth      | `FW`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBLegacyFilterBandwidth      | `FW$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoALock                       | `LK`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBLock                       | `LK$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAOperatingMode              | `MD`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBOperatingMode              | `MD$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoLinkedState                 | `LN`     |       | **Y** |       |       |       |       |       |
//! | LoadFirmware                      | `LD`     |       |       |       |       |       |       | **Y** |
//! | MoveVfoAFrequencyDown             | `DN`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoAFrequencyUp               | `UP`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoBFrequencyDown             | `DN$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoBFrequencyUp               | `UP$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SelectMenuItem                    | `MN` \[4] |      | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | SetAgcTimeConstant                | `GT`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetAfGain                         |  \[1]     |      |       |       |       |       |       | **Y** |
//! | SendCwText                        | `KY`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetAudioPeakingFilterState        | `AP`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetAutoInfoMode                   | `AI`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetBaudRate                       | `BR`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetCommandProcessingDelay         | `DE`     |       | **Y** | **Y** |       |       |       |       |
//! | SetDataSubMode                    | `DT`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetDisplayText                    | `DS`     |       |       |       |       |       |       | **Y** |
//! | SetDiversityMode                  | `DV`     |       | **Y** | **Y** |       |       |       |       |
//! | SetDspCommandDebugState           | `DL`     |       | **Y** | **Y** |       |       |       |       |
//! | SetErrorLogging                   | `EL`     |       |       |       |       | **Y** | **Y** |       |
//! | SetEssbMode                       | `ES`     |       | **Y** | **Y** |       |       |       |       |
//! | SetKeyerSpeed                     | `KS`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetMemoryChannel                  | `MC`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetMenuParameter                  | `MP` \[4] |      | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | GetMenuParameter16                | `MQ`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetMicGain                        | `MG`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetMonitorLevel                   | `ML`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetOperatingFrequency             | `FA`     |       |       |       |       |       |       | **Y** |
//! | SetOperatingMode                  | `MD`     |       |       |       |       |       |       | **Y** |
//! | SetReceiveAntenna                 | `AR`     |       | **Y** | **Y** |       |       |       |       |
//! | SetReceiveVfo                     | `FR`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetSpeechCompression              | `CP`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetTransmitVfoSplitModeState      | `FT`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAAfGain                     | `AG`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBAfGain                     | `AG$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoABandNumber                 | `BN`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBBandNumber                 | `BN$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBDisplayText                | `DB`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAFilterBandwidth            | `BW`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBFilterBandwidth            | `BW$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAIfShift                    | `IS`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBIfShift                    | `IS$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoALegacyFilterBandwidth      | `FW`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBLegacyFilterBandwidth      | `FW$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoALock                       | `LK`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBLock                       | `LK$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAOperatingMode              | `MD`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBOperatingMode              | `MD$`    |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoLinkedState                 | `LN`     |       | **Y** |       |       |       |       |       |
//! | SetVfoOffset                      | `FO`     |       |       |       |       |       |       | **Y** |
//!
//! ## Notes
//!
//! 1. For the KH1 the command ID is `AG` which is the same as the VFO A commands for dual VFO
//!    transceivers.
//! 2. For the KH1 the command ID is `SW{n}H`, for K3, K#S, KX2, and KX3 it is `SWH`.
//! 3. For the KH1 the command ID is `SW{n}T`, for K3, K#S, KX2, and KX3 it is `SWT`.
//! 4. While the command ID is the same, the different families have different argument tyes as well
//!    as different valid values for the arguments.
//!
//! ## Not Yet Categorized
//!
//! * GetAtuMode K4!
//! * GetAudioLineInputLevel K4!
//! * GetAudioLineOutputLevel K4!
//! * GetAudioMixRatio K4!
//! * GetBandIndependenceState K4!
//! * GetBufferedText
//! * GetCoarseTuningStep K4!
//! * GetCurrentBandPowerLimit K4!
//! * GetDigitalAudioRoutingMode K4!
//! * GetDigitalOutputPin1State K4!
//! * GetErrorReportingState K4!
//! * GetFirmwareRevision
//! * GetHighResolutionSMeter
//! * GetKeyerPaddleEmulationMode K4!
//! * GetMicInputSource K4!
//! * GetMonitorLevel
//! * GetPowerStatus
//! * GetPowerStatus K4!
//! * GetQskDelay
//! * GetRepeaterOffset K4!
//! * GetRitControl
//! * GetRitXitOffset
//! * GetScreenCount K4!
//! * GetStreamingLatencyClass K4!
//! * GetSubReceiver
//! * GetTransceiverId K4!
//! * GetTransceiverSerialNumber K4!
//! * GetTransmitBufferedText
//! * GetTransmitDataBandwidth K4!
//! * GetTransmitGain K4!
//! * GetTransmitGainConstant K4!
//! * GetTransmitMeterMode
//! * GetTransmitPowerControl
//! * GetTransmitState
//! * GetTransmitTestModeState K4!
//! * GetUtcTimestamp K4!
//! * GetVfoAAgcMode K4!
//! * GetVfoAAutoNotchState K4!
//! * GetVfoACtssTone K4!
//! * GetVfoAFilterPresetSlot K4!
//! * GetVfoAIfCenterPitch K4!
//! * GetVfoALock (LK).
//! * GetVfoAManualNotchSettings K4!
//! * GetVfoAModeAlternates K4!
//! * GetVfoANoiseBlanker (NB).
//! * GetVfoANoiseBlankerLevel (NL).
//! * GetVfoANoiseReductionSettings K4!
//! * GetVfoAPreamp (PA).
//! * GetVfoAReceiveAttenuator (RA).
//! * GetVfoARfGain (RG).
//! * GetVfoASMeter (SM).
//! * GetVfoASquelch (SQ).
//! * GetVfoATextDecodeMode K4!
//! * GetVfoATransverterActiveBandSlot K4!
//! * GetVfoATransverterOffset K4!
//! * GetVfoATuningStep K4!
//! * GetVfoAXfilNumber (XF).
//! * GetVfoBAgcMode K4!
//! * GetVfoBAutoNotchState K4!
//! * GetVfoBCtssTone K4!
//! * GetVfoBFilterPresetSlot K4!
//! * GetVfoBIfCenterPitch K4!
//! * GetVfoBLock (LK$).
//! * GetVfoBManualNotchSettings K4!
//! * GetVfoBModeAlternates K4!
//! * GetVfoBNoiseBlanker (NB$).
//! * GetVfoBNoiseBlankerLevel (NL$).
//! * GetVfoBNoiseReductionSettings K4!
//! * GetVfoBPreamp (PA$).
//! * GetVfoBReceiveAttenuator (RA$).
//! * GetVfoBRfGain (RG$).
//! * GetVfoBSMeter (SM$).
//! * GetVfoBSquelch (SQ$).
//! * GetVfoBTextDecodeMode K4!
//! * GetVfoBTransverterActiveBandSlot K4!
//! * GetVfoBTransverterOffset K4!
//! * GetVfoBTuningStep K4!
//! * GetVfoBXfilNumber (XF$).
//! * GetVox (VX).
//! * GetVoxGain K4!
//! * GetVoxInhibitState K4!
//! * GetWattmeterCalibrationConstant K4!
//! * GetXitControl (XT).
//! * GoToReceive (RX).
//! * GoToTransmit (TX).
//! * MoveRitOffsetDown (RD).
//! * MoveRitOffsetUp (RU).
//! * PlayDvrMessage K4!
//! * SetActiveSoftwareReleaseChannel K4!
//! * SetAtuMode K4!
//! * SetAtuTuningState K4!
//! * SetAudioLineInputLevel K4!
//! * SetAudioLineOutputLevel K4!
//! * SetAudioMixRatio K4!
//! * SetBandIndependenceState K4!
//! * SetCoarseTuningStep K4!
//! * SetCommandEchoState K4!
//! * SetCwSidetonePitch K4!
//! * SetDigitalAudioRoutingMode K4!
//! * SetDigitalOutputPin1State K4!
//! * SetErrorReportingState K4!
//! * SetK4QskOrVoxDelay K4!
//! * SetKeyerPaddleEmulationMode K4!
//! * SetKeyerSpeed K4!
//! * SetMicInputSource K4!
//! * SetMonitorLevel (ML).
//! * SetPowerStatus (PS).
//! * SetPowerStatus K4!
//! * SetRepeaterOffset K4!
//! * SetRitControl (RT).
//! * SetRitXitOffset (RO).
//! * SetStreamingLatencyClass K4!
//! * SetSubReceiver (SB).
//! * SetSystemAutoInfoInterval K4!
//! * SetTextToTerminal (TT).
//! * SetTransmitDataBandwidth K4!
//! * SetTransmitEqualizer (TE).
//! * SetTransmitMeterMode (TM).
//! * SetTransmitPowerControl (PC).
//! * SetTransmitTestModeState K4!
//! * SetVfoAAgcMode K4!
//! * SetVfoAAutoNotchState K4!
//! * SetVfoABandNumber (BN).
//! * SetVfoACtssTone K4!
//! * SetVfoAFilterPresetSlot K4!
//! * SetVfoALock (LK).
//! * SetVfoAManualNotchSettings K4!
//! * SetVfoANoiseBlanker (NB).
//! * SetVfoANoiseBlankerLevel (NL).
//! * SetVfoANoiseReductionSettings K4!
//! * SetVfoAPreamp (PA).
//! * SetVfoAReceiveAttenuator (RA).
//! * SetVfoARfGain (RG).
//! * SetVfoASquelch (SQ).
//! * SetVfoATextDecodeMode K4!
//! * SetVfoATransverterActiveBandSlot K4!
//! * SetVfoATuningStep K4!
//! * SetVfoBAgcMode K4!
//! * SetVfoBAutoNotchState K4!
//! * SetVfoBBandNumber (BN$).
//! * SetVfoBCtssTone K4!
//! * SetVfoBFilterPresetSlot K4!
//! * SetVfoBLock (LK$).
//! * SetVfoBManualNotchSettings K4!
//! * SetVfoBNoiseBlanker (NB$).
//! * SetVfoBNoiseBlankerLevel (NL$).
//! * SetVfoBNoiseReductionSettings K4!
//! * SetVfoBPreamp (PA$).
//! * SetVfoBReceiveAttenuator (RA$).
//! * SetVfoBRfGain (RG$).
//! * SetVfoBSquelch (SQ$).
//! * SetVfoBTextDecodeMode K4!
//! * SetVfoBTransverterActiveBandSlot K4!
//! * SetVfoBTuningStep K4!
//! * SetVox (VX).
//! * SetVoxGain K4!
//! * SetVoxInhibitState K4!
//! * SetWattmeterCalibrationConstant K4!
//! * SetXitControl (XT).
//! * SwapVfoAandVfoB K4!
//!
//! # Amplifiers
//!
//! | Command                       | ID     | KPA1500 | KPA500 | KXPA100 |
//! |-------------------------------|--------|:-------:|:------:|:-------:|
//!
//!
//! # Panadapters
//!
//! | Command                       | ID     | P3      | PX3    |
//! |-------------------------------|--------|:-------:|:------:|
//!
//!
//! # Tuners
//!
//! Only supports the KAT500 Automatic Antenna Tuner.
//!
//!
//! # References
//!
//! 1. [Elecraft K3S/K3/KX3 Programmer's Reference, rev. F2](./K3S&K3&KX3%20Pgmrs%20Ref,%20F2.pdf), Jul 2015.
//! 2. [ElecraftK3S/K3/KX3/KX2 Programmer's Reference, rev. G4](https://ftp.elecraft.com/KX2/Manuals%20Downloads/K3S&K3&KX3&KX2%20Pgmrs%20Ref,%20G4.pdf), November 2018.
//! 3. [ElecraftK3S/K3/KX3/KX2 Programmer's Reference, rev. G5](https://ftp.elecraft.com/K3S/Manuals%20Downloads/K3S&K3&KX3&KX2%20Pgmrs%20Ref,%20G5.pdf), Feb 2019.
//! 4. [K4 Programmer's Reference, rev. C7](https://lutz-electronics.ch/pdf/ELECRAFT/K4_Programmers_Reference_rev.C7.pdf), 2022.
//! 5. [K4 Programmer's Reference, rev. D11](https://ftp.elecraft.com/K4/Manuals%20Downloads/K4%20Programmer's%20Reference,%20rev.%20D12.pdf), May 2026
//! 6. [Elecraft KIO2 Programmer's Reference](https://ftp.elecraft.com/K2/Manuals%20Downloads/KIO2%20Pgmrs%20Ref%20rev%20E.pdf), Feb 2004.
//!    * Complete programmer's command reference for RS-232 computer control of the K2 with the KIO2 or KPA100.
//! 7. [Elecraft KH1 Programmer's Reference, rev. B2](https://ftp.elecraft.com/KH1/Manuals%20Downloads/Elecraft%20KH1%20Programmer's%20Ref,%20rev%20B2.pdf), Jan 2026.
//! 8. [Elecraft P3 Programmer's Reference, rev. A7](https://ftp.elecraft.com/P3/Manuals%20Downloads/P3_Pgmrs_Ref_Rev_A7.pdf), Apr 2016.
//! 9. [Elecraft PX3 Programmer's Reference, rev. A6](https://ftp.elecraft.com/PX3/Manuals%20Downloads/PX3_Pgmrs_Ref_A6.pdf), Feb 2017.
//! 10. [Elecraft KAT500 Automatic Antenna Tuner Command Reference](https://ftp.elecraft.com/KAT500/Manuals%20Downloads/KAT500%20Automatic%20Antenna%20Tuner%20Serial%20Command%20Reference.pdf), Sep 2023.
//! 11. [Elecraft KPA500 Programmer's Reference, rev A2](https://ftp.elecraft.com/KPA/Manuals%20Downloads/KPA500%20Programmers%20Ref.pdf)., Jul 2011
//! 12. [Elecraft KPA1500 Programmer's Reference, rev 3.03](https://ftp.elecraft.com/KPA1500/Manuals%20Downloads/KPA1500ProgrammingReferenceV3.pdf), Jun 2026.
//! 13. [Elecraft KXPA100 Programmer's Reference](https://ftp.elecraft.com/KXPA/Manuals%20Downloads/KXPA100%20Amplifier%20Command%20Reference.pdf), Feb 2014.
//!

// ------------------------------------------------------------------------------------------------
// Transceiver Sub-Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "meta")]
pub mod meta;

#[cfg(feature = "k3-kx")]
pub mod k3_kx;
#[cfg(feature = "k4")]
pub mod k4;
#[cfg(feature = "kh1")]
pub mod kh1;

// ------------------------------------------------------------------------------------------------
// Amplifier Sub-Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "kpa1500")]
pub mod kpa1500;
#[cfg(feature = "kpa500")]
pub mod kpa500;
#[cfg(feature = "kxpa100")]
pub mod kxpa100;

// ------------------------------------------------------------------------------------------------
// Panadapter Sub-Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "p3")]
pub mod p3;
#[cfg(feature = "px3")]
pub mod px3;

// ------------------------------------------------------------------------------------------------
// Tuner Sub-Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "kat500")]
pub mod kat500;
