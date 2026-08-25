# Rig Control Quirks, Bugs & Gotchas Reference

A personal reference of known quirks, bugs, and non-obvious behaviors in CAT (Yaesu/Kenwood/Elecraft) and CI-V (Icom) computer-control implementations, gathered from Hamlib GitHub issues, groups.io mailing lists, vendor forums, and other public discussions. Each entry is a summary in my own words with a link back to the original source for full details.

---

## CAT — Yaesu

### FT-450 / FT-450D
- **RF power (PC) command scale changed between models**: The `PC` CAT command range is 000–255 (arbitrary scale) on the plain FT-450 but 005–100 (actual watts) on the FT-450D. Software that doesn't distinguish the two by their `ID` response (0241 vs 0244) will set/report the wrong power level. [Source](https://github.com/Hamlib/Hamlib/issues/43)
- **Unwanted PTT activation via hardware flow control**: Several users found the FT-450D keys up immediately and stays keyed when connected to CAT software (QLog, flrig) — traced to hardware (RTS/DTR) flow control being enabled on the serial port, which some digital-mode interfaces use for their own PTT line and which the radio interprets as a key-down signal. [Source](https://forum.digirig.net/t/ft-450d-ptt-always-on-in-qlog-and-flrig/9906)

### FT-710
- **CAT-driven mode change interacts with "CW FREQ DISPLAY" setting**: When the radio's "PITCH OFFSET" CW display option is active, changing mode via CAT (e.g. clicking a DX cluster spot) can leave the displayed/tuned frequency shifted rather than landing exactly on the spotted frequency. [Source](https://github.com/foldynl/QLog/discussions/348)
- **CAT disconnects when switching into DATA-U mode**: Switching from USB to DATA-U (packet/digital USB) via CAT triggers a serial communication error requiring manual reconnection in at least one control program. [Source](https://github.com/foldynl/QLog/discussions/348)
- **Power level misreported over CAT**: One user found Hamlib reported 32 W of output when the radio was actually running 100 W, while flrig read it correctly — pointing to a Hamlib/CAT-layer scaling issue rather than the radio itself. [Source](https://github.com/foldynl/QLog/discussions/348)
- **Best compatibility using the FTDX10 profile**: As of Hamlib 4.5.5, the dedicated FT-710 backend was incomplete enough that users got more reliable control by selecting the FTDX10 rig profile instead of "FT-710" in their software. [Source](https://github.com/foldynl/QLog/discussions/348)

### FTDX10
- **Documented read-only command used as a set command**: A third-party CAT control app was found sending `PR0;`/`PR1;` to toggle the speech processor, but per Yaesu's own CAT manual `PR` is a read-only status query, not a set command — a good example of a command that looks settable but isn't. [Source](https://github.com/mm5agm/Yaesu_Web_Control)

### FTX-1
- **Split command forces Sub VFO into TX and freezes it**: Sending the standard split-enable CAT command (`S 1 1`) puts the Sub VFO into a forced transmit/PTT state (the TX LED lights) and its frequency becomes unchangeable while in that state — which breaks satellite Doppler-correction software that expects to retune the Sub (uplink) VFO continuously during split operation. [Source](https://github.com/Hamlib/Hamlib/issues/1972)

### FT-991 / FT-991A
- **CAT rate menu must match exactly or commands lag/fail**: Menu item 031 (CAT rate) has to match the baud rate configured in the control software; mismatches are a very common cause of "it sort of works but is laggy or drops" reports. [Source](https://groups.io/g/FT-991A/topic/cat_access/95495138)
- **Hamlib 3.3 CAT timeouts that don't occur in FLDigi**: Users reported "Communication timed out while getting current VFO frequency" errors under Hamlib 3.3 with WSJT-X/JTDX using identical serial settings that worked fine in FLDigi, indicating a Hamlib-side (not radio-side) protocol issue at that version. [Source](https://github.com/Hamlib/Hamlib/issues/167)
- **C4FM digital mode not a valid CAT mode token**: The radio's actual digital voice mode is C4FM, but Hamlib originally only exposed "AMS" as a mode string; setting C4FM via CAT caused the rig to accept the command but then return corrupted status (`Mode: None`, garbage split value) on the next query, disconnecting logging software. [Source](https://github.com/Hamlib/Hamlib/issues/1027)

### FT-891
- **Mandatory ~50 ms post-write delay for back-to-back CAT writes**: Hamlib's FT-891 driver inserts a 50 ms delay after each write specifically because sending sequential "fast" CAT commands without it causes problems — a concrete example of the "radio needs a break between commands" issue. [Source](https://github.com/Hamlib/Hamlib/blob/master/rigs/yaesu/ft891.h)
- **Memory channels can't be written via CAT**: The CAT interface can read/tune the FT-891 but Hamlib cannot write memory channel contents to it, a documented limitation rather than a bug per se. [Source](https://github.com/Hamlib/Hamlib/issues/1338)
- **Reports of radio sticking in TX during a CAT "Test PTT"**: At least one user reported the FT-891 getting stuck transmitting when WSJT-X's CAT test triggered PTT, though Hamlib maintainers were unable to reproduce it, so treat as an anecdotal/possibly cable-specific issue. [Source](https://github.com/Hamlib/Hamlib/issues/1338)

### FTDX101MP / FTDX101D
- **Multiple documented control gaps found during a detailed CAT audit**: A thorough test of the FTDX101 turned up several commands that don't work as expected: FM mode isn't exposed as its own mode (only PKTFM is), AM/FM filter width settings have no effect, MIC GAIN is reported on a 0–40 scale instead of the documented 0–100%, and COMP/MONITOR/LOCK can't be reliably toggled via CAT. [Source](https://github.com/Hamlib/Hamlib/issues/423)
- **MP vs D differ in RF power CAT range**: The FTDX101MP supports up to 200 W in its power-set command while the FTDX101D tops out at 100 W — software has to know which variant it's talking to. [Source](https://github.com/Hamlib/Hamlib/issues/423)

### FTDX3000
- **Band change via CAT can drop receive audio**: Switching bands through WSJT-X causes the VFO-A RX indicator to flash and the waterfall to go silent even though the S-meter shows a signal; audio only returns after manually pressing RX or reselecting VFO-A on the radio/software. [Source](https://github.com/Hamlib/Hamlib/issues/903)
- **Antenna/band selection is per-VFO, catching out split operation**: `get_ant`/band-select behavior is tied to whichever VFO is active, so in split mode the "current" antenna reported can be the wrong one unless both VFOs are set explicitly. [Source](https://github.com/Hamlib/Hamlib/issues/510)
- **No native profile in some CAT programs**: At least one popular logger doesn't list "FTDX3000" as an option; users are told to pick the FTDX-5000 profile as a compatible substitute. [Source](https://forum.log4om.com/viewtopic.php?t=6987)

### FT-857D
- **Spontaneous VFO A/B toggling under CAT polling**: Multiple users report the radio's active VFO flipping between A and B on its own while a CAT/logging program is polling it, intermittently and without a clear trigger; a related Hamlib fix note describes avoiding setting RX frequency while transmitting (and vice versa) specifically for "VFO swapping rigs" like this one. [Source](https://forum.log4om.com/viewtopic.php?t=8049)
- **Silent 0x00 ACK byte for unimplemented commands**: Per Hamlib developer notes, the rig responds to any CAT command it doesn't recognize/support with a single 0x00 byte rather than an error — a behavior not documented in the official CAT manual that control software has to account for. [Source](https://sourceforge.net/p/hamlib/discussion/25919/thread/157a4ec00f/)

### FT-817ND
- **CW narrow filter changes undocumented status byte values**: With the optional CW narrow filter installed and turned on, the frequency/mode status query returns values like 0x82/0x83/0x8A instead of the documented 0x02/0x03/0x0A (filter off returns the documented values) — an undocumented high-bit flag that trips up naive parsers. [Source](https://www.mail-archive.com/wsjt-devel@lists.sourceforge.net/msg21265.html)

### General Yaesu (newcat-based rigs)
- **"?;" busy response wrongly treated as a fatal error across the whole newcat family**: When a Yaesu rig is momentarily busy and returns `?;`, Hamlib's shared "newcat" backend (used by the FT-991, FT-891, FTDX10, FTDX101, etc.) can treat that as a permanent command rejection instead of a retriable "try again" condition, so `IF;`/`FA;`/`NA0;` queries fail outright instead of being retried. [Source](https://github.com/Hamlib/Hamlib/issues/505)

---

## CAT — Kenwood

### TS-480
- **Hamlib 4.6.5 regression breaks PTT with a timeout**: Starting in Hamlib 4.6.5, the TX CAT command stopped receiving any reply from the radio, so Hamlib waits out its 500 ms timeout and reports a communication error on every PTT attempt; reverting to 4.6.4 fixed it for affected users. [Source](https://github.com/Hamlib/Hamlib/issues/1989)
- **`TX0;` PTT command mutes line-in audio**: Hamlib issues `TX0;` for CAT-triggered PTT, which the radio interprets as "transmit using the MIC input," muting the ACC2/USB line-input audio path used for digital modes — so CAT PTT can key the radio but send no audio. [Source](https://hamlib-developer.narkive.com/pVRU5oNr/ts-590-and-newer-models-ptt-issue)
- **CAT memory writes silently force SPLIT mode**: A CHIRP bug report found that writing regular (non-split) memory channels to a TS-480HX/SAT via CAT caused all channels to be stored with the SPLIT flag set, even when both frequencies were identical. [Source](https://chirpmyradio.com/issues/8297)

### TS-570
- **Filter/bandwidth parameter ignored on mode-set**: Hamlib's `kenwood_set_mode()` documented behavior ignores the width/filter argument specifically for the TS-570 (also K2/K3), so requesting a particular filter width via CAT has no effect — width has to be set separately. [Source](https://github.com/Hamlib/Hamlib/blob/master/rigs/kenwood/README.kenwood)
- **Needs explicit RTS/CTS hardware handshake**: Reliable CAT communication with the TS-570D depends on enabling hardware flow control in the serial settings; without it, connections can be intermittent even on a supported Hamlib version. [Source](https://www.cqrlog.com/node/3613)

### TS-590 (S / SG)
- **`TX0;` PTT semantics also mute ACC2/USB audio**: The same mic-input-vs-line-in mismatch documented for the TS-480 applies to the TS-590 — CAT PTT keys the radio but can leave the line-level digital-mode audio muted. [Source](https://hamlib-developer.narkive.com/pVRU5oNr/ts-590-and-newer-models-ptt-issue)
- **ATT/PREAMP level get/set broken via Hamlib bindings**: Reading `RIG_LEVEL_PREAMP` returns `None` and attempts to set it don't take effect, reported specifically against the Python bindings. [Source](https://github.com/Hamlib/Hamlib/issues/953)
- **S-meter (`SM`) readback broken on both S and SG due to a missing calibration table**: A Hamlib developer confirmed the `SM` CAT command itself is identical between the TS-590S and TS-590SG, but S-meter readout fails on both because Hamlib's capability table lacks a calibration curve for either radio — not a difference between the two models as initially suspected. [Source](https://sourceforge.net/p/hamlib/mailman/message/34773775/)

### TS-890S
- **Starting VFO selection breaks split-frequency commands**: If VFO B happens to be the active VFO when Hamlib connects, subsequent `rig_set_split_freq_mode()` calls always change VFO B — even when the transmit VFO should be VFO A — because of how Hamlib's VFO "fixup" logic resolves the TX VFO on this model. Starting from VFO A avoids the problem. [Source](https://github.com/Hamlib/Hamlib/issues/745)

### TS-2000
- **Needs hardware handshake for reliable connection**: Intermittent Hamlib timeout errors on the TS-2000 are commonly resolved by explicitly enabling hardware (RTS/CTS) flow control, e.g. `rigctl -m 214 -s 57600 -r COM1 -C serial_handshake=Hardware`. [Source](https://www.scivision.dev/rigctl-hamlib-with-kenwood-ts-2000/)
- **Backend carries special-case handling for SDR "Kenwood-emulation" quirks**: Because real Kenwood rigs don't reply to some set commands (e.g. `Rx;`) the way various SDR programs emulating a TS-2000 do, Hamlib's TS-2000 backend has extra code paths to tolerate those emulator differences — a reminder that "TS-2000 compatible" devices don't all behave identically over CAT. [Source](http://navtronik.com/otj2/hamlib-kenwood.html)

### General Kenwood (confirmed on TS-480SAT and TS-590S)
- **Noise reduction mode 2 (NR2) can't be set via CAT, only read**: `rigctl` can read back that NR is in mode 2 after it's set manually on the radio, but issuing the equivalent set command (`U VFOA NR 2`) fails, while modes 0 and 1 work fine in both directions. [Source](https://github.com/Hamlib/Hamlib/issues/1625)

---

## CAT — Elecraft

### K3 / K3S / KX3 / KX2 (shared CAT command set)
These four models share the same Elecraft CAT Programmer's Reference and firmware lineage, so most of the following apply across the family.
- **Frequency (band) change must precede mode change, with a settle delay**: There is no dedicated "band change" CAT command — changing bands is done by setting frequency — and the radio remembers the last-used mode *per band*. Sending mode before frequency (or not waiting long enough between them) can leave the rig in the wrong mode after a band change, e.g. when auto-tuning to a DX cluster spot. [Source](https://forum.log4om.com/viewtopic.php?t=2213)
- **Switch-emulation commands need a delay before their effect can be queried**: The Programmer's Reference explicitly documents that commands like `SWT16;` (switch press emulation, e.g. XMIT) must be followed by a delay before a status command like `TQ;` will reflect the change. [Source](https://ftp.elecraft.com/KX2/Manuals%20Downloads/K3S&K3&KX3&KX2%20Pgmrs%20Ref,%20G4.pdf)
- **Commands sent while the rig is "busy" return `?;` instead of queuing**: States like transmitting, BSET, or VFO-reverse limit which commands can be processed; a busy rig returns `?;` rather than an error, and the reference recommends checking `TQ;`/`IC;` status rather than polling continuously (especially fast polling during TX, which should be avoided). [Source](https://ftp.elecraft.com/KX2/Manuals%20Downloads/K3S&K3&KX3&KX2%20Pgmrs%20Ref,%20G4.pdf)
- **Hamlib K3 driver historically reported to hang after extended use**: At least one user reported Hamlib locking up after some minutes of K3 control while other CAT software (Ham Radio Deluxe, OmniRig, RigCAT) worked fine against the same radio, suggesting a Hamlib-side rather than radio-side issue. [Source](https://hamlib-developer.narkive.com/K6RPm5to/k3-problem)
- **KX3 split mode via Hamlib rejects the Sub VFO**: `kenwood_set_split_vfo` fails with "unsupported VFO Sub" when WSJT-X/JS8Call try to enable split on a KX3, a regression from earlier Hamlib releases (4.0–0.9) where it worked. [Source](https://github.com/Hamlib/Hamlib/issues/482)

### K4
- **Wrong CAT command used for Sub-Rx data mode (Hamlib regression)**: The Hamlib build bundled with WSJT-X 2.5 sent `DT0;` (which sets the *main* VFO's data mode) instead of `DT$0;` (Sub VFO) when asked to put the Sub receiver into DATA mode — so instead of the Sub VFO switching to data mode, the main VFO did, breaking FT8/data split operation. The equivalent command for the "K" (main) VFO worked; only the "$"-prefixed Sub VFO variant was wrong. [Source](https://github.com/Hamlib/Hamlib/issues/825)

---

## CI-V — Icom

### IC-705
- **PTT-specific I/O error even though CAT/RX works fine**: Users report CAT connecting successfully (frequency/mode tracking works) but PTT attempts fail with a serial re-open error (`serial_open status=-6`), suggesting the PTT path reopens or re-negotiates the port separately from the CAT data path and can fail independently of it. [Source](https://github.com/Hamlib/Hamlib/issues/986)
- **CI-V Transceive (on by default) causes protocol errors with JTDX over the Icom Remote Utility**: With Transceive left on — the IC-705's factory default — users hit "Protocol error while getting current VFO frequency" and "Command rejected" errors; turning Transceive off in Connectors → CI-V resolves it. [Source](https://ramdor.co.uk/2021/01/02/icom-remote-utility-issue-and-hamlib/)

### IC-905
- **Manual VFO changes on the radio truncate the frequency read back over CI-V**: Retuning the VFO on the transceiver itself (e.g. 10386.100.000 → 10386.099.000) can cause the frequency reported back over CAT to drop its leading digits (read as "386.099.000"), producing a false out-of-band error in logging/WSJT-X. [Source](https://github.com/Hamlib/Hamlib/issues/1314)

### IC-9700
- **Doppler-correction commands cause Main/Sub VFO swapping**: Each time satellite-tracking software (gpredict) sends a frequency correction, the radio's Main and Sub VFOs can swap places and swap back, and SPLIT can activate unexpectedly — reproduced on both the dedicated IC-9700 driver (Hamlib 4.0) and the older IC-910-based driver (3.3), pointing to a deeper protocol-handling issue rather than a single driver bug. [Source](https://github.com/csete/gpredict/issues/181)
- **No direct "AGC off" command — must zero the AGC time constant instead**: To fully disable AGC on the IC-9700 (and IC-7300) over CI-V, you have to set the AGC time-constant parameter (command `1A 04`) to 0 rather than using a dedicated AGC-off setting. [Source](https://github.com/Hamlib/Hamlib/issues/1136)

### IC-R8600
- **Memory channels 11 and 13 misread due to flow-control byte collision**: Reading memory channels 11 or 13 via the `1A 00` CI-V command returns malformed data (missing channel number; reading channel 13 corrupts the next transfer) — root-caused to software (XON/XOFF) flow control being enabled on the PC's serial port, since 0x11/0x13 are the XON/XOFF control bytes and the OS was intercepting them from the CI-V data stream. Disabling software flow control fixes it. [Source](https://forums.radioreference.com/threads/ic-r8600-ci-v-command-bug.399175/)

### IC-7610
- **Power meter shows the power *setting*, not actual output**: `RIG_LEVEL_RFPOWER_METER_WATTS` wasn't implemented for the 7610, so under JTDX the "power" readout reflected the configured power level rather than a real-time measurement of actual RF output. [Source](https://github.com/Hamlib/Hamlib/issues/533)

### IC-7100
- **CI-V Transceive vs. USB Echo Back trip up USB-only setups**: When connecting over the USB CI-V virtual port (rather than the dedicated CI-V jack), CI-V Transceive should be turned off and "CI-V USB Echo Back" turned on, or users hit protocol errors and rejected commands. [Source](https://2e0pgs.github.io/blog/programming/2018/12/17/ic7100-hamlib/)
- **19200 baud is the practical ceiling for combined USB+CI-V**: Even though higher rates are selectable, the highest speed reported to work reliably when both USB and the CI-V socket are connected simultaneously is 19200 baud. [Source](https://2e0pgs.github.io/blog/programming/2018/12/17/ic7100-hamlib/)

### IC-7300
- **CI-V USB Echo Back needs to be manually enabled for non-flrig software**: The radio's USB CI-V echo setting defaults in a way that's fine for flrig but several other CAT programs need "CI-V USB Echo Back" explicitly turned On to work correctly. [Source](https://www.w1hkj.org/flrig-help/ic7300_setup.html)
- **USB audio-level CI-V command address changed on the Mk2**: The command to get/set the USB AF (audio) level moved from sub-command `0x60` to `0x70` on the IC-7300 Mk2; software (including Hamlib at the time) written for the original IC-7300 targets the wrong parameter address on a Mk2. [Source](https://github.com/Hamlib/Hamlib/issues/1985)
- **Bandwidth/passband width set command silently ignored**: Sending a custom filter width (e.g. 2400 Hz) appears to succeed, but querying the mode immediately afterward shows the width reverted to its previous value (e.g. 3000 Hz) — the set is effectively a no-op despite no error being returned. [Source](https://github.com/Hamlib/Hamlib/issues/811)
- **Auto power-on over CI-V is unreliable**: Multiple separate bug reports spanning several years describe CI-V "power on" commands to a powered-off IC-7300 timing out rather than waking the radio, making remote/scripted power-on unreliable. [Source](https://github.com/Hamlib/Hamlib/issues/1142)

### IC-7851
- **Split mode fails outright because VFO B isn't defined for this model in some drivers**: Attempting to enable split produces "rig does not have VFOB" — the driver's capability table for the 7851 doesn't declare a second VFO, so split mode can't be engaged at all through that path. [Source](https://github.com/Hamlib/Hamlib/issues/654)
- **Radio can end up parked on the Sub receiver instead of Main**: When control software (rigctld/WSJT-X) sets frequency and mode on connect, VFO resolution logic can resolve "current VFO" to Sub rather than Main, leaving the radio tuned correctly but listening on the wrong receiver. [Source](https://github.com/Hamlib/Hamlib/issues/231)

### General CI-V (cross-model)
- **Default CI-V addresses collide when multiple radios of the same model share a bus**: Every radio of a given model ships with the same factory default CI-V address, so connecting two identical rigs to one CI-V bus requires manually reassigning at least one radio's address in its menu before software can address them individually. [Source](https://www.onallbands.com/making-the-ci-v-connection%EF%BB%BF/)
- **CI-V Transceive can cause bus collisions with multiple listeners**: When Transceive (automatic frequency/mode broadcast) is left on, a radio's own unsolicited status broadcasts can collide on the bus with commands issued by control software or other devices at the same moment; the common workaround is to disable Transceive everywhere and have the control software poll for status instead. [Source](http://www.dxlabsuite.com/dxlabwiki/IcomCIVTransceive)

---

## Radios with no quirks found

Despite targeted searches, no specific, verifiable CAT/CI-V quirks, bugs, or gotchas turned up for these priority-list radios (general setup/configuration guidance exists, but nothing rising to a documented quirk with a citable source):

- Kenwood TS-990S
- Lab599 TX-500 *(CAT support/compatibility questions found in the Lab599 groups.io list, but no confirmed, specific bug with enough detail to cite)*
- Icom ID-50A/ID-50E
