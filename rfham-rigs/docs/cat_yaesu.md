# Yaesu CAT — Computer-Control Protocol Reference

> **Scope of this file.** This document covers the **Yaesu** family of "CAT" (Computer
> Aided Transceiver) command sets. Yaesu, Kenwood, and Elecraft/Lab599 each publish a
> *completely separate and non-interoperable* control protocol; they share only the
> marketing term "CAT". The Kenwood and Elecraft/Lab599 protocols are documented in the
> sibling files `cat_kenwood.md` and `cat_elecraft.md`. A command mnemonic such as `FA`
> may appear in more than one vendor's set with a *different* message layout — do not
> assume cross-vendor compatibility.
>
> Radios covered here: FT-450, FT-450D, FTDX3000, FT-991, FT-991A, FT-891, FTDX10,
> FT-710, FTDX101MP/D, FTX-1 (ASCII protocol) and FT-857D, FT-817ND (binary protocol).

---

## 1. Two distinct Yaesu protocols

Even *within* Yaesu there are **two structurally different CAT protocols**, and they are
not interchangeable:

| Protocol | Framing | Radios |
|---|---|---|
| **ASCII / semicolon** ("newcat"-style) | Printable ASCII, 2-letter command + fixed-width ASCII parameters, terminated by `;` | FT-450, FT-450D, FTDX3000, FT-991, FT-991A, FT-891, FTDX10, FT-710, FTDX101MP/D, FTX-1 |
| **5-byte binary opcode** | Five raw hex bytes per command, opcode last | FT-857D, FT-817ND (and the closely related FT-817, FT-897) |

Parts 3–5 below cover the ASCII protocol (the large majority of the fleet). Part 6 covers
the 5-byte binary protocol used by the FT-857D / FT-817ND.

---

## 2. ASCII protocol fundamentals

A control command is: **2-letter command** + **parameters** + **terminator (`;`)**.
Commands are case-insensitive on input. There are three message directions:

- **Set** — computer → radio, sets a condition (e.g. `FA14250000;`).
- **Read** — computer → radio, requests a value (e.g. `FA;`).
- **Answer** — radio → computer, returns a value (e.g. `FA014250000;`).

Not every command supports all three directions. The per-command "Set / Read / Answer"
capability is enumerated in the master matrix (§4).

**Serial framing.** 8 data bits, no parity, 2 stop bits (8N2 on most models; the CAT-rate
menu selects the baud rate — commonly 4800/9600/19200/38400). The USB-virtual-COM port on
modern radios also carries CAT.

**Parameter rules (from the manuals).**
- Each parameter field has a **fixed, predetermined width**; you must pad numeric fields
  with leading zeros to the full width (e.g. IF-shift `IS0+1000;`, not `IS0+100;`).
- Where a field is "not applicable" to a given model, fill it with any character except
  ASCII control codes (00–1Fh) and the terminator `;`.
- `Pn` in the layout tables denotes a parameter; repeated `Pn` across positions means that
  parameter spans those character positions.

**Auto-Information (AI).** With `AI1;` set, the radio spontaneously emits Answer messages
when its state changes at the front panel. `AI` resets to `0` (OFF) at power-off. The "AI"
column in the matrix indicates whether a command participates in auto-information reporting.

**`?;` busy response.** When momentarily busy (e.g. during TX or a menu operation) a Yaesu
radio may answer a command with `?;`. This is a *retriable* "try again later" condition,
not a permanent rejection — see the general Yaesu quirk in §7.

---

## 3. Mode codes (ASCII `MD` / `IF` / `MR` parameter)

Modern radios (FTDX10, FT-710, FT-991/A, FT-891, FTDX101, FTX-1, FTDX3000) use this mode
table for the `MD`, `IF`, `OI`, `MR`, `MT`, `MW` mode field:

| Code | Mode | Code | Mode |
|---|---|---|---|
| 1 | LSB | 9 | RTTY-U |
| 2 | USB | A | DATA-FM (C4FM/PKT-FM on some) |
| 3 | CW-U | B | FM-N |
| 4 | FM | C | DATA-U |
| 5 | AM | D | AM-N |
| 6 | RTTY-L | E | PSK |
| 7 | CW-L | F | DATA-FM-N |
| 8 | DATA-L | | |

**Legacy radios (FT-450 / FT-450D)** use a smaller, *different* set for `MD`/`IF`:
1 LSB, 2 USB, 3 CW, 4 FM, 5 AM, 6 DATA (RTTY-LSB), 7 CW-R, 8 USER-L, 9 DATA (RTTY-USB),
B FM-N, C USER-U. (No separate CW-U/CW-L, no DATA-FM/AM-N/PSK.) Always confirm the model's
own mode table before hard-coding these values.

---

## 4. Master command support matrix (ASCII radios)

`Y` = the command has a documented control-command table in that model's CAT manual;
`—` = not documented for that model. Set/Read/Answer capability per command is uniform
across the family unless noted in the per-command section. Column order roughly oldest →
newest. (Command names are Yaesu's; a few auto-extracted names are noted/corrected in the
per-command sections.)

| Cmd | 450 | 450D | 3000 | 991 | 991A | 891 | DX10 | 710 | DX101 | FTX-1 | Function |
|-----|:---:|:----:|:----:|:---:|:----:|:---:|:----:|:---:|:-----:|:-----:|----------|
| AB | — | — | Y | Y | Y | Y | Y | Y | Y | — | VFO-A → VFO-B |
| AC | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Antenna tuner control |
| AG | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | AF gain |
| AI | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Auto-information |
| AM | — | — | Y | Y | Y | Y | Y | Y | Y | Y | VFO-A → memory channel |
| AN | — | — | Y | — | — | — | — | — | Y | — | Antenna number |
| AO | — | — | — | — | — | — | Y | Y | Y | Y | AMC output level |
| AS | — | — | — | — | — | — | — | Y | — | — | AESS (Acoustic Enhanced Speaker) |
| AV | — | — | — | — | — | — | Y | Y | Y | — | Anti-VOX level |
| BA | — | — | Y | Y | Y | Y | Y | Y | Y | — | VFO-B → VFO-A |
| BC | — | — | Y | Y | Y | Y | Y | Y | Y | Y | Auto notch (DNF) |
| BD | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Band down |
| BI | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Break-in |
| BM | — | — | — | — | — | — | Y | Y | Y | — | VFO-B/Sub → memory channel |
| BP | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Manual notch |
| BS | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Band select |
| BU | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Band up |
| BY | Y | Y | Y | Y | Y | Y | Y | — | Y | — | Busy (squelch) status |
| CF | — | — | — | — | — | Y | Y | Y | — | Y | Clarifier (combined) |
| CH | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Channel up/down |
| CN | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | CTCSS tone number |
| CO | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Contour / APF |
| CS | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | CW spot |
| CT | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | CTCSS on/off |
| DA | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Dimmer / LCD contrast |
| DN | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Mic down / down |
| DS | Y | Y | — | — | — | — | — | — | — | — | Dimmer switch (450/450D only) |
| DT | — | — | — | Y | Y | — | Y | Y | Y | Y | Date and time |
| ED | Y | Y | Y | Y | Y | Y | Y | — | Y | — | Encoder down |
| EK | — | — | Y | Y | Y | Y | — | — | — | — | ENT key |
| EM | — | — | Y | — | — | — | Y | — | Y | — | Encode memory |
| EN | — | — | Y | — | — | — | Y | — | Y | — | Encode |
| EO | — | — | — | — | — | — | — | — | — | Y | Encoder offset |
| EU | Y | Y | Y | Y | Y | Y | Y | — | Y | — | Encoder up |
| EX | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Menu |
| FA | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Frequency VFO-A |
| FB | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Frequency VFO-B |
| FN | — | — | — | — | — | — | Y | Y | Y | Y | Fine tuning |
| FR | — | — | Y | — | — | — | — | — | Y | Y | Function RX |
| FS | Y | Y | Y | Y | Y | Y | — | — | Y | — | Fast step |
| FT | Y | Y | Y | Y | Y | — | Y | Y | Y | Y | Function TX |
| GP | — | — | — | — | — | — | — | Y | — | Y | GP OUT A/B/C/D |
| GT | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | AGC function |
| ID | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Identification |
| IF | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Information (VFO-A) |
| IS | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | IF-shift |
| KM | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Keyer memory |
| KP | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Key pitch |
| KR | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Keyer on/off |
| KS | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Key speed |
| KY | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | CW keying |
| LK | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Lock |
| LM | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Load message |
| MA | — | — | Y | Y | Y | Y | Y | Y | Y | — | Memory channel → VFO-A |
| MB | — | — | — | — | — | — | Y | Y | Y | — | Memory channel → VFO-B/Sub |
| MC | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Memory channel select |
| MD | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Operating mode |
| MG | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Mic gain |
| MK | Y | Y | — | — | — | — | — | — | — | — | Mode key (450/450D only) |
| ML | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Monitor level |
| MR | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Memory channel read |
| MS | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Meter switch |
| MT | — | — | — | Y | Y | — | Y | Y | Y | Y | Memory channel write/tag |
| MW | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Memory channel write |
| MX | — | — | Y | Y | Y | Y | Y | — | Y | Y | MOX set |
| MZ | — | — | — | — | — | — | — | — | — | Y | Split memory |
| NA | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Narrow |
| NB | Y | Y | Y | Y | Y | Y | Y | Y | Y | — | Noise blanker status |
| NL | — | — | Y | Y | Y | Y | Y | Y | Y | Y | Noise blanker level |
| NR | Y | Y | Y | Y | Y | Y | Y | Y | Y | — | Noise reduction on/off |
| OI | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Opposite-band information |
| OS | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Offset (repeater shift) |
| PA | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Pre-amp (IPO) |
| PB | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Play back (DVS) |
| PC | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Power (RF output) control |
| PL | — | — | Y | Y | Y | Y | Y | Y | Y | Y | Speech processor level |
| PR | — | — | Y | Y | Y | Y | Y | Y | Y | Y | Speech processor on/off |
| PS | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Power switch |
| QI | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | QMB store |
| QR | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | QMB recall |
| QS | Y | Y | Y | Y | Y | Y | Y | — | Y | — | Quick split |
| RA | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | RF attenuator |
| RC | Y | Y | Y | Y | Y | Y | Y | — | Y | — | Clarifier clear |
| RD | Y | Y | Y | Y | Y | Y | Y | — | Y | — | Clarifier down |
| RF | — | — | Y | — | — | — | Y | — | Y | — | Roofing filter |
| RG | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | RF gain |
| RI | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Radio information |
| RL | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Noise reduction level |
| RM | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Read meter |
| RO | — | — | Y | — | — | — | — | — | — | — | Rotator (FTDX3000) |
| RP | Y | Y | — | — | — | — | — | — | — | — | Reset power on (450/450D) |
| RS | Y | Y | Y | Y | Y | Y | Y | — | Y | — | Radio status (normal/menu) |
| RT | Y | Y | Y | Y | Y | — | Y | — | Y | — | RX clarifier on/off |
| RU | Y | Y | Y | Y | Y | Y | Y | — | Y | — | Clarifier up |
| SC | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Scan |
| SD | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | CW break-in delay time |
| SF | — | — | Y | — | — | — | Y | Y | Y | Y | Sub-dial function |
| SH | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Width (IF bandwidth) |
| SM | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | S-meter reading |
| SQ | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Squelch level |
| SS | — | — | — | — | — | — | Y | Y | Y | Y | Spectrum scope |
| ST | Y¹ | Y¹ | — | — | — | Y² | Y² | Y² | Y² | Y² | ST = STEP (450/450D) / SPLIT (modern) |
| SV | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Swap VFO |
| SY | — | — | — | — | — | — | — | — | Y | — | Sync (FTDX101) |
| TS | Y | Y | Y | Y | Y | Y | Y | Y | — | Y | TXW (transmit watch) |
| TX | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | TX set (PTT) |
| UL | Y | Y | Y | Y | Y | Y | — | — | Y | — | PLL unlock status |
| UP | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Mic up / up |
| VD | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | VOX delay time |
| VE | — | — | — | — | — | — | — | Y | — | Y | Firmware version |
| VF | — | — | Y | — | — | — | — | — | — | — | VRF filter (FTDX3000) |
| VG | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | VOX gain |
| VM | — | — | Y | Y | Y | Y | Y | Y | Y | Y | [V/M] key / VFO→memory |
| VR | Y | Y | — | — | — | — | — | — | — | — | Voice (450/450D) |
| VS | Y | Y | Y | — | — | — | Y | Y | Y | Y | VFO select |
| VT | — | — | — | — | — | — | — | — | Y | — | VC tune (FTDX101) |
| VV | Y | Y | — | — | — | — | — | — | — | — | VFO → VFO (450/450D) |
| VX | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | VOX status |
| XT | — | — | Y | Y | Y | — | Y | Y | Y | — | TX clarifier |
| ZI | — | — | — | Y | Y | Y | Y | Y | Y | Y | Zero-in |

¹ On the FT-450 / FT-450D, mnemonic **`ST` = STEP** (tuning step). ² On the modern
radios, **`ST` = SPLIT**. This is a genuine cross-model collision of the same mnemonic —
see §5 (`ST`). ³ A handful of names in this matrix were auto-extracted; where a name looked
off (e.g. `VM`, `MR`) it is corrected in the per-command section.

> **Matrix caveats.** Presence is derived from each manual's own control-command tables.
> The FTDX101 column represents the FTDX101MP/D combined manual; the FT-991 and FT-991A
> manuals are nearly identical CAT sets. A "—" means the command is not in that model's
> published CAT table; some undocumented commands may still be silently accepted. The
> FT-857D and FT-817ND are **not** in this matrix — they use the binary protocol (§6).

---

## 5. ASCII command reference

Layouts below are given in Yaesu's character-position form. The canonical layout shown is
the modern one (FTDX10 / FT-710 / FTX-1, which share it); legacy (FT-450/450D) and
model-specific differences are called out per command. In the message strings, each `Pn`
occupies one character position unless a width/range is stated; numeric fields are
zero-padded to their full width. `;` is the terminator.

### AB — VFO-A to VFO-B

Copies VFO-A contents into VFO-B. Set-only.

| Direction | Message |
|---|---|
| Set | `AB;` |

Not present on FT-450/450D or FTX-1.

### AC — Antenna Tuner Control

| Direction | Message | Notes |
|---|---|---|
| Set | `AC P1 P2 P3 ;` | 5 chars: `AC`+P1+P2+P3+`;` |
| Read | `AC;` | |
| Answer | `AC P1 P2 P3 ;` | |

- **P1** `0`: fixed.
- **P2** `0`: fixed (internal/external tuner). On FT-710/FTX-1, P2 also selects `2`: ATAS.
- **P3** `0`: Tuner OFF (tuning stop) · `1`: Tuner ON · `2`: Tuning start (FT-450D: `2` = Tuning start; FTDX10 merges "Tuning start/stop" onto `2`). When P2=2 (ATAS, FT-710/FTX-1): `0` stop, `1` freq up 50 ms, `2` freq down 50 ms, `3` tuning start.

### AG — AF Gain

| Direction | Message | Notes |
|---|---|---|
| Set | `AG P1 P2 P2 P2 ;` | P1 `0` fixed; P2 = `000`–`255` |
| Read | `AG P1 ;` | P1 `0` |
| Answer | `AG P1 P2 P2 P2 ;` | |

AF gain scale is `000`–`255` on all ASCII models.

### AI — Auto Information

| Direction | Message | Notes |
|---|---|---|
| Set | `AI P1 ;` | P1 `0`: OFF · `1`: ON |
| Read | `AI;` | |
| Answer | `AI P1 ;` | |

Resets to `0` at power-off. On modern radios the note reads "AI is available only when the
PC is connected with a USB cable." No Answer/AI participation (AI column = X).

### AM — VFO-A to Memory Channel

Writes VFO-A into the currently selected memory channel. Set-only (`AM;`). Not on
FT-450/450D.

### AN — Antenna Number

FTDX3000 and FTDX101 only. Selects antenna jack. `AN P1 P2 ;` with P1 = band/receiver
selector and P2 = antenna number (`1`/`2`/`3`); consult the model manual for the exact P1
map (FTDX101 differs from FTDX3000).

### AO — AMC Output Level

FTDX10/FT-710/FTDX101/FTX-1. `AO P1 P1 P1 ;`, P1 = `001`–`100`. Read `AO;`.

### AS — AESS

FT-710 only (Acoustic Enhanced Speaker System). `AS P1 P2 P3 ;` — see FT-710 manual for the
speaker-mode sub-parameters.

### AV — Anti-VOX Level

FTDX10/FT-710/FTDX101. `AV P1 P1 P1 ;`, P1 = `001`–`100`. Read `AV;`.

### BA — VFO-B to VFO-A

Copies VFO-B into VFO-A. Set-only (`BA;`). Not on FT-450/450D or FTX-1.

### BC — Auto Notch (DNF)

| Direction | Message | Notes |
|---|---|---|
| Set | `BC P1 P2 ;` | P1 `0` fixed; P2 `0`: OFF · `1`: ON |
| Read | `BC P1 ;` | |
| Answer | `BC P1 P2 ;` | |

Not on FT-450/450D (which use `BP`/`CO`-style notch instead).

### BD — Band Down

`BD P1 ;` — steps the band down. **P1 differs by model:** FT-450/450D `0`: VFO-A · `1`:
VFO-B. Modern (FTDX10/FT-710/FTX-1) `0`: MAIN band · `1`: SUB band. Set-only.

### BI — Break-In

`BI P1 ;`, P1 `0`: OFF · `1`: ON. Read `BI;`.

### BM — VFO-B / Sub Band to Memory Channel

FTDX10/FT-710/FTDX101. Set-only (`BM;`).

### BP — Manual Notch

| Direction | Message | Notes |
|---|---|---|
| Set | `BP P1 P2 P3 P3 P3 ;` | |
| Read | `BP P1 P2 ;` | |
| Answer | `BP P1 P2 P3 P3 P3 ;` | |

- **P1** `0`: fixed.
- **P2** `0`: Manual notch ON/OFF · `1`: Manual notch frequency.
- **P3** (when P2=0) `000`: OFF, `001`: ON. (when P2=1) FTDX10 `001`–`320` (notch freq ×10 Hz); **FT-450/450D** `001`–`199` move left, `200` center, `201`–`400` move right.

### BS — Band Select

`BS P1 P1 ;` (Set-only). P1 = band code:
`00` 1.8 MHz, `01` 3.5, `02` 5 MHz (FT-450/450D: "Invalid"), `03` 7, `04` 10, `05` 14,
`06` 18, `07` 21, `08` 24.5, `09` 28, `10` 50, `11` GEN, `12` MW (FTDX10; not on 450).

### BU — Band Up

`BU P1 ;`. **P1 differs:** FT-450/450D `0`: fixed. Modern `0`: MAIN band · `1`: SUB band.
Set-only.

### BY — Busy (Squelch) Status

Read/Answer only. `BY;` → `BY P1 P2 ;`. P1 `0`: RX busy OFF · `1`: RX busy ON (squelch
open). P2 `0`: fixed. Not on FT-710/FTX-1.

### CF — Clarifier (combined)

FT-891/FTDX10/FT-710/FTX-1. Combined clarifier read/set (variable-length payload):

| Direction | Message |
|---|---|
| Set | `CF P1 P2 P3 P4 P5 P6 P7 P8 ;` |
| Read | `CF P1 P2 P3 ;` |
| Answer | `CF P1 P2 P3 P4 P5 P6 P7 P8 ;` |

P1 `0`: MAIN · `1`: SUB. P2 `0`: fixed. P3 `0`: CLAR setting, `1`: CLAR frequency.
When P3=0: P4 RX CLAR OFF/ON, P5 TX CLAR OFF/ON, P6–P8 fixed. When P3=1: P4 = `+`/`-`,
P5–P8 = `0000`–`9999` Hz.

### CH — Channel Up/Down

`CH P1 ;` (Set-only). P1 `0`: memory channel UP · `1`: DOWN.

### CN — CTCSS Tone (Number)

| Direction | Message | Notes |
|---|---|---|
| Set | `CN P1 P2 P3 P3 P3 ;` (modern) / `CN P1 P2 P2 ;` (450/450D) | |
| Read | `CN P1 ;` | |
| Answer | as Set | |

Modern: P1 `0` MAIN/`1` SUB, P2 `0` fixed, **P3 = `000`–`049`** tone-number index into the
50-entry CTCSS chart (000=67.0 Hz … 049=254.1 Hz). **FT-450/450D:** no MAIN/SUB byte; P2 =
`00`–`49` tone number. The tone-number → frequency chart is identical across the family
(67.0 Hz base, 50 standard tones).

### CO — Contour / APF

| Direction | Message | Notes |
|---|---|---|
| Set | `CO P1 P2 P3 P3 P3 P3 ;` (modern) | |
| Read | `CO P1 P2 ;` | |
| Answer | as Set | |

Modern (FTDX10): P1 `0` fixed; P2 `0` CONTOUR ON/OFF, `1` CONTOUR FREQ, `2` APF ON/OFF, `3`
APF FREQ. P3 (P2=0) `0000` OFF / `0001` ON; (P2=1) `0010`–`3200` = 10–3200 Hz; (P2=2) APF
OFF/ON; (P2=3) `0000`–`0050` = −250…+250 Hz. **FT-450/450D** differs: P2 `0` ON/OFF, `1`
frequency; P3 is a 2-digit code (dB level when P2=0: −2…+2; band index 01–32 when P2=1).

### CS — CW Spot

`CS P1 ;`, P1 `0`: OFF · `1`: ON. Read `CS;`.

### CT — CTCSS (on/off)

`CT P1 P2 ;` (modern, P1 `0` MAIN/`1` SUB) or `CT P1 P2 ;` (450/450D, P1 `0` fixed). P2 `0`:
OFF · `1`: ENC/DEC (TSQ) · `2`: ENC only. Read `CT P1 ;`.

### DA — Dimmer / LCD Contrast

**Layout differs substantially by generation.** FT-450/450D: `DA P1 P1 P2 P2 ;` (P1 =
dimmer `00`–`08`, P2 `00` fixed). FTDX10/FT-710 (TFT radios): `DA P1 P1 P2 P2 P3 P3 P4 P4
;` — P1 `00` fixed, P2 `00`–`20` TFT contrast, P3 `00`–`20` TFT brightness, P4 `00`–`20` LED
indicator brightness. Read `DA;`.

### DN — Mic Down / Down

Set-only. `DN;` on FTDX10/FT-450D (single-step mic-down). Steps the mic [DWN] function.

### DS — Dimmer Switch

FT-450/450D only. `DS P1 ;`, P1 `0`: OFF · `1`: ON. Read `DS;`.

### DT — Date and Time

FT-991/991A, FTDX10, FT-710, FTDX101, FTX-1 (radios with a real-time clock). Variable
length: `DT P1 P2… ;`. P1 `0`: Date, `1`: Time (UTC). P2 = `yyyymmdd` (P1=0) or `hhmmss`
24-hour (P1=1). Read `DT P1 ;`.

### ED — Encoder Down

`ED P1 P2 P2 ;` (FTDX10). P1 selects the encoder (`0` MAIN, `1` MPVD, `4` MAIN NOTCH, `5`
MAIN CONT, `8` MULTI). P2 = `01`–`99` frequency steps (frequency encoders) or `01` fixed.
FT-450/450D use a simpler single-parameter form. Set-only. Not on FT-710/FTX-1.

### EK — ENT Key

FTDX3000/FT-991/991A/FT-891. Emulates the [ENT] keypad key. See model manual for P1 keycodes.

### EM — Encode Memory

FTDX3000/FTDX10/FTDX101. Stores RTTY/DATA message-memory text. `EM P1 P2 P3…P3 ;` — P1 `0`
RTTY/`1` DATA, P2 = channel `1`–`5`, P3 = up to 50 ASCII message characters. Read `EM P1 P2 ;`.

### EN — Encode

FTDX3000/FTDX10/FTDX101. Triggers transmission of an encode-memory channel. `EN P1 P2 ;`,
P1 `0` RTTY/`1` DATA, P2 = channel `1`–`5`. Set-only.

### EO — Encoder Offset

FTX-1 only. See FTX-1 manual.

### EU — Encoder Up

Companion to `ED`. `EU P1 P2 P2 ;`, same P1/P2 meanings. Set-only. Not on FT-710/FTX-1.

### EX — Menu

Reads/sets a menu item. **The menu chart is large and entirely model-specific** — the
FTDX10 alone has hundreds of items across a 3-level hierarchy — so only the message
structure is given here; consult each radio's "Table 2 (MENU Chart)" for item numbers and
value ranges.

- **Older form (FT-450/450D, FTDX3000, FT-991/991A, FT-891):** `EX P1 P1 P1 P2… ;` — P1 =
  3-digit menu item number, P2 = value.
- **Newer hierarchical form (FTDX10, FT-710, FTDX101, FTX-1):** `EX P1 P1 P2 P2 P3 P3 P4… ;`
  — P1 = menu group (01–05), P2 = sub-group (01–07), P3 = item (01–23), P4 = value
  (width per item, 1–12 digits/chars). Read form `EX P1 P1 P2 P2 P3 P3 ;`.

### FA — Frequency VFO-A

| Direction | Message | Notes |
|---|---|---|
| Set | `FA P1×9 ;` | P1 = 9-digit frequency in Hz, zero-padded |
| Read | `FA;` | |
| Answer | `FA P1×9 ;` | |

**Frequency field is 9 digits (Hz).** Range varies by model: FT-450D `30000`–`60000000`;
FTDX10 `000030000`–`075000000`; other models per their coverage. Example: 14.250000 MHz =
`FA014250000;`.

> **Quirk (general newcat):** a momentarily-busy radio may answer `FA;` with `?;`; treat as
> retriable, not fatal.

### FB — Frequency VFO-B

Identical layout to `FA` (9-digit Hz). Set `FB P1×9 ;`, Read `FB;`.

### FN — Fine Tuning

FTDX10/FT-710/FTDX101/FTX-1. `FN P1 ;`, P1 `0`: OFF · `1`: ON. Read `FN;`.

### FR — Function RX

FTDX3000/FTDX101/FTX-1. Selects the receive VFO/receiver. See model manual for P1 map
(MAIN/SUB/receiver selection).

### FS — Fast Step

FT-450/450D, FTDX3000, FT-991/991A, FT-891, FTDX101. `FS P1 ;`, P1 `0`: FAST key OFF · `1`:
ON. Read `FS;`.

### FT — Function TX

`FT P1 ;` (Set), Read `FT;`, Answer `FT P2 ;`. **Meaning differs:** FT-450/450D P1 `0`:
transmit displayed band · `1`: transmit opposite band. Modern (FTDX10): P1 `2`: MAIN
transmitter TX · `3`: SUB transmitter TX; Answer P2 `0`: MAIN TX · `1`: SUB TX. Not on
FT-891.

### GP — GP OUT A/B/C/D

FT-710/FTX-1. Controls the general-purpose output pins. See model manual for P-field map.

### GT — AGC Function

| Direction | Message | Notes |
|---|---|---|
| Set | `GT P1 P2 ;` | P1 `0` fixed |
| Read | `GT P1 ;` | |
| Answer | `GT P1 P3 ;` | |

**Value set differs.** FTDX10 Set P2: `0` OFF, `1` FAST, `2` MID, `3` SLOW, `4` AUTO;
Answer P3: `0` OFF, `1` FAST, `2` MID, `3` SLOW, `4` AUTO-FAST, `5` AUTO-MID, `6`
AUTO-SLOW. **FT-450/450D** has only `0` OFF, `1` FAST, `2` SLOW, `3` SLOW (no MID/AUTO).

### ID — Identification

Read/Answer only. `ID;` → `ID P1 P1 P1 P1 ;` (4-digit model ID). Known IDs:
FT-450 = `0241`, FT-450D = `0244`, FTDX3000 = `0460`, FT-991/991A = `0570`, FT-891 =
`0650`, FTDX10 = `0761`, FT-710 = `0800` (per FT-710 manual), FTDX101 = `0681`. (Software
uses `ID` to distinguish e.g. FT-450 vs FT-450D — see `PC` quirk.)

### IF — Information (VFO-A)

Read/Answer only. `IF;` → a fixed-width status block. **This is the primary "read
everything about the operating VFO" command** and its layout is consistent across the
modern family (FTDX10 shown):

| Pos | Field | Values |
|---|---|---|
| 1–2 | `IF` | command |
| 3–5 | P1 | Memory channel `001`–`099`, `P1L`–`P9U` (PMS), `5xx` (5 MHz band), `EMG` |
| 6–14 | P2 | VFO-A frequency, 9 digits (Hz) |
| 15–19 | P3 | Clarifier: sign `+`/`-` + offset `0000`–`9990` Hz (5 chars) |
| 20 | P4 | RX clarifier `0`/`1` |
| 21 | P5 | TX clarifier `0`/`1` |
| 22 | P6 | Mode (1-char, see §3 mode table) |
| 23 | P7 | `0` VFO, `1` Memory, `2` Memory Tune, `3` QMB, `5` PMS |
| 24 | P8 | CTCSS `0` OFF / `1` ENC-DEC / `2` ENC |
| 25–26 | P9 | `00` fixed |
| 27 | P10 | `0` Simplex / `1` +shift / `2` −shift |
| 28 | `;` | terminator |

**FT-450/450D differ:** P1 = `000`–`510` memory channel, clarifier offset `0000`–`9999`,
mode codes from the legacy table (§3), and an extra P9 tone-number field. Byte offsets are
otherwise analogous but confirm against the FT-450D manual.

### IS — IF-Shift

| Direction | Message | Notes |
|---|---|---|
| Set | `IS P1 P2 P3 P4 P4 P4 P4 ;` (modern) | |
| Read | `IS P1 ;` | |
| Answer | as Set | |

Modern (FTDX10): P1 `0` fixed, P2 `0` fixed, P3 = `+`/`-`, P4 = `0`–`1200` Hz (20 Hz steps,
4 digits). **FT-450/450D:** `IS P1 (-/+) P2 P2 P2 P2 ;` — the sign occupies position 4
directly and P2 = `0000`–`1000` Hz. (This is the manual's canonical "not enough
digits/padding" example.)

### KM — Keyer Memory

Stores CW keyer message text. Modern: `KM P1 P2…P2 ;`, P1 = channel `1`–`5`, P2 = up to 50
ASCII chars. **FT-450/450D:** P1 = `1`–`3` (beacon text channel), up to 40 chars. Read `KM P1 ;`.

### KP — Key Pitch

`KP P1 P1 ;`, Read `KP;`. **Encoding differs.** FTDX10/FT-710: P1 = `00`–`75` = 300–1050 Hz
(10 Hz steps). **FT-450/450D:** P1 = even codes `02`=400 Hz, `04`=500, `06`=600, `08`=700,
`10`=800 Hz (coarser).

### KR — Keyer (on/off)

`KR P1 ;`, P1 `0`: OFF · `1`: ON. Read `KR;`.

### KS — Key Speed

`KS P1 P1 P1 ;`, P1 = `004`–`060` WPM. Read `KS;`.

### KY — CW Keying

Set-only. `KY P1 ;` (playback of a stored message) or `KY {text};` on models that accept
direct-text CW keying. FTDX10 P1: `1`–`5` = keyer-memory 1–5 playback, `6`–`A` = message
keyer 1–5 playback. Front-panel manuals note a busy/buffer behavior — send subsequent `KY`
only after the buffer drains.

### LK — Lock

`LK P1 ;`, P1 `0`: dial lock OFF · `1`: ON. Read `LK;`.

### LM — Load Message (DVS record)

`LM P1 P2 ;`. P1 `0`: DVS. P2 `0`: recording stop, `1`–`5`: start/stop recording DVS
channel 1–5. Read `LM P1 ;`.

### MA — Memory Channel to VFO-A

Set-only (`MA;`). Copies the current memory channel into VFO-A. Not on FT-450/450D/FTX-1.

### MB — Memory Channel to VFO-B / Sub

Set-only (`MB;`). FTDX10/FT-710/FTDX101 only.

### MC — Memory Channel (select)

`MC P1 P1 P1 ;`, P1 = `001`–`099` memory channel, `P1L`–`P9U` (PMS), `5xx` (5 MHz band),
`EMG`. Read `MC;`. (FT-450/450D use `000`–`5xx` range per that manual.)

### MD — Operating Mode

| Direction | Message | Notes |
|---|---|---|
| Set | `MD P1 P2 ;` | |
| Read | `MD P1 ;` | |
| Answer | `MD P1 P2 ;` | |

Modern: P1 `0` MAIN / `1` SUB; P2 = mode code (§3 modern table). **FT-450/450D:** P1 `0`
fixed; P2 = legacy mode code (§3 legacy table). See the FT-991 C4FM quirk (§7) — the digital
voice mode is not exposed cleanly as a CAT mode token on that radio.

### MG — Mic Gain

`MG P1 P1 P1 ;`, Read `MG;`. **Scale differs.** FTDX10/FT-710: P1 = `000`–`100`.
**FT-450/450D:** P1 = `000`–`255` (banded: 000–085 = "L", 086–170 = "M", 171–255 = "H").
Note the FTDX101 MIC-GAIN 0–40 vs 0–100 reporting quirk (§7).

### MK — Mode Key

FT-450/450D only. `MK P1 ;`, P1 `7`: mode up, `8`: mode down, `9`: reverse (CW). Set-only.

### ML — Monitor Level

`ML P1 P2 P2 P2 ;`, Read `ML P1 ;`. P1 `0`: MONI ON/OFF, `1`: MONI level. When P1=0: P2
`000` OFF / `001` ON. When P1=1: P2 `000`–`100`. (FT-450/450D: only the ON/OFF form.)

### MR — Memory Channel Read

Read/Answer. `MR P0 P0 P0 ;` → a per-channel status block. The Answer layout mirrors `IF`
(channel number, 9-digit frequency, clarifier sign+offset, RX/TX clarifier, mode, VFO/mem
flag, CTCSS, shift) — see the `IF` table; `MR`'s P7 is `0` VFO / `1` Memory only. P0/P1 =
`001`–`099`, `P1L`–`P9U`, `5xx`, `EMG`.

### MS — Meter Switch

`MS P1 P2 ;`, Read `MS;`. P1 selects the metered quantity: `0` PO, `1` COMP, `2` ALC, `3`
VDD, `4` ID, `5` SWR. P2 `0` fixed. (Older radios expose fewer meter selections.)

### MT — Memory Channel Write/Tag

FT-991/991A, FTDX10, FT-710, FTDX101, FTX-1. Writes a memory channel *including its
alphanumeric tag*. Large fixed block (FTDX10 = 50 chars): channel + 9-digit freq +
clarifier + mode + flags + P12 = up to 12 ASCII tag characters. Read `MT P0 P0 P0 ;`. See
the FTDX10 manual for the full 50-position layout.

### MW — Memory Channel Write

`MW P1 P1 P1 P2×9 P3(5) P4 P5 P6 P7 P8 P9 P9 P10 ;` — writes a memory channel *without* a
tag. P1 = channel `001`–`099` / `P1L`–`P9U`; P2 = 9-digit frequency; P3 = clarifier sign +
offset; P4/P5 RX/TX clar; P6 = mode; P7 `0` fixed; P8 CTCSS; P9 `00` fixed; P10 shift.
Set-only. (Note the FT-891 "cannot write memory channels via CAT" Hamlib limitation — §7.)

### MX — MOX Set

FTDX3000, FT-991/991A, FT-891, FTDX10, FTDX101, FTX-1. `MX P1 ;`, P1 `0`: MOX OFF · `1`:
MOX ON (keys the transmitter). Read `MX;`.

### MZ — Split Memory

FTX-1 only. See FTX-1 manual. (Related to the FTX-1 split/Sub-VFO behavior — §7.)

### NA — Narrow

`NA P1 P2 ;`, Read `NA P1 ;`. P1 `0` fixed (modern adds MAIN/SUB on dual-RX models); P2 `0`:
OFF · `1`: ON (narrow filter). Not on FTX-1.

### NB — Noise Blanker Status

`NB P1 P2 ;`, Read `NB P1 ;`. P1 `0` fixed; P2 `0`: OFF · `1`: ON. Not on FTX-1.

### NL — Noise Blanker Level

FTDX3000+, FT-991/991A, FT-891, FTDX10, FT-710, FTDX101, FTX-1. `NL P1 P2 P2 P2 ;`, P1 `0`
fixed, P2 = `000`–`010`. Read `NL P1 ;`.

### NR — Noise Reduction (on/off)

`NR P1 P2 ;`, Read `NR P1 ;`. P1 `0` fixed; P2 `0`: OFF · `1`: ON. Not on FTX-1.

### OI — Opposite-Band Information

Read/Answer only. `OI;` → status block for VFO-B / the opposite band. Same field layout as
`IF` (channel, 9-digit VFO-B frequency, clarifier, mode, flags). P7 includes VFO/Memory/
Memory-Tune/QMB/PMS states.

### OS — Offset (Repeater Shift)

`OS P1 P2 ;`, Read `OS P1 ;`. P1 `0` MAIN / `1` SUB (modern) or `0` fixed (450). P2 `0`:
Simplex · `1`: +shift · `2`: −shift. Only active in FM mode.

### PA — Pre-Amp (IPO)

`PA P1 P2 ;`, Read `PA P1 ;`. P1 `0` fixed; P2 `0`: IPO (preamp off) · `1`: AMP 1 · `2`: AMP
2. (Some models offer only IPO / AMP.)

### PB — Play Back (DVS)

`PB P1 P2 ;`, Read `PB P1 ;`. P1 `0` fixed; P2 `0`: playback stop, `1`–`5`: play DVS channel
1–5.

### PC — Power (RF Output) Control

`PC P1 P1 P1 ;`, Read `PC;`. **Scale differs by model — a documented footgun:**

- FT-450D, FTDX10, FT-710, FT-991/991A, FT-891, FTX-1: `005`–`100` (watts).
- **FT-450 (non-D): `000`–`255`** (arbitrary scale) — *not* watts.
- **FTDX101MP: up to `200`** (200 W); FTDX101D: up to `100`.

> **Quirk:** Software that doesn't distinguish FT-450 vs FT-450D by their `ID` response
> (`0241` vs `0244`) will set/report the wrong power. The FTDX101MP/D differ by max value.
> The FT-710 has a reported CAT power-scaling read discrepancy in some Hamlib versions
> (reads ~32 W when running 100 W). (§7)

### PL — Speech Processor Level

FTDX3000+. `PL P1 P1 P1 ;`, P1 = `000`–`100`. Read `PL;`.

### PR — Speech Processor (on/off)

FTDX3000+. `PR P1 P2 ;`, Read `PR P1 ;`. P1 `0`: speech processor · `1`: parametric mic
equalizer. P2 `1`: OFF · `2`: ON.

> **Quirk (FTDX10):** `PR` is a *read/settable* status per the manual, but note a
> third-party app was found (mis)using undocumented `PR0;`/`PR1;` toggles — the documented
> form uses the P1/P2 pair above. (§7)

### PS — Power Switch

`PS P1 ;`, P1 `0`: OFF · `1`: ON. Read `PS;`. **To power ON:** send dummy data first, wait
~1 s, then send `PS1;` within 2 s. Over a true RS-232C connection the radio generally
**cannot** be powered on via CAT (USB only).

### QI — QMB Store

Set-only (`QI;`). Stores current state to the Quick Memory Bank.

### QR — QMB Recall

Set-only (`QR;`). Recalls from the Quick Memory Bank.

### QS — Quick Split

Set-only (`QS;`). Enables quick-split (TX VFO +offset). Not on FT-710/FTX-1.

### RA — RF Attenuator

`RA P1 P2 ;`, Read `RA P1 ;`. P1 `0` fixed; P2 `0`: OFF, `1`: 6 dB, `2`: 12 dB, `3`: 18 dB
(FTDX10). (Older/smaller radios expose fewer steps.)

### RC — Clarifier Clear

Set-only (`RC;`). Zeroes the clarifier offset. Not on FT-710/FTX-1.

### RD — Clarifier Down

`RD P1 P1 P1 P1 ;`, P1 = `0000`–`9990` Hz (sets clarifier to a downward offset). Set-only.
Not on FT-710/FTX-1.

### RF — Roofing Filter

FTDX3000/FTDX10/FTDX101. `RF P1 P2 ;`, Read `RF P1 ;`, Answer `RF P1 P3 ;`. P1 `0` fixed;
P2/P3 select filter: `1`/`6` = 12 kHz, `2`/`7` = 3 kHz, `4`/`9` = 500 Hz, `5`/`A` = 300 Hz
(option). Availability of the 300 Hz filter is model/option-dependent.

### RG — RF Gain

`RG P1 P2 P2 P2 ;`, Read `RG P1 ;`. P1 `0` fixed; P2 = `000`–`255`.

### RI — Radio Information

Read/Answer only. `RI P1 ;` → `RI P1 P2 ;`. P1 selects the item: `0` HI-SWR, `3` REC, `4`
PLAY, `D` "unable to transmit", etc.; P2 `0`: OFF · `1`: ON. (Item map is model-specific.)

### RL — Noise Reduction Level

`RL P1 P2 P2 ;`, Read `RL P1 ;`. P1 `0` fixed; P2 = `01`–`15` (DNR level).

### RM — Read Meter

Read/Answer only. `RM P1 ;` → `RM P1 P2 P2 P2 P3 P3 P3 ;`. P1 selects the meter: `1` S,
`3` COMP, `4` ALC, `5` PO, `6` SWR, `7` IDD, `8` VDD. P2 = `000`–`255` reading, P3 = `000`
fixed. (Not all meters on all models.)

### RO — Rotator

FTDX3000 only. Antenna-rotator control. See FTDX3000 manual.

### RP — Reset Power On

FT-450/450D only. Set-only (`RP;`). Reset-related.

### RS — Radio Status

Read/Answer only. `RS;` → `RS P1 ;`. P1 `0`: normal mode · `1`: menu mode. Not on
FT-710/FTX-1.

### RT — RX Clarifier (on/off)

`RT P1 ;`, Read `RT;`. P1 `0`: RX clarifier OFF · `1`: ON. Not on FT-891/FT-710/FTX-1
(which use the combined `CF`).

### RU — Clarifier Up

`RU P1 P1 P1 P1 ;`, P1 = `0000`–`9990` Hz (upward clarifier offset). Set-only. Not on
FT-710/FTX-1.

### SC — Scan

`SC P1 ;`, Read `SC;`. P1 `0`: scan OFF · `1`: scan ON (upward) · `2`: scan ON (downward).

### SD — CW Break-In Delay Time

`SD P1 P1 ;`, Read `SD;`. FTDX10 P1 = `00`–`33` mapped to 30/50/100/150/200/250 ms then
300 ms…3000 ms in 100 ms steps. **FT-450/450D** name it "Semi Break-In Delay Time" with a
model-specific code map.

### SF — Sub-Dial Function

FTDX3000/FTDX10/FT-710/FTDX101/FTX-1. `SF P1 P2 ;`, Read `SF P1 ;`. P1 `0` MPVD / `1` FUNC
knob; P2 = assigned function (large model-specific map — see manual).

### SH — Width (IF Bandwidth)

`SH P1 P2 P3 P3 ;` (modern) / `SH P1 P2 P2 ;` (450/450D). Read `SH P1 ;`. Modern (FTDX10):
P1 `0` fixed, P2 `0` fixed, P3 = `00`–`23` bandwidth index (per-mode chart: e.g. SSB 01=300
Hz…23=4000 Hz; CW/RTTY/PSK 01=50 Hz…21=4000 Hz). **FT-450/450D:** P2 = `00`–`31` (00–10
Narrow, 11–21 Normal, 22–31 Wide); Answer returns P3 with default indices 00/16/31.

> **Quirk:** On the FTDX101, AM/FM filter-width settings via `SH` reportedly have no audible
> effect. On the TS-570/K2/K3 lineage (not Yaesu) the width arg is ignored on mode-set —
> unrelated but a common cross-vendor confusion. (§7)

### SM — S-Meter Reading

Read/Answer only. `SM P1 ;` → `SM P1 P2 P2 P2 ;`. P1 `0` fixed (modern adds MAIN/SUB); P2 =
`000`–`255` S-meter value.

### SQ — Squelch Level

`SQ P1 P2 P2 P2 ;`, Read `SQ P1 ;`. P1 `0` fixed. **Range differs:** FTDX10 P2 = `000`–`100`;
FT-450/450D P2 = `000`–`255`.

### SS — Spectrum Scope

FTDX10/FT-710/FTDX101/FTX-1. `SS P1 P2 P3 P4 P5 P6 P7 ;` (Set), Read `SS P1 P2 ;`. P2
selects the scope attribute: `0` SPEED, `1` PEAK, `2` MARKER, `3` COLOR, `4` LEVEL, `5`
SPAN, `6` MODE, `7` AF-FFT/OSC, `8` HOLD — each with its own P3… sub-values (SPAN `0`=1 kHz…
`9`=1 MHz, etc.). Large model-specific map; see manual.

### ST — Split *(modern)* / Step *(FT-450/450D)*

**This mnemonic means two different things:**

- **Modern (FT-891, FTDX10, FT-710, FTDX101, FTX-1):** `ST` = **SPLIT**. `ST P1 ;`, Read
  `ST;`. P1 `0`: split OFF · `1`: split ON · `2`: split ON + 5 kHz up.
- **FT-450 / FT-450D:** `ST` = **STEP** (tuning step). `ST P1 ;`, Read `ST;`. P1 selects the
  per-mode tuning step (FM `0`=5 kHz…`7`=50 kHz; AM/SSB/CW have their own maps in the
  FT-450D manual).

Software must branch on model — sending `ST1;` to an FT-450D sets a *tuning step*, not
split. (FTDX3000/FT-991/991A do not list `ST` in their tables.)

### SV — Swap VFO

Set-only (`SV;`). Exchanges VFO-A and VFO-B contents.

### SY — Sync

FTDX101 only. `SY P1 ;` — VFO sync feature. See FTDX101 manual.

### TS — TXW (Transmit Watch)

`TS P1 ;`, Read `TS;`. P1 `0`: TXW OFF · `1`: ON. Not on FTDX101.

### TX — TX Set (PTT)

`TX P1 ;`, Read `TX;`. P1 `0`: RADIO TX OFF / CAT TX OFF · `1`: RADIO TX OFF / CAT TX ON
(keys via CAT) · `2`: RADIO TX ON / CAT TX OFF (Answer only — front-panel PTT active). This
is the standard CAT PTT command. (See FT-891/FT-857D "stuck in TX" anecdotes — §7.)

### UL — PLL Unlock Status

Read/Answer only. `UL;` → `UL P1 ;`. Reports PLL lock. Not on FTDX10/FT-710/FTX-1.

### UP — Mic Up / Up

Set-only (`UP;`). Steps the mic [UP] function.

### VD — VOX Delay Time

`VD P1 P1 P1 P1 ;` (FTDX10; 4-digit code) or shorter on older radios. Read `VD;`. FTDX10 P1
= `00`–`33` (30 ms…3000 ms). On FTDX10 the parameter is VOX-delay or DATA-VOX-delay
depending on the [VOX SELECT] menu setting.

### VE — Firmware Version

FT-710/FTX-1. Read/Answer only. `VE;` → firmware version string.

### VF — VRF Filter

FTDX3000 only. Variable RF front-end filter (µ-tune). See FTDX3000 manual.

### VG — VOX Gain

`VG P1 P1 P1 ;`, P1 = `000`–`100`. Read `VG;`.

### VM — [V/M] Key / VFO→Memory

Behaviour varies: on most modern radios `VM` emulates the [V/M] key (set-only, `VM;`); the
FTDX10 manual labels it "MAIN BAND TO MEMORY CHANNEL". Not on FT-450/450D.

### VR — Voice

FT-450/450D only. `VR P1 ;` — voice/DVS related. Read supported.

### VS — VFO Select

`VS P1 ;`, Read `VS;`. P1 `0`: VFO-A operation · `1`: VFO-B operation. Not on
FT-991/991A/FT-891.

### VT — VC Tune (µ-Tune)

FTDX101 only. See FTDX101 manual.

### VV — VFO to VFO

FT-450/450D only. Set (`VV;`) copies one VFO to the other.

### VX — VOX Status

`VX P1 ;`, Read `VX;`. P1 `0`: VOX OFF · `1`: VOX ON.

### XT — TX Clarifier

FTDX3000, FT-991/991A, FTDX10, FT-710, FTDX101. `XT P1 ;`, Read `XT;`. P1 `0`: TX clarifier
OFF · `1`: ON. Not on FT-891/FT-450/450D/FTX-1.

### ZI — Zero-In

FT-991/991A, FT-891, FTDX10, FT-710, FTDX101, FTX-1. Set-only (`ZI P1 ;`, P1 `0` fixed). CW
auto zero-in. Not on FT-450/450D/FTDX3000.

---

## 6. Binary protocol (FT-857D / FT-817ND)

The FT-857D and FT-817ND (and the related FT-817, FT-897, FT-100) do **not** use the ASCII
semicolon protocol. They use a fixed **5-byte binary command frame**. This set is small (17
opcodes) and is documented here separately.

**Frame format.** Every command is exactly **five bytes**, sent with up to 200 ms between
bytes. The **last** (5th) byte is the instruction **opcode**; the first four bytes are
arguments (parameters, or dummy padding when the opcode needs fewer). All values are
**hexadecimal**. Serial framing: 1 start bit, 8 data bits, no parity, 2 stop bits.

```
 DATA1   DATA2   DATA3   DATA4   DATA5
 [--------- arguments ---------] [opcode]
```

Example — set VFO to 439.70 MHz (opcode `01`, frequency packed BCD, 10 Hz resolution):
`43 97 00 00 01`. Example — Split ON (opcode `02`, dummy args): `00 00 00 00 02`.

### 6.1 Opcode command chart

Both radios expose 17 opcodes. Differences between the two models are noted in the last
column. "CMD" means the opcode byte itself carries the on/off variant (no separate args).

| Command | Args (DATA1–4) | Opcode (DATA5) | Values / notes |
|---|---|---|---|
| Lock ON/OFF | — | `00` / `80` | opcode `00`=Lock ON, `80`=Lock OFF |
| PTT ON/OFF | — | `08` / `88` | opcode `08`=PTT ON (TX), `88`=PTT OFF (RX) |
| Set Frequency | P1 P2 P3 P4 | `01` | BCD, 10 Hz resolution. `01 42 34 56 [01]` = 14.23456 MHz |
| Operating Mode | P1 | `07` | see mode table below |
| CLAR ON/OFF | — | `05` / `85` | `05`=CLAR ON, `85`=CLAR OFF |
| CLAR Frequency | P1 · P3 P4 | `F5` | P1 `00`=`+` offset, `≠00`=`−` offset; P3P4 = BCD kHz (`12 34`=12.34 kHz) |
| VFO A/B toggle | — | `81` | toggles active VFO |
| Split ON/OFF | — | `02` / `82` | `02`=Split ON, `82`=Split OFF |
| Repeater Shift dir. | P1 | `09` | P1 `09`=`−` shift, `49`=`+` shift, `89`=simplex |
| Repeater Offset freq | P1 P2 P3 P4 | `F9` | BCD frequency digits, `05 43 21 00 [F9]`=5.4321 MHz |
| CTCSS/DCS Mode | P1 | `0A` | see CTCSS/DCS mode table below |
| CTCSS Tone | P1 P2 (P3 P4) | `0B` | tone code(s), BCD ×0.1 Hz (see below) |
| DCS Code | P1 P2 (P3 P4) | `0C` | DCS code(s), BCD (see below) |
| Read RX Status | — | `E7` | returns 1 byte: S-meter + squelch/tone/discriminator flags |
| Read TX Status | — | `F7` | returns 1 byte: PO-meter + PTT/SWR/split flags |
| Read Freq & Mode | — | `03` | returns 5 bytes: 4 BCD freq + 1 mode byte |
| Power ON/OFF | — | `0F` / `8F` | **FT-817ND only** (`0F`=ON, `8F`=OFF); not on FT-857D |

**Model differences in the chart:**
- **Mode opcode `07` values.** FT-857D: `00` LSB, `01` USB, `02` CW, `03` CWR, `04` AM,
  `08` FM, `88` FM-N, `0A` DIG, `0C` PKT. **FT-817ND** is the same but **omits FM-N (`88`)**:
  `00` LSB, `01` USB, `02` CW, `03` CWR, `04` AM, `08` FM, `0A` DIG, `0C` PKT.
- **CTCSS/DCS Mode opcode `0A`.** FT-857D: `0A` DCS ON, `0B` DCS DEC ON, `0C` DCS ENC ON,
  `2A` CTCSS ON, `3A` CTCSS DEC ON, `4A` CTCSS ENC ON, `8A` OFF (separate encoder/decoder
  states). **FT-817ND** is reduced: `0A` DCS ON, `2A` CTCSS ON, `4A` ENC ON, `8A` OFF.
- **CTCSS Tone (`0B`) / DCS Code (`0C`).** FT-857D takes **four** arg bytes = separate TX
  and RX tones/codes (P1P2 = TX, P3P4 = RX). **FT-817ND** takes **two** arg bytes = a single
  tone/code for both TX and RX. Tone example: `08 85` = 88.5 Hz. DCS example: `00 23` = 023.
- **Power ON/OFF (`0F`/`8F`).** FT-817ND only. Its manual warns: do not use on alkaline/NiMH
  battery power, and send a 5-byte all-`00` dummy frame first before the power-on frame.

### 6.2 Read-status response layouts

**Read RX Status (`E7`)** — returns **one byte**:

| Bits | Field | Meaning |
|---|---|---|
| 0–3 | S-meter | S-meter reading |
| 4 | Dummy | — |
| 5 | Discriminator | `0` centered, `1` off-center (FM); `0` in SSB/CW/AM |
| 6 | CTCSS/DCS | `0` code matched, `1` unmatched; `0` when tone off |
| 7 | Squelch | `0` squelch off = signal present, `1` = no signal |

**Read TX Status (`F7`)** — returns **one byte**:

| Bits | Field | Meaning |
|---|---|---|
| 0–3 | PO-meter | power-output reading |
| 4 | Dummy | — |
| 5 | Split | `0` split ON, `1` split OFF |
| 6 | HI-SWR | `0` off, `1` high SWR |
| 7 | PTT | `0` PTT ON (TX), `1` PTT OFF (RX) |

**Read Frequency & Mode Status (`03`)** — returns **five bytes**: DATA1–4 = frequency in
packed BCD (10 Hz resolution, e.g. `43 97 00 00` = 439.7000 MHz), DATA5 = mode byte
(`00` LSB, `01` USB, `02` CW, `03` CWR, `82` CW-N, `04` AM, `06` WFM, `08` FM, `88` NFM,
`0A` DIG, `0C` PKT on the FT-857D).

> **Quirk (FT-817ND):** with the optional CW narrow filter installed and ON, the mode byte
> in the `03` response returns values with the high bit set (`0x82`/`0x83`/`0x8A`) instead
> of the documented `0x02`/`0x03`/`0x0A`. Naive parsers that mask on exact values misread
> the mode. (§7)

> **Quirk (FT-857D):** the rig replies to any **unrecognized** command with a single `0x00`
> ACK byte rather than an error — undocumented, and CAT software has to account for it. It
> is also a documented "VFO-swapping" rig (active VFO can flip A/B under polling). (§7)

---

## 7. Known quirks & gotchas (Yaesu)

Cross-referenced from `quirks.md` (sources are cited there). These are real-world
implementation/interoperability issues, not manual-documented behavior.

**FT-450 / FT-450D**
- `PC` power scale differs by model: FT-450 = `000`–`255` (arbitrary), FT-450D = `005`–`100`
  (watts). Distinguish by `ID` (`0241` vs `0244`) or you set/report the wrong power.
- Hardware (RTS/DTR) flow control on the serial port can be interpreted as a key-down,
  keying the radio immediately on connect.

**FT-710**
- CAT mode-change interacts with the "CW FREQ DISPLAY = PITCH OFFSET" setting — tuned
  frequency can end up shifted rather than exactly on a spotted frequency.
- Switching USB → DATA-U via CAT has triggered a serial error requiring reconnection.
- Power reported over CAT mis-scaled in some Hamlib versions (read ~32 W at 100 W).
- Historically best controlled with the FTDX10 profile (FT-710 backend was incomplete).

**FTDX10**
- `PR` (speech processor) is a read/status query with a P1/P2 pair; some apps wrongly send
  bare `PR0;`/`PR1;` as toggles.

**FTX-1**
- The standard split-enable (`ST1;`-style) forces the Sub VFO into a locked TX state (TX LED
  lit) and freezes its frequency — breaks satellite Doppler correction on the Sub/uplink VFO.

**FT-991 / FT-991A**
- CAT-rate menu (item 031) must match the software's baud rate exactly or commands lag/drop.
- The digital voice mode is C4FM but only "AMS" was historically exposed; setting it via CAT
  could return corrupted status (`Mode: None`) on the next query.
- Hamlib 3.3 had spurious CAT timeouts on this radio that didn't occur in FLDigi.

**FT-891**
- Needs a ~50 ms delay after each CAT write; back-to-back fast writes cause problems
  (Hamlib inserts this delay deliberately).
- Memory-channel contents cannot be written via CAT in Hamlib (read/tune only).
- Anecdotal reports of the radio sticking in TX during a CAT "Test PTT" (not reproduced).

**FTDX101MP / FTDX101D**
- FM is not exposed as its own mode (only PKT-FM); AM/FM filter-width (`SH`) settings have no
  audible effect; MIC GAIN reports on a 0–40 scale instead of 0–100%; COMP/MONITOR/LOCK
  don't reliably toggle via CAT.
- `PC` max differs: MP = 200 W, D = 100 W.

**FTDX3000**
- Band change via CAT (e.g. from WSJT-X) can drop RX audio until RX/VFO-A is reselected.
- Antenna/band selection is per-VFO, so the "current" antenna reported in split mode can be
  the wrong one unless both VFOs are set explicitly.
- Not always listed as a profile in loggers; FTDX-5000 profile is used as a substitute.

**FT-857D**
- Spontaneous VFO A/B toggling under CAT polling ("VFO-swapping rig").
- Replies to unrecognized commands with a single `0x00` byte (undocumented).

**FT-817ND**
- CW-narrow filter ON changes the `03` status mode byte to high-bit-set values
  (`0x82`/`0x83`/`0x8A`) vs the documented `0x02`/`0x03`/`0x0A`.

**General (newcat family: FT-991, FT-891, FTDX10, FTDX101, FT-710, etc.)**
- A momentarily-busy rig returns `?;`. This is retriable ("try again"), but the shared
  Hamlib "newcat" backend has treated it as a permanent rejection, failing `IF;`/`FA;`/`NA0;`
  queries outright instead of retrying.

---

## Coverage & Methodology Notes (Yaesu)

**Method.** All 12 Yaesu manuals were converted to text with `pdftotext -layout` and read
directly. The ASCII master matrix (§4) was built programmatically by extracting each
model's control-command-table headers (the 2-letter mnemonic + name that heads each
command's layout table), then cross-tabulated; it was spot-checked against the human-readable
"CAT Control Command List" pages. Per-command layouts (§5) were transcribed from the
**FTDX10** manual as the canonical modern reference (its 101-command table is the most
complete of the family), with legacy/variant differences pulled from the **FT-450D** manual
and, where flagged, other models. The binary protocol (§6) was transcribed from the FT-857D
and FT-817ND operating-manual CAT chapters.

**Fully analyzed (CAT command tables transcribed or matrixed):**
- FT-450 (dedicated CAT reference, 18 pp) — matrixed.
- FT-450D (dedicated CAT OM, 18 pp) — fully transcribed as the legacy reference.
- FTDX10 (dedicated CAT OM, 25 pp) — fully transcribed as the canonical modern reference.
- FT-710 (25 pp), FTX-1 (29 pp), FT-991 (20 pp), FT-991A (20 pp), FT-891 (20 pp),
  FTDX101MP/D (26 pp), FTDX3000 (19 pp) — command lists and tables analyzed and matrixed;
  per-command differences noted where they diverge from the FTDX10 canonical layout.
- FT-857D (CAT chapter within the 136-pp OM), FT-817ND (CAT chapter within the 84-pp OM) —
  binary opcode charts and status-byte layouts fully transcribed.

**Partially analyzed / scoped out:**
- **`EX` (MENU) parameter tables** were *not* transcribed item-by-item for any model. Each
  radio's menu chart has dozens-to-hundreds of items (the FTDX10 "Table 2" spans several
  pages); only the `EX` *message structure* is documented here, with a pointer to each
  manual's menu chart. Same approach for the large per-attribute maps of `SS` (spectrum
  scope), `SF` (sub-dial), `EX`, and the model-specific `AN`/`FR`/`GP`/`SY`/`VT` commands.
- Full 50-position character grids for `MT`/`MW`/`MR`/`IF`/`OI` are summarized (field list +
  key offsets) rather than drawn cell-by-cell; the `IF` grid is given in full as the
  representative example.

**Data-quality caveats:**
- A few auto-extracted command *names* in the master matrix reflect PDF text-flow artifacts
  (e.g. `VM` picked up an adjacent function name); these are corrected in the per-command
  prose. Presence (`Y`/`—`) is reliable; treat the one-line matrix name as secondary to the
  per-command heading.
- Set/Read/Answer capability is stated per command in §5; it is uniform across the family
  for the vast majority of commands. Where a model omits a direction, it usually omits the
  whole command.
- The FT-991 and FT-991A CAT sets are treated as identical (their manuals match); any true
  divergence would be firmware-level and is not separately documented here.
- The older `K3S&K3&KX3 Pgmrs Ref, F2.pdf` is an Elecraft file and is handled in
  `cat_elecraft.md`, not here.

**Total commands documented (Yaesu):** **120 distinct ASCII command mnemonics** across the
10 ASCII radios (see §4/§5), plus **17 binary opcodes** for the FT-857D/FT-817ND (§6).
