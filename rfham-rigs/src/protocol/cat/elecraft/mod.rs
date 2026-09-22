//!
//! Provides commands for Elecraft products, covering the K and KX series transceivers, KPA and KXPA
//! amplifiers, KAT500 tuner, and KP and KXP panadapters.
//!
//! # Transceivers
//!
//! | Command                               | ID        | K2    | K3    | K3S   | K4    | KX2   | KX3   | KH1   |
//! |---------------------------------------|-----------|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|
//! | CaptureScreenshot                     | `SS`      |       |       |       | **Y** |       |       |       |
//! | CenterPanadapterOnVfoA                | `FC`      |       |       |       | **Y** |       |       |       |
//! | CenterPanadapterOnVfoB                | `FC$`     |       |       |       | **Y** |       |       |       |
//! | ClearRitOffset                        |           |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | CopyVfoAtoVfoB                        | `AB0`     |       |       |       | **Y** |       |       |       |
//! | DumpLog                               | `LG`      |       |       |       |       |       |       | **Y** |
//! | EmulateButtonHold                     | \[2]      |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | EmulateButtonTap                      | \[3]      |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | EmulateEncodeRotation                 | `EN`      |       |       |       |       |       |       | **Y** |
//! | EmulateHandKeyPress                   | `HK`      |       |       |       |       |       |       | **Y** |
//! | Get/Set ActiveSoftwareReleaseChannel  | `RL`      |       |       |       | **Y** |       |       |       |
//! | Get/Set AgcTimeConstant               | `GT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set AtuMode                       | `AT`      |       |       |       | **Y** |       |       |       |
//! | Get/Set AudioLineInputLevel           | `LI`      |       |       |       | **Y** |       |       |       |
//! | Get/Set AudioLineOutputLevel          | `LO`      |       |       |       | **Y** |       |       |       |
//! | Get/Set AudioMixRatio                 | `MX`      |       |       |       | **Y** |       |       |       |
//! | Get/Set AudioPeakingFilterState       | `AP`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set AutoInfoMode                  | `AI`      | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set BandIndependenceState         | `BI`      |       |       |       | **Y** |       |       |       |
//! | Get/Set CoarseTuningStep              | `VC`      |       |       |       | **Y** |       |       |       |
//! | Get/Set CwSidetonePitch               | `CW`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set DataSubMode                   | `DT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set DigitalAudioRoutingMode       | `DA`      |       |       |       | **Y** |       |       |       |
//! | Get/Set DigitalOutputPin1State        | `DO`      |       |       |       | **Y** |       |       |       |
//! | Get/Set DisplayText                   | `DS`      |       |       |       |       |       |       | **Y** |
//! | Get/Set DiversityMode                 | `DV`      |       | **Y** | **Y** |       |       |       |       |
//! | Get/Set ErrorReportingState           | `ER`      |       |       |       | **Y** |       |       |       |
//! | Get/Set EssbMode                      | `ES`      |       | **Y** | **Y** |       |       |       |       |
//! | Get/Set K2CommandMode                 | `K2`      | **Y** |       |       |       |       |       |       |
//! | Get/Set K3CommandMode                 | `K3`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set K4CommandMode                 | `K4`      |       |       |       | **Y** |       |       |       |
//! | Get/Set KeyerPaddleEmulationMode      | `KP`      |       |       |       | **Y** |       |       |       |
//! | Get/Set MemoryChannel                 | `MC`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set MicGain                       | `MG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set MicInputSource                | `MI`      |       |       |       | **Y** |       |       |       |
//! | Get/Set MonitorLevel                  | `ML`      |       | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | Get/Set PowerStatus                   | `PS` \[5] |       | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | Get/Set ReceiveAntenna                | `AR`      |       | **Y** | **Y** |       |       |       |       |
//! | Get/Set ReceiveVfo                    | `FR`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set RepeaterOffset                | `RP`      |       |       |       | **Y** |       |       |       |
//! | Get/Set RitControl                    | `RT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set RitXitOffset                  | `RO`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set SpeechCompression             | `CP`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set StreamingLatencyClass         | `SL`      |       |       |       | **Y** |       |       |       |
//! | Get/Set SubReceiver                   | `SB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set TransmitDataBandwidth         | `DW`      |       |       |       | **Y** |       |       |       |
//! | Get/Set TransmitMeterMode             | `TM`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set TransmitPowerControl          | `PC`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set TransmitTestModeState         | `TS`      |       |       |       | **Y** |       |       |       |
//! | Get/Set TransmitVfoSplitModeState     | `FT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoAAfGain                    | `AG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoAAgcMode                   | `GT`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoAAutoNotchState            | `NA`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoABandNumber                | `BN`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoACtssTone                  | `PL`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoAFilterBandwidth           | `BW`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoAFilterPresetSlot          | `FP`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoAIfShift                   | `IS`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoALegacyFilterBandwidth     | `FW`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoALock                      | `LK`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoAManualNotchSettings       | `NM`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoANoiseBlanker              | `NB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoANoiseBlankerLevel         | `NL`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoANoiseReductionSettings    | `NR`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoAOperatingFrequency        | `FA`      | **Y** | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | Get/Set VfoAOperatingMode             | `MD`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoAPreamp                    | `PA`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoAReceiveAttenuator         | `RA`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoARfGain                    | `RG`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoASquelch                   | `SQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoATextDecodeMode            | `TD`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoATransverterActiveBandSlot | `XV`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoATuningStep                | `VT`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBAfGain                    | `AG$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBAgcMode                   | `GT$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBAutoNotchState            | `NA$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBBandNumber                | `BN$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBCtssTone                  | `PL$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBDisplayText               | `DB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBFilterBandwidth           | `BW$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBFilterPresetSlot          | `FP$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBIfShift                   | `IS$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBLegacyFilterBandwidth     | `FW$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBLock                      | `LK$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBManualNotchSettings       | `NM$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBNoiseBlanker              | `NB$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBNoiseBlankerLevel         | `NL$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBNoiseReductionSettings    | `NR$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBOperatingFrequency        | `FA$`     | **Y** | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | Get/Set VfoBOperatingMode             | `MD$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBPreamp                    | `PA$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBReceiveAttenuator         | `RA$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBRfGain                    | `RG$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBSquelch                   | `SQ$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VfoBTextDecodeMode            | `TD$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBTransverterActiveBandSlot | `XV$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoBTuningStep                | `VT$`     |       |       |       | **Y** |       |       |       |
//! | Get/Set VfoLinkedState                | `LN`      |       | **Y** |       |       |       |       |       |
//! | Get/Set Vox                           | `VX`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | Get/Set VoxGain                       | `VG`      |       |       |       | **Y** |       |       |       |
//! | Get/Set VoxInhibitState               | `VI`      |       |       |       | **Y** |       |       |       |
//! | Get/Set WattmeterCalibrationConstant  | `WM`      |       |       |       | **Y** |       |       |       |
//! | Get/Set XitControl                    | `XT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetActualPowerOutput                  |           |       |       |       |       | **Y** | **Y** |       |
//! | GetAntennaSelection                   | `AN`      | **Y** |       |       |       |       |       |       |
//! | GetAtuNetworkValues                   | `AK`      |       |       |       |       | **Y** | **Y** |       |
//! | GetBargraphValue                      | `BG` \[6] | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetBufferedText                       | `TB`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetCurrentBandPowerLimit              | `PP`      |       |       |       | **Y** |       |       |       |
//! | GetFirmwareRevision                   | `RV`      |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | GetHelpInformation                    | `H`       |       |       |       |       |       |       | **Y** |
//! | GetHighResolutionSMeter               | `SMH`     |       | **Y** | **Y** |       |       |       |       |
//! | GetIfCenterFrequency                  | `FI`      |       | **Y** |       |       |       |       |       |
//! | GetInstalledOptions                   | `OM`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetK3IconsAndStatus                   | `IC`      |       | **Y** | **Y** |       |       |       |       |
//! | GetKeyerSpeed                         | `KS`      |       | **Y** | **Y** |   Y   | **Y** | **Y** |       |
//! | GetMenuParameter                      | `MP` \[4] |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | GetMenuParameter16                    | `MQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetMenuParameter16                    | `MQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetQskDelay                           | `SD`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetScreenCount                        | `SC`      |       |       |       | **Y** |       |       |       |
//! | GetTransceiverId                      | `I`       |       |       |       | **Y** |       |       | **Y** |
//! | GetTransceiverInformation             | `IF`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransceiverSerialNumber            | `SN`      |       |       |       | **Y** |       |       | **Y** |
//! | GetTransceiverStatus                  | `ST`      |       |       |       |       |       |       | **Y** |
//! | GetTransmitBufferedText               | `TBX`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransmitGain                       | `TG`      |       |       |       | **Y** |       |       |       |
//! | GetTransmitGainConstant               | `TA`      |       |       |       | **Y** |       |       |       |
//! | GetTransmitLowerLimit                 | `TXL`     |       |       |       |       |       |       | **Y** |
//! | GetTransmitState                      | `TQ`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetTransmitUpperLimit                 | `TXH`     |       |       |       |       |       |       | **Y** |
//! | GetUtcTimestamp                       | `UT`      |       |       |       | **Y** |       |       |       |
//! | GetVfoADisplayAndIcons                | `DS` \[6] | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoAIfCenterPitch                  | `IS`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAModeAlternates                 | `MA`      |       |       |       | **Y** |       |       |       |
//! | GetVfoASMeter                         | `SM`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoATransverterOffset              | `VO`      |       |       |       | **Y** |       |       |       |
//! | GetVfoAXfilNumber                     | `XF`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBIfCenterPitch                  | `IS$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBModeAlternates                 | `MA$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBSMeter                         | `SM$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GetVfoBTransverterOffset              | `VO$`     |       |       |       | **Y** |       |       |       |
//! | GetVfoBXfilNumber                     | `XF$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GoToReceive                           | `RX`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | GoToTransmit                          | `TX`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | LoadFirmware                          | `LD`      |       |       |       |       |       |       | **Y** |
//! | MoveRitOffsetDown                     | `RD`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveRitOffsetUp                       | `RU`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoAFrequencyDown                 | `DN` \[6] | **Y** | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoAFrequencyUp                   | `UP`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoBFrequencyDown                 | `DN$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | MoveVfoBFrequencyUp                   | `UP$`     |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | PlayDvrMessage                        | `PB`      |       |       |       | **Y** |       |       |       |
//! | SelectMenuItem                        | `MN` \[4] |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | SendCwText                            | `KY`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetAfGain                             | `AG` \[1] |       |       |       |       |       |       | **Y** |
//! | SetAtuTuningState                     | `TU`      |       |       |       | **Y** |       |       |       |
//! | SetBaudRate                           | `BR`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetCommandEchoState                   | `EC`      |       |       |       | **Y** |       |       |       |
//! | SetCommandProcessingDelay             | `DE`      |       | **Y** | **Y** |       |       |       |       |
//! | SetDspCommandDebugState               | `DL`      |       | **Y** | **Y** |       |       |       |       |
//! | SetErrorLogging                       | `EL`      |       |       |       |       | **Y** | **Y** |       |
//! | SetK2CommandMode                      | `K2`      | **Y** |       |       |       |       |       |       |
//! | SetK3CommandMode                      | `K3`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetK4CommandMode                      | `K4`      |       |       |       | **Y** |       |       |       |
//! | SetKeyerSpeed                         | `KS`      |       | **Y** | **Y** | **Y** | **Y** | **Y** |       |
//! | SetMenuParameter                      | `MP` \[4] |       | **Y** | **Y** |       | **Y** | **Y** | **Y** |
//! | SetOperatingFrequency                 | `FA`      |       |       |       |       |       |       | **Y** |
//! | SetOperatingMode                      | `MD`      |       |       |       |       |       |       | **Y** |
//! | SetQskOrVoxDelay                      | `SD`      |       |       |       | **Y** |       |       |       |
//! | SetSystemAutoInfoInterval             | `SI`      |       |       |       | **Y** |       |       |       |
//! | SetTextToTerminal                     | `TT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetTransmitEqualizer                  | `TE`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetTransmitVfoSplitModeState          | `FT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SetXitControl                         | `XT`      |       | **Y** | **Y** |       | **Y** | **Y** |       |
//! | SwapVfoAandVfoB                       | `AB1`     |       |       |       | **Y** |       |       |       |
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
//! | Command                                       | ID     | KPA1500 | KPA500 | KXPA100 |
//! |-----------------------------------------------|--------|:-------:|:------:|:-------:|
//!
//!
//! # Panadapters
//!
//! In the PX3 column, those commands implemented by the P3 and usable as-is are marked with a 'Y'.
//! If the command is unique to thePX3, or is significantly different from the P3 implementation, it
//! is marked with a bold '**Y**'.
//!
//! | Command                                       | ID        | P3      | PX3    |
//! |-----------------------------------------------|-----------|:-------:|:------:|
//! | ExecuteFunctionKey                            | `#FNX`    | **Y**   | Y      |
//! | Get/Set BaudRate                              | `#BR`     | **Y**   | Y      |
//! | Get/Set BeaconModeState                       | `#BCN`    |         | **Y**  |
//! | Get/Set BeaconTextMemoryLocation              | `#BCL`    |         | **Y**  |
//! | Get/Set BeaconTransmissionInterval            | `#BCI`    |         | **Y**  |
//! | Get/Set CalibrationSignalState                | `#CAL`    |         | **Y**  |
//! | Get/Set CenterFrequency                       | `#CTF`    | **Y**   | Y      |
//! | Get/Set DisplayAveragingTimeConstant          | `#AVG`    | **Y**   | Y      |
//! | Get/Set DisplayFontSize                       | `#FON`    | **Y**   |        |
//! | Get/Set DisplayMode                           | `#DSM`    | **Y**   | **Y**  |
//! | Get/Set FixedTuneAutoAdjustMode               | `#FXA`    | **Y**   | Y      |
//! | Get/Set FixedTuneOrTrackingMode               | `#FXT`    | **Y**   |        |
//! | Get/Set FunctionKeyLabelDisplayState          | `#LBL`    | **Y**   | **Y**  |
//! | Get/Set MarkerAFrequency                      | `#MFA`    | **Y**   |        |
//! | Get/Set MarkerAState                          | `#MKA`    | **Y**   |        |
//! | Get/Set MarkerBFrequency                      | `#MFB`    | **Y**   |        |
//! | Get/Set MarkerBState                          | `#MKB`    | **Y**   |        |
//! | Get/Set NoiseBlankerLevel                     | `#NBL`    | **Y**   | Y      |
//! | Get/Set NoiseBlankerState                     | `#NB`     | **Y**   | Y      |
//! | Get/Set OppositeSideBandNullAmplitude         | `#OSBA`   |         | **Y**  |
//! | Get/Set OppositeSideBandNullPhase             | `#OSBP`   |         | **Y**  |
//! | Get/Set PeakModeState                         | `#PKM`    | **Y**   | Y      |
//! | Get/Set PowerStatus                           | `#PS`     | **Y**   | Y      |
//! | Get/Set ReferenceLevel                        | `#REF`    | **Y**   | Y      |
//! | Get/Set RelativeCenterFrequency               | `#RCF`    | **Y**   | Y      |
//! | Get/Set Scale                                 | `#SCL`    | **Y**   | Y      |
//! | Get/Set Span                                  | `#SPN`    | **Y**   | Y      |
//! | Get/Set SpanMode                              | `#SPM`    | **Y**   |        |
//! | Get/Set SvgaDecodedDataDisplayState           | `#SVDT`   | **Y**   |        |
//! | Get/Set SvgaDisplayResolution                 | `#SVRS`   | **Y**   |        |
//! | Get/Set SvgaDisplayState                      | `#SVEN`   | **Y**   |        |
//! | Get/Set SvgaFontSize                          | `#SVFN`   | **Y**   |        |
//! | Get/Set SvgaSpectrumFillState                 | `#SVFL`   | **Y**   |        |
//! | Get/Set SvgaWaterfallBias                     | `#SVWB`   | **Y**   |        |
//! | Get/Set TextHangTime                          | `#TXH`    | **Y**   |        |
//! | Get/Set TextTransmitMode                      | `#TXM`    | **Y**   |
//! | Get/Set TransceiverConnected                  | `#XCV`    | **Y**   |        |
//! | Get/Set VfoBCursorState                       | `#VFB`    | **Y**   | Y      |
//! | Get/Set WaterfallAveragingState               | `#WFA`    | **Y**   |        |
//! | Get/Set WaterfallColor                        | `#WFC`    | **Y**   |        |
//! | Get/Set WaterfallMarkersState                 | `#WFM`    | **Y**   |        |
//! | GetFirmwareRevision                           | `#RVM`    | **Y**   |        |
//! | GetFpgaImageFirmwareRevision                  | `#RVF`    | **Y**   |        |
//! | GetFunctionKeyLabel                           | `#FNL`    | **Y**   | Y      |
//! | GetProductId                                  | `=`       | **Y**   | Y      |
//! | GetSvgaFirmwareRevision                       | `#RVS`    | **Y**   |        |
//! | GetUsbKeyboardDetectedState                   | `#USB`    |         | **Y**  |
//! | MoveMarkerAFrequency                          | `#MAA`    |         | **Y**  |
//! | MoveMarkerBFrequency                          | `#MBA`    |         | **Y**  |
//! | Reset                                         | `#RST`    | **Y**   |        |
//! | SaveScreenshotToFlashDrive                    | `#MSS`    |         | **Y**  |
//! | SetPassThroughModeState                       | `#PT`     | **Y**   | Y      |
//! | SetQsyToMarker                                | `#QSY`    | **Y**   | Y      |
//! | UploadScreenshotBitmap                        | `#BMP`    | **Y**   | Y      |
//!
//! # Tuners
//!
//! Only supports the KAT500 Automatic Antenna Tuner.
//!
//! | Command                                       | ID         |
//! |-----------------------------------------------|------------|
//! | ClearFaultCondition                           | `FLTC`     |
//! | ForceBypassMode                               | `BYP`      |
//! | Get/Set AmplifierInterfaceRelayClosedState    | `AMPI`     |
//! | Get/Set AntennaSelection                      | `AN` \[1]  |
//! | Get/Set AntennaSideSelection                  | `SIDE`     |
//! | Get/Set AttenuatorState                       | `ATTN`     |
//! | Get/Set AutoBypassState                       | `AB`       |
//! | Get/Set AutoEnableState                       | `AE`       |
//! | Get/Set Band                                  | `BN`       |
//! | Get/Set BaudRate                              | `#BR` \[1] |
//! | Get/Set CapacitorTopology                     | `CT`       |
//! | Get/Set CapacitorValue                        | `C`        |
//! | Get/Set DemoModeState                         | `DM`       |
//! | Get/Set FanInhibitState                       | `IF`       |
//! | Get/Set FanThreshold                          | `FC`       |
//! | Get/Set FaultDelayTime                        | `FDT`      |
//! | Get/Set FaultThresholdHigh                    | `FT0`      |
//! | Get/Set FaultThresholdLow                     | `FT1`      |
//! | Get/Set FixedBypassState                      | `FY`       |
//! | Get/Set FixedLcState                          | `FX`       |
//! | Get/Set InductanceTap                         | `I`        |
//! | Get/Set InductorSwitch                        | `L`        |
//! | Get/Set KeepInPlaceState                      | `AKIP`     |
//! | Get/Set MeterType                             | `MT`       |
//! | Get/Set OperatingFrequency                    | `F`  \[1]  |
//! | Get/Set OperatingMode                         | `MD` \[1]  |
//! | Get/Set PresetSlotNumber                      | `AP`       |
//! | Get/Set SwrBypassThreshold                    | `VSWRB`    |
//! | Get/Set TuningPower                           | `TP`       |
//! | Get/Set TuningSatisfiedSwrThreshold           | `FTNS`     |
//! | Get/Set TuningSpeedLimit                      | `SL`       |
//! | GetErrorMessage                               | `EM`       |
//! | GetFaultCondition                             | `FLT`      |
//! | GetFirmwareVersion                            | `RV` \[1]  |
//! | GetForwardPowerSensorInput                    | `PSI`      |
//! | GetForwardVoltage                             | `VFWD`     |
//! | GetMeterChannelAForwardPower                  | `FA`       |
//! | GetMeterChannelBForwardPower                  | `FB`       |
//! | GetPowerStatus                                | `PS`       |
//! | GetReflectedVoltage                           | `VRFL`     |
//! | GetSerialNumber                               | `SN`       |
//! | GetSwr                                        | `VSWR`     |
//! | GetSwrMeter                                   | `SM`       |
//! | GetTuningState                                | `T`        |
//! | ResetDevice                                   | `RSTX`     |
//! | ResetToFactoryDefaults                        | `EEINIT`   |
//! | StartTuningCycle                              | `ST`       |
//!
//! ## Notes
//!
//! 1. Has the same command ID, and meaning, as a transceiver command, but different argument/return
//!    types, or range of valid values.
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
