//!
//! Provides commands for Elecraft products, covering the K and KX series transceivers, KPA and KXPA
//! amplifiers, and KP and KXP panadapters.
//!
//! # Transceivers
//!
//! | Command                           | ID        | K2    | K3    | K3S   | K4    | KX2   | KX3   | KH1   |
//! |-----------------------------------|-----------|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
//! | CaptureScreenshot                 | `SS`      |       |       |       | **Y** |       |       |       |
//! | CenterPanadapterOnVfoA            | `FC`      |       |       |       | **Y** |       |       |       |
//! | CenterPanadapterOnVfoB            | `FC$`     |       |       |       | **Y** |       |       |       |
//! | ClearRitOffset                    |           |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | CopyVfoAtoVfoB                    | `AB0`     |       |       |       | **Y** |       |       |       |
//! | DumpLog                           | `LG`      |       |       |       |       |       |       | **Y** |
//! | EmulateButtonHold                 | \[2]      |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | EmulateButtonTap                  | \[3]      |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | EmulateEncodeRotation             | `EN`      |       |       |       |       |       |       | **Y** |
//! | EmulateHandKeyPress               | `HK`      |       |       |       |       |       |       | **Y** |
//! | GetActiveSoftwareReleaseChannel   | `RL`      |       |       |       | **Y** |       |       |       |
//! | GetActualPowerOutput              |           |       |       |       |       | **Y** | **Y** |       |
//! | GetAgcTimeConstant                | `GT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetAntennaSelection               | `AN`      | **Y** |       |       |       |       |       |       |
//! | GetAtuMode                        | `AT`      |       |       |       | **Y** |       |       |       |
//! | GetAtuNetworkValues               | `AK`      |       |       |       |       | **Y** | **Y** |       |
//! | GetAudioLineInputLevel            | `LI`      |       |       |       | **Y** |       |       |       |
//! | GetAudioLineOutputLevel           | `LO`      |       |       |       | **Y** |       |       |       |
//! | GetAudioMixRatio                  | `MX`      |       |       |       | **Y** |       |       |       |
//! | GetAudioPeakingFilterState        | `AP`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetAutoInfoMode                   | `AI`      | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetBandIndependenceState          | `BI`      |       |       |       | **Y** |       |       |       |
//! | GetBargraphValue                  | `BG` \[6] | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetBufferedText                   | `TB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetCoarseTuningStep               | `VC`      |       |       |       | **Y** |       |       |       |
//! | GetCurrentBandPowerLimit          | `PP`      |       |       |       | **Y** |       |       |       |
//! | GetCwSidetonePitch                | `CW`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetDataSubMode                    | `DT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetDigitalAudioRoutingMode        | `DA`      |       |       |       | **Y** |       |       |       |
//! | GetDigitalOutputPin1State         | `DO`      |       |       |       | **Y** |       |       |       |
//! | GetDisplayText                    | `DS`      |       |       |       |       |       |       | **Y** |
//! | GetDiversityMode                  | `DV`      |       | **Y** | **Y** |       |       |       |       |
//! | GetErrorReportingState            | `ER`      |       |       |       | **Y** |       |       |       |
//! | GetEssbMode                       | `ES`      |       | **Y** | **Y** |       |       |       |       |
//! | GetFirmwareRevision               | `RV`      |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | GetHelpInformation                | `H`       |       |       |       |       |       |       | **Y** |
//! | GetHighResolutionSMeter           | `SMH`     |       | **Y** | **Y** |       |       |       |       |
//! | GetIfCenterFrequency              | `FI`      |       | **Y** |       |       |       |       |       |
//! | GetInstalledOptions               | `OM`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetK2CommandMode                  | `K2`      | **Y** |       |       |       |       |       |       |
//! | GetK3CommandMode                  | `K3`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetK3IconsAndStatus               | `IC`      |       | **Y** | **Y** |       |       |       |       |
//! | GetK4CommandMode                  | `K4`      |       |       |       | **Y** |       |       |       |
//! | GetKeyerPaddleEmulationMode       | `KP`      |       |       |       | **Y** |       |       |       |
//! | GetKeyerSpeed                     | `KS`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMemoryChannel                  | `MC`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMenuParameter                  | `MP` \[4] |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | GetMenuParameter16                | `MQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMenuParameter16                | `MQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMicGain                        | `MG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMicInputSource                 | `MI`      |       |       |       | **Y** |       |       |       |
//! | GetMonitorLevel                   | `ML`      |       | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | GetPowerStatus                    | `PS` \[5] |       | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | GetQskDelay                       | `SD`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetReceiveAntenna                 | `AR`      |       | **Y** | **Y** |       |       |       |       |
//! | GetReceiveVfo                     | `FR`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetRepeaterOffset                 | `RP`      |       |       |       | **Y** |       |       |       |
//! | GetRitControl                     | `RT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetRitXitOffset                   | `RO`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetScreenCount                    | `SC`      |       |       |       | **Y** |       |       |       |
//! | GetSpeechCompression              | `CP`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetStreamingLatencyClass          | `SL`      |       |       |       | **Y** |       |       |       |
//! | GetSubReceiver                    | `SB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransceiverId                  | `I`       |       |       |       | **Y** |       |       | **Y** |
//! | GetTransceiverInformation         | `IF`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransceiverSerialNumber        | `SN`      |       |       |       | **Y** |       |       | **Y** |
//! | GetTransceiverStatus              | `ST`      |       |       |       |       |       |       | **Y** |
//! | GetTransmitBufferedText           | `TBX`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransmitDataBandwidth          | `DW`      |       |       |       | **Y** |       |       |       |
//! | GetTransmitGain                   | `TG`      |       |       |       | **Y** |       |       |       |
//! | GetTransmitGainConstant           | `TA`      |       |       |       | **Y** |       |       |       |
//! | GetTransmitLowerLimit             | `TXL`     |       |       |       |       |       |       | **Y** |
//! | GetTransmitMeterMode              | `TM`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransmitPowerControl           | `PC`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransmitState                  | `TQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransmitTestModeState          | `TS`      |       |       |       | **Y** |       |       |       |
//! | GetTransmitUpperLimit             | `TXH`     |       |       |       |       |       |       | **Y** |
//! | GetTransmitVfoSplitModeState      | `FT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetUtcTimestamp                   | `UT`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAAfGain                     | `AG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAAgcMode                    | `GT`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAAutoNotchState             | `NA`      |       |       |       | **Y** |       |       |       |
//! | GetVfoABandNumber                 | `BN`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoACtssTone                   | `PL`      |       |       |       | **Y** |       |       |       |
//! | GetVfoADisplayAndIcons            | `DS` \[6] | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAFilterBandwidth            | `BW`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAFilterPresetSlot           | `FP`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAIfCenterPitch              | `IS`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAIfShift                    | `IS`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoALegacyFilterBandwidth      | `FW`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoALock                       | `LK`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAManualNotchSettings        | `NM`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAModeAlternates             | `MA`      |       |       |       | **Y** |       |       |       |
//! | GetVfoANoiseBlanker               | `NB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoANoiseBlankerLevel          | `NL`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoANoiseReductionSettings     | `NR`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAOperatingFrequency         | `FA`      | **Y** | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | GetVfoAOperatingMode              | `MD`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAPreamp                     | `PA`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAReceiveAttenuator          | `RA`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoARfGain                     | `RG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoASMeter                     | `SM`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoASquelch                    | `SQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoATextDecodeMode             | `TD`      |       |       |       | **Y** |       |       |       |
//! | GetVfoATransverterActiveBandSlot  | `XV`      |       |       |       | **Y** |       |       |       |
//! | GetVfoATransverterOffset          | `VO`      |       |       |       | **Y** |       |       |       |
//! | GetVfoATuningStep                 | `VT`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAXfilNumber                 | `XF`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBAfGain                     | `AG$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBAgcMode                    | `GT$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBAutoNotchState             | `NA$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBBandNumber                 | `BN$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBCtssTone                   | `PL$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBDisplayText                | `DB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBFilterBandwidth            | `BW$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBFilterPresetSlot           | `FP$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBIfCenterPitch              | `IS$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBIfShift                    | `IS$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBLegacyFilterBandwidth      | `FW$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBLock                       | `LK$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBLock                       | `LK$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBManualNotchSettings        | `NM$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBModeAlternates             | `MA$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBNoiseBlanker               | `NB$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBNoiseBlankerLevel          | `NL$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBNoiseReductionSettings     | `NR$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBOperatingFrequency         | `FA$`     | **Y** | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | GetVfoBOperatingMode              | `MD$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBPreamp                     | `PA$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBReceiveAttenuator          | `RA$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBRfGain                     | `RG$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBSMeter                     | `SM$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBSquelch                    | `SQ$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBTextDecodeMode             | `TD$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBTransverterActiveBandSlot  | `XV$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBTransverterOffset          | `VO$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBTuningStep                 | `VT$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBXfilNumber                 | `XF$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoLinkedState                 | `LN`      |       | **Y** |       |       |       |       |       |
//! | GetVox                            | `VX`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVoxGain                        | `VG`      |       |       |       | **Y** |       |       |       |
//! | GetVoxInhibitState                | `VI`      |       |       |       | **Y** |       |       |       |
//! | GetWattmeterCalibrationConstant   | `WM`      |       |       |       | **Y** |       |       |       |
//! | GetXitControl                     | `XT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GoToReceive                       | `RX`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GoToTransmit                      | `TX`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | LoadFirmware                      | `LD`      |       |       |       |       |       |       | **Y** |
//! | MoveRitOffsetDown                 | `RD`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveRitOffsetUp                   | `RU`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoAFrequencyDown             | `DN` \[6] | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoAFrequencyUp               | `UP`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoBFrequencyDown             | `DN$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoBFrequencyUp               | `UP$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | PlayDvrMessage                    | `PB`      |       |       |       | **Y** |       |       |       |
//! | SelectMenuItem                    | `MN` \[4] |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | SendCwText                        | `KY`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetActiveSoftwareReleaseChannel   | `RL`      |       |       |       | **Y** |       |       |       |
//! | SetAfGain                         | `AG` \[1] |       |       |       |       |       |       | **Y** |
//! | SetAgcTimeConstant                | `GT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetAtuMode                        | `AT`      |       |       |       | **Y** |       |       |       |
//! | SetAtuTuningState                 | `TU`      |       |       |       | **Y** |       |       |       |
//! | SetAudioLineInputLevel            | `LI`      |       |       |       | **Y** |       |       |       |
//! | SetAudioLineOutputLevel           | `LO`      |       |       |       | **Y** |       |       |       |
//! | SetAudioMixRatio                  | `MX`      |       |       |       | **Y** |       |       |       |
//! | SetAudioPeakingFilterState        | `AP`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetAutoInfoMode                   | `AI`      | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetBandIndependenceState          | `BI`      |       |       |       | **Y** |       |       |       |
//! | SetBaudRate                       | `BR`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetCoarseTuningStep               | `VC`      |       |       |       | **Y** |       |       |       |
//! | SetCommandEchoState               | `EC`      |       |       |       | **Y** |       |       |       |
//! | SetCommandProcessingDelay         | `DE`      |       | **Y** | **Y** |       |       |       |       |
//! | SetCwSidetonePitch                | `CW`      |       |       |       | **Y** |       |       |       |
//! | SetDataSubMode                    | `DT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetDigitalAudioRoutingMode        | `DA`      |       |       |       | **Y** |       |       |       |
//! | SetDigitalOutputPin1State         | `DO`      |       |       |       | **Y** |       |       |       |
//! | SetDisplayText                    | `DS`      |       |       |       |       |       |       | **Y** |
//! | SetDiversityMode                  | `DV`      |       | **Y** | **Y** |       |       |       |       |
//! | SetDspCommandDebugState           | `DL`      |       | **Y** | **Y** |       |       |       |       |
//! | SetErrorLogging                   | `EL`      |       |       |       |       | **Y** | **Y** |       |
//! | SetErrorReportingState            | `ER`      |       |       |       | **Y** |       |       |       |
//! | SetEssbMode                       | `ES`      |       | **Y** | **Y** |       |       |       |       |
//! | SetK2CommandMode                  | `K2`      | **Y** |       |       |       |       |       |       |
//! | SetK3CommandMode                  | `K3`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetK4CommandMode                  | `K4`      |       |       |       | **Y** |       |       |       |
//! | SetKeyerPaddleEmulationMode       | `KP`      |       |       |       | **Y** |       |       |       |
//! | SetKeyerSpeed                     | `KS`      |       |       |       | **Y** |       |       |       |
//! | SetKeyerSpeed                     | `KS`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetMemoryChannel                  | `MC`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetMenuParameter                  | `MP` \[4] |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | SetMicGain                        | `MG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetMicInputSource                 | `MI`      |       |       |       | **Y** |       |       |       |
//! | SetMonitorLevel                   | `ML`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetMonitorLevel                   | `ML`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetOperatingFrequency             | `FA`      |       |       |       |       |       |       | **Y** |
//! | SetOperatingMode                  | `MD`      |       |       |       |       |       |       | **Y** |
//! | SetPowerStatus                    | `PS`      |       |       |       | **Y** |       |       |       |
//! | SetPowerStatus                    | `PS`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetQskOrVoxDelay                  | `SD`      |       |       |       | **Y** |       |       |       |
//! | SetReceiveAntenna                 | `AR`      |       | **Y** | **Y** |       |       |       |       |
//! | SetReceiveVfo                     | `FR`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetRepeaterOffset                 | `RP`      |       |       |       | **Y** |       |       |       |
//! | SetRitControl                     | `RT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetRitXitOffset                   | `RO`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetSpeechCompression              | `CP`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetStreamingLatencyClass          | `SL`      |       |       |       | **Y** |       |       |       |
//! | SetSubReceiver                    | `SB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetSystemAutoInfoInterval         | `SI`      |       |       |       | **Y** |       |       |       |
//! | SetTextToTerminal                 | `TT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetTransmitDataBandwidth          | `DW`      |       |       |       | **Y** |       |       |       |
//! | SetTransmitEqualizer              | `TE`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetTransmitMeterMode              | `TM`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetTransmitPowerControl           | `PC`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetTransmitTestModeState          | `TS`      |       |       |       | **Y** |       |       |       |
//! | SetTransmitVfoSplitModeState      | `FT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAAfGain                     | `AG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAAgcMode                    | `GT`      |       |       |       | **Y** |       |       |       |
//! | SetVfoAAutoNotchState             | `NA`      |       |       |       | **Y** |       |       |       |
//! | SetVfoABandNumber                 | `BN`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoABandNumber                 | `BN`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoACtssTone                   | `PL`      |       |       |       | **Y** |       |       |       |
//! | SetVfoAFilterBandwidth            | `BW`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAFilterPresetSlot           | `FP`      |       |       |       | **Y** |       |       |       |
//! | SetVfoAIfShift                    | `IS`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoALegacyFilterBandwidth      | `FW`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoALock                       | `LK`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoALock                       | `LK`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAManualNotchSettings        | `NM`      |       |       |       | **Y** |       |       |       |
//! | SetVfoANoiseBlanker               | `NB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoANoiseBlankerLevel          | `NL`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoANoiseReductionSettings     | `NR`      |       |       |       | **Y** |       |       |       |
//! | SetVfoAOperatingFrequency         | `FA`      | **Y** | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | SetVfoAOperatingMode              | `MD`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAPreamp                     | `PA`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoAReceiveAttenuator          | `RA`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoARfGain                     | `RG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoASquelch                    | `SQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoATextDecodeMode             | `TD`      |       |       |       | **Y** |       |       |       |
//! | SetVfoATransverterActiveBandSlot  | `XV`      |       |       |       | **Y** |       |       |       |
//! | SetVfoATuningStep                 | `VT`      |       |       |       | **Y** |       |       |       |
//! | SetVfoBAfGain                     | `AG$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBAgcMode                    | `GT$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoBAutoNotchState             | `NA$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoBBandNumber                 | `BN$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBBandNumber                 | `BN$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBCtssTone                   | `PL$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoBDisplayText                | `DB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBFilterBandwidth            | `BW$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBFilterPresetSlot           | `FP$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoBIfShift                    | `IS$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBLegacyFilterBandwidth      | `FW$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBLock                       | `LK$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBLock                       | `LK$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBManualNotchSettings        | `NM$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoBNoiseBlanker               | `NB$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBNoiseBlankerLevel          | `NL$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBNoiseReductionSettings     | `NR$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoBOperatingFrequency         | `FA$`     | **Y** | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | SetVfoBOperatingMode              | `MD$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBPreamp                     | `PA$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBReceiveAttenuator          | `RA$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBRfGain                     | `RG$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBSquelch                    | `SQ$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVfoBTextDecodeMode             | `TD`      |       |       |       | **Y** |       |       |       |
//! | SetVfoBTextDecodeMode             | `TD$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoBTransverterActiveBandSlot  | `XV$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoBTuningStep                 | `VT$`     |       |       |       | **Y** |       |       |       |
//! | SetVfoLinkedState                 | `LN`      |       | **Y** |       |       |       |       |       |
//! | SetVfoOffset                      | `FO`      |       |       |       |       |       |       | **Y** |
//! | SetVox                            | `VX`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetVoxGain                        | `VG`      |       |       |       | **Y** |       |       |       |
//! | SetVoxInhibitState                | `VI`      |       |       |       | **Y** |       |       |       |
//! | SetWattmeterCalibrationConstant   | `WM`      |       |       |       | **Y** |       |       |       |
//! | SetXitControl                     | `XT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SwapVfoAandVfoB                   | `AB1`     |       |       |       | **Y** |       |       |       |
//!
//! ## Notes
//!
//! 1. For the KH1 the command ID is `AG` which is the same as the VFO A commands for dual VFO
//!    transceivers.
//! 2. For the KH1 the command ID is `SW{n}H`, for K3, K#S, KX2, and KX3 it is `SWH`.
//! 3. For the KH1 the command ID is `SW{n}T`, for K3, K#S, KX2, and KX3 it is `SWT`.
//! 4. While the command ID is the same, the different families have different argument tyes as well
//!    as different valid values for the arguments.
//! 5. The K2/K3/KX families only support on/off while the K4 supports an additional firmware
//!    restart state.
//! 6. The K2 command qrguments and responses are sub-sets of the K3/K4/KX families.
//!
//! # Amplifiers
//!
//! | Command                       | ID     | KPA1500 | KPA500 | KXPA100 |
//! |-------------------------------|--------|:-------:|:------:|:-------:|
//!
//!
//! # Panadapters
//!
//! | Command                           | ID        | P3      | PX3    |
//! |-----------------------------------|-----------|:-------:|:------:|
//! | ExecuteFunctionKey                | `#FNX`    | **Y**   | Y      |
//! | GetBaudRate                       | `#BR`     | **Y**   | Y      |
//! | GetBeaconModeState                | `#BCN`    |         | **Y**  |
//! | GetBeaconTextMemoryLocation       | `#BCL`    |         | **Y**  |
//! | GetBeaconTransmissionInterval     | `#BCI`    |         | **Y**  |
//! | GetCalibrationSignalState         | `#CAL`    |         | **Y**  |
//! | GetCenterFrequency                | `#CTF`    | **Y**   | Y      |
//! | GetDisplayAveragingTimeConstant   | `#AVG`    | **Y**   | Y      |
//! | GetDisplayFontSize                | `#FON`    | **Y**   |        |
//! | GetDisplayMode                    | `#DSM`    |         | **Y**  |
//! | GetDisplayMode                    | `#DSM`    | **Y**   |        |
//! | GetFirmwareRevision               | `#RVM`    | **Y**   |        |
//! | GetFixedTuneAutoAdjustMode        | `#FXA`    | **Y**   | Y      |
//! | GetFixedTuneOrTrackingMode        | `#FXT`    | **Y**   |        |
//! | GetFpgaImageFirmwareRevision      | `#RVF`    | **Y**   |        |
//! | GetFunctionKeyLabel               | `#FNL`    | **Y**   | Y      |
//! | GetFunctionKeyLabelDisplayState   | `#LBL`    |         | **Y**  |
//! | GetFunctionKeyLabelDisplayState   | `#LBL`    | **Y**   |        |
//! | GetMarkerAFrequency               | `#MFA`    | **Y**   |        |
//! | GetMarkerAState                   | `#MKA`    | **Y**   |        |
//! | GetMarkerBFrequency               | `#MFB`    | **Y**   |        |
//! | GetMarkerBState                   | `#MKB`    | **Y**   |        |
//! | GetNoiseBlankerLevel              | `#NBL`    | **Y**   | Y      |
//! | GetNoiseBlankerState              | `#NB`     | **Y**   | Y      |
//! | GetOppositeSideBandNullAmplitude  | `#OSBA`   |         | **Y**  |
//! | GetOppositeSideBandNullPhase      | `#OSBP`   |         | **Y**  |
//! | GetPeakModeState                  | `#PKM`    | **Y**   | Y      |
//! | GetPowerStatus                    | `#PS`     | **Y**   | Y      |
//! | GetProductId                      | `=`       | **Y**   | Y      |
//! | GetReferenceLevel                 | `#REF`    | **Y**   | Y      |
//! | GetRelativeCenterFrequency        | `#RCF`    | **Y**   | Y      |
//! | GetScale                          | `#SCL`    | **Y**   | Y      |
//! | GetSpan                           | `#SPN`    | **Y**   | Y      |
//! | GetSpanMode                       | `#SPM`    | **Y**   |        |
//! | GetSvgaDecodedDataDisplayState    | `#SVDT`   | **Y**   |        |
//! | GetSvgaDisplayResolution          | `#SVRS`   | **Y**   |        |
//! | GetSvgaDisplayState               | `#SVEN`   | **Y**   |        |
//! | GetSvgaFirmwareRevision           | `#RVS`    | **Y**   |        |
//! | GetSvgaFontSize                   | `#SVFN`   | **Y**   |        |
//! | GetSvgaSpectrumFillState          | `#SVFL`   | **Y**   |        |
//! | GetSvgaWaterfallBias              | `#SVWB`   | **Y**   |        |
//! | GetTextHangTime                   | `#TXH`    |         | **Y**  |
//! | GetTextTransmitMode               | `#TXM`    |         | **Y**  |
//! | GetTransceiverConnected           | `#XCV`    | **Y**   |        |
//! | GetUsbKeyboardDetectedState       | `#USB`    |         | **Y**  |
//! | GetVfoBCursorState                | `#VFB`    | **Y**   | Y      |
//! | GetWaterfallAveragingState        | `#WFA`    | **Y**   |        |
//! | GetWaterfallColor                 | `#WFC`    | **Y**   |        |
//! | GetWaterfallMarkersState          | `#WFM`    | **Y**   |        |
//! | MoveMarkerAFrequency              | `#MAA`    |         | **Y**  |
//! | MoveMarkerBFrequency              | `#MBA`    |         | **Y**  |
//! | Reset                             | `#RST`    | **Y**   |        |
//! | SaveScreenshotToFlashDrive        | `#MSS`    |         | **Y**  |
//! | SetBaudRate                       | `#BR`     | **Y**   | Y      |
//! | SetBeaconModeState                | `#BCN`    | **Y**   |        |
//! | SetBeaconTextMemoryLocation       | `#BCL`    | **Y**   |        |
//! | SetBeaconTransmissionInterval     | `#BCI`    | **Y**   |        |
//! | SetCalibrationSignalState         | `#CAL`    | **Y**   |        |
//! | SetCenterFrequency                | `#CTF`    | **Y**   | Y      |
//! | SetDisplayAveragingTimeConstant   | `#AVG`    | **Y**   | Y      |
//! | SetDisplayFontSize                | `#FON`    | **Y**   |        |
//! | SetDisplayMode                    | `#DSM`    | **Y**   |        |
//! | SetDisplayMode                    | `#DSM`    | **Y**   |        |
//! | SetFixedTuneAutoAdjustMode        | `#FXA`    | **Y**   | Y      |
//! | SetFixedTuneOrTrackingMode        | `#FXT`    | **Y**   |        |
//! | SetFunctionKeyLabelDisplayState   | `#LBL`    | **Y**   |        |
//! | SetMarkerAFrequency               | `#MFA`    | **Y**   |        |
//! | SetMarkerAState                   | `#MKA`    | **Y**   |        |
//! | SetMarkerBFrequency               | `#MFB`    | **Y**   |        |
//! | SetMarkerBState                   | `#MKB`    | **Y**   |        |
//! | SetNoiseBlankerLevel              | `#NBL`    | **Y**   | Y      |
//! | SetNoiseBlankerState              | `#NB`     | **Y**   | Y      |
//! | SetOppositeSideBandNullAmplitude  | `#OSBA`   | **Y**   |        |
//! | SetOppositeSideBandNullPhase      | `#OSBP`   | **Y**   |        |
//! | SetPassThroughModeState           | `#PT`     | **Y**   | Y      |
//! | SetPeakModeState                  | `#PKM`    | **Y**   | Y      |
//! | SetPowerStatus                    | `#PS`     | **Y**   | Y      |
//! | SetQsyToMarker                    | `#QSY`    | **Y**   | Y      |
//! | SetReferenceLevel                 | `#REF`    | **Y**   | Y      |
//! | SetRelativeCenterFrequency        | `#RCF`    | **Y**   | Y      |
//! | SetScale                          | `#SCL`    | **Y**   | Y      |
//! | SetSpan                           | `#SPN`    | **Y**   | Y      |
//! | SetSpanMode                       | `#SPM`    | **Y**   |        |
//! | SetSvgaDecodedDataDisplayState    | `#SVDT`   | **Y**   |        |
//! | SetSvgaDisplayResolution          | `#SVRS`   | **Y**   |        |
//! | SetSvgaDisplayState               | `#SVEN`   | **Y**   |        |
//! | SetSvgaFontSize                   | `#SVFN`   | **Y**   |        |
//! | SetSvgaSpectrumFillState          | `#SVFL`   | **Y**   |        |
//! | SetSvgaWaterfallBias              | `#SVWB`   | **Y**   |        |
//! | SetTextHangTime                   | `#TXH`    | **Y**   |        |
//! | SetTextTransmitMode               | `#TXM`    | **Y**   |        |
//! | SetTransceiverConnected           | `#XCV`    | **Y**   |        |
//! | SetVfoBCursorState                | `#VFB`    | **Y**   | Y      |
//! | SetWaterfallAveragingState        | `#WFA`    | **Y**   |        |
//! | SetWaterfallColor                 | `#WFC`    | **Y**   |        |
//! | SetWaterfallMarkersState          | `#WFM`    | **Y**   |        |
//! | UploadScreenshotBitmap            | `#BMP`    | **Y**   | Y      |
//!
//! # Tuners
//!
//! Only supports the KAT500 Automatic Antenna Tuner.
//!
//! * AntennaSideIter; An iterator over the variants of AntennaSide
//! * Bypass; Force the ATU into bypass mode immediately.
//! * ClearCurrentFault; Clear the current fault condition.
//! * EepromInit; Re-initialize EEPROM storage to factory defaults.
//! * GetAmplifierInterface; Get the amplifier interface relay state.
//! * GetAntenna; Get the currently selected antenna port.
//! * GetAntennaSide; Get the antenna side selection.
//! * GetAttenuatorState; Get whether the built-in attenuator is enabled.
//! * GetAtuFaultState; Get whether the ATU currently has a fault condition.
//! * GetAtuKeepInPlaceState; Get the ATU keep-in-place state.
//! * GetAtuPreset; Get the current ATU preset slot number.
//! * GetAutoBypassState; Get whether automatic bypass is enabled.
//! * GetAutoEnableState; Get whether the ATU is enabled, i.e. whether automatic tuning is allowed.
//! * GetBand; Get the current band number.
//! * GetBaudRate; Get the serial port baud rate.
//! * GetCapacitorTopology; Get the tuning capacitor topology (hi-Z or lo-Z).
//! * GetCapacitorValue; Get the tuning capacitor value.
//! * GetDemoModeState; Get the demo-mode state.
//! * GetErrorMessage; Get the last error message string.
//! * GetFanThreshold; Get the fan-on power threshold.
//! * GetFaultDelayTime; Get the fault delay time.
//! * GetFaultStatus; Get the current fault status code.
//! * GetFaultThresholdHigh; Get the upper fault SWR threshold.
//! * GetFaultThresholdLow; Get the lower fault SWR threshold.
//! * GetFirmwareVersion; Get the firmware version string.
//! * GetFixedBypassState; Get whether fixed bypass mode is active.
//! * GetFixedLcState; Get whether fixed L/C mode is enabled.
//! * GetForwardPowerA; Get the forward power reading on meter channel A.
//! * GetForwardPowerB; Get the forward power reading on meter channel B.
//! * GetForwardVoltage; Get the ADC forward voltage reading.
//! * GetFrequency; Get the operating frequency, in Hz.
//! * GetInductance; Get the tuning inductance tap.
//! * GetInductorSwitch; Get the inductor switch bitmask.
//! * GetInhibitFan; Get whether the cooling fan is inhibited.
//! * GetMeterType; Get the front-panel meter display type.
//! * GetOperatingMode; Get the current ATU operating mode.
//! * GetPowerSensorInput; Get the forward power reading from the internal sensor.
//! * GetPowerStatus; Get the power-on status.
//! * GetReflectedVoltage; Get the ADC reflected voltage reading.
//! * GetSerialNumber; Get the unit serial number.
//! * GetSwr; Get the computed standing wave ratio.
//! * GetSwrBypassThreshold; Get the SWR threshold above which bypass is engaged.
//! * GetSwrMeter; Get the current SWR meter reading.
//! * GetTunePower; Get the RF power level used during a tune cycle.
//! * GetTuneSatisfiedSwr; Get the SWR threshold below which a tune cycle is considered successful.
//! * GetTuneState; Get whether a tuning cycle is currently in progress.
//! * GetTuningSpeedLimit; Get the tuning speed limit setting.
//! * MeterTypeIter; An iterator over the variants of MeterType
//! * OperatingModeIter; An iterator over the variants of OperatingMode
//! * ResetDevice; Perform a soft reset of the KAT500, triggering a firmware restart.
//! * SetAmplifierInterface; Set the amplifier interface relay state.
//! * SetAntenna; Set the currently selected antenna port.
//! * SetAntennaSide; Set the antenna side selection.
//! * SetAttenuatorState; Set whether the built-in attenuator is enabled.
//! * SetAtuKeepInPlaceState; Set the ATU keep-in-place state.
//! * SetAtuPreset; Set the current ATU preset slot number.
//! * SetAutoBypassState; Set whether automatic bypass is enabled.
//! * SetAutoEnableState; Set whether the ATU is enabled, i.e. whether automatic tuning is allowed.
//! * SetBand; Set the current band number.
//! * SetBaudRate; Set the serial port baud rate.
//! * SetCapacitorTopology; Set the tuning capacitor topology (hi-Z or lo-Z).
//! * SetCapacitorValue; Set the tuning capacitor value.
//! * SetDemoModeState; Set the demo-mode state.
//! * SetFanThreshold; Set the fan-on power threshold.
//! * SetFaultDelayTime; Set the fault delay time.
//! * SetFaultThresholdHigh; Set the upper fault SWR threshold.
//! * SetFaultThresholdLow; Set the lower fault SWR threshold.
//! * SetFixedBypassState; Set whether fixed bypass mode is active.
//! * SetFixedLcState; Set whether fixed L/C mode is enabled.
//! * SetFrequency; Set the operating frequency, in Hz.
//! * SetInductance; Set the tuning inductance tap.
//! * SetInductorSwitch; Set the inductor switch bitmask directly.
//! * SetInhibitFan; Set whether the cooling fan is inhibited.
//! * SetMeterType; Set the front-panel meter display type.
//! * SetOperatingMode; Set the current ATU operating mode.
//! * SetSwrBypassThreshold; Set the SWR threshold above which bypass is engaged.
//! * SetTunePower; Set the RF power level used during a tune cycle.
//! * SetTuneSatisfiedSwr; Set the SWR threshold below which a tune cycle is considered successful.
//! * SetTuningSpeedLimit; Set the tuning speed limit.
//! * StartTune
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

use core::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

define_command_enum!(
    "Identifies the VFO to which a command applies." => Vfo {
    "VFO-A, or primary." => A = b'0',
    "VFO-B, sometimes refers to a sub-receiver." => B = b'1'
});

impl Display for Vfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => "VFO-A",
            Self::B => "VFO-B",
        }
        .fmt(f)
    }
}

// ------------------------------------------------------------------------------------------------
// Transceiver Sub-Modules
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "k2-kio2")]
pub mod k2;

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
