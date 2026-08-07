# Kenwood PC-Control ("CAT") Command Reference

> **Scope of this file.** This document covers the **Kenwood** family of computer-control
> commands (Kenwood's own name is "PC Control Command", often lumped under the generic
> "CAT" label). Yaesu, Kenwood, and Elecraft/Lab599 each publish a *completely separate,
> non-interoperable* protocol — they share only the marketing umbrella "CAT". Yaesu is in
> `cat_yaesu.md`; Elecraft/Lab599 in `cat_elecraft.md`.
>
> **Important:** although Kenwood *also* uses 2-letter, semicolon-terminated ASCII commands
> and even reuses some mnemonics that Yaesu uses (`FA`, `MD`, `IF`, `AI`, `SM`…), the field
> layouts, parameter encodings, and value maps are **different from Yaesu's**. Do not treat
> a Kenwood `FA`/`IF`/`MD` as interchangeable with the Yaesu one. The Elecraft K-line and
> the Lab599 TX-500 are *derived from* the Kenwood command style (see `cat_elecraft.md`),
> which makes Kenwood the closest cross-vendor relative — but even there the sets diverge.
>
> Radios covered: TS-480 (HX/SAT), TS-570, TS-590S/SG (documented as "TS-590G"),
> TS-890S, TS-990S, TS-2000.

---

## 1. Protocol fundamentals

A PC control command is: **2-letter command** + **parameters** + **terminator (`;`)**.
Commands are handled as printable ASCII. Three message directions:

- **Set** — computer → radio (e.g. `FA00014250000;`).
- **Read** — computer → radio (e.g. `FA;`).
- **Answer** — radio → computer (e.g. `FA00014250000;`).

**Serial framing.** 1 start bit, 8 data bits, 1 stop bit, no parity — **except at 4800 bps,
which requires 2 stop bits** (stated explicitly in the TS-480 reference). Baud rates are
model/menu-selectable (4800/9600/19200/38400/57600/115200 depending on model). RTS/CTS
hardware handshaking is used on the 9-pin COM port and is often *required* for reliable
operation (see quirks, §6). Newer radios add a USB virtual COM port and (TS-890S/TS-990S)
a LAN/KNS network path that carries the same command set.

**Parameter rules.**
- Fixed-width, zero-padded numeric fields, same discipline as Yaesu (`IS_+_1000` example is
  in the Kenwood manuals too). Non-applicable digits should be filled with `0` (or any
  non-control, non-`;` char).
- `Pn` denotes a parameter; a parameter repeated across positions spans those characters.

**Auto-Information (`AI`).** Kenwood defines multiple AI *levels*, unlike Yaesu's simple
on/off:
- **TS-480 / older:** `AI0` off, `AI1` "old AI format" on, `AI2` "extended AI format" on,
  `AI3` both on. With old-AI on, the radio sends an `IF;` report ~every 1.5 s when state
  changes; with extended-AI on it auto-sends the relevant command. Resets to `0` at power-off.
- **Newer radios (TS-590/890/990):** `AI0`/`AI1`/`AI2` (off / on / extended), same idea.

---

## 2. Mode codes (`MD` / `IF` mode field)

The Kenwood mode field is a single digit. Base map (all radios):

| Code | Mode | Code | Mode |
|---|---|---|---|
| 1 | LSB | 6 | FSK (RTTY) |
| 2 | USB | 7 | CW-R (CW reverse) |
| 3 | CW | 8 | (none / setting failure) |
| 4 | FM | 9 | FSK-R (RTTY reverse) |
| 5 | AM | | |

`0` and `8` are returned as "None (setting failure)". **Newer radios (TS-890S, TS-990S)**
keep this map but add a separate **data/PSK sub-mode** selection (via the `DA`/`DV`/`DC`
data commands and a data-mode byte) rather than adding new base `MD` codes — always check
the specific model's `MD` and data-mode commands. The TS-2000 additionally uses this map on
both its main and sub receivers.

---

## 3. Master command support matrix

`Y` = the command has a documented PC-control table in that model's manual; `—` = not
found in that model's tables. Column order roughly oldest → newest. Presence was extracted
programmatically from each manual's command tables and cross-checked; see coverage notes
(§7) for accuracy caveats (notably TS-570, whose older OM format under-reports).

| Cmd | 570 | 480 | 2000 | 590G | 890S | 990S | Function (Kenwood) |
|-----|:---:|:---:|:----:|:----:|:----:|:----:|--------------------|
| AC | Y | Y | Y | Y | Y | Y | Antenna tuner control |
| AG | Y | Y | Y | Y | Y | Y | AF gain |
| AI | — | Y | Y | Y | Y | Y | Auto information |
| AL | — | — | Y | — | — | — | Auto notch level (TS-2000) |
| AM | — | — | Y | — | Y | Y | Auto mode / AMC |
| AN | — | Y | Y | Y | Y | — | Antenna connector select |
| AR | — | — | Y | — | — | — | (TS-2000) |
| AS | — | Y | Y | Y | — | — | Auto mode set (band/mode table) |
| BC | Y | Y | Y | Y | Y | Y | Beat canceller |
| BD | — | Y | Y | — | — | — | Band down |
| BI | — | — | — | — | Y | Y | Break-in (newer) |
| BK | — | — | — | — | Y | — | (TS-890S) |
| BP | — | — | Y | Y | Y | Y | Manual beat-canceller / notch position |
| BU | — | Y | Y | — | — | — | Band up |
| BY | Y | Y | Y | Y | Y | Y | Busy status |
| CA | Y | Y | Y | Y | Y | Y | CW auto zero-beat |
| CB | — | — | — | — | — | Y | (TS-990S) |
| CG | — | — | Y | Y | Y | Y | Carrier gain / CW rise |
| CH | — | Y | Y | Y | Y | Y | Channel/step up-down |
| CI | — | — | Y | — | — | — | (TS-2000) call/CI |
| CM | — | — | Y | — | — | — | (TS-2000) |
| CN | Y | Y | Y | Y | Y | Y | CTCSS number |
| CP | — | — | — | — | Y | — | Speech compressor (TS-890S) |
| CT | Y | Y | Y | Y | — | — | CTCSS on/off |
| DA | — | — | — | Y | — | — | (TS-590 data) |
| DC | — | — | Y | — | — | — | (TS-2000) |
| DF | — | — | — | — | Y | Y | Data / dual-freq |
| DL | Y | — | — | — | — | — | (TS-570) |
| DN | — | Y | Y | — | — | — | Mic down |
| DP | — | — | — | — | — | Y | (TS-990S) |
| DQ | — | — | Y | — | — | — | (TS-2000) DCS |
| DV | — | — | — | — | Y | Y | Data VOX / data mode |
| EC | — | — | — | — | Y | Y | (echo / newer) |
| EM | — | — | — | Y | Y | Y | (encode memory) |
| EQ | — | — | — | Y | — | — | Equalizer (TS-590) |
| ES | — | — | — | Y | — | — | (TS-590) |
| EX | Y | Y | Y | Y | Y | Y | Menu (extended) |
| FA | Y | Y | Y | Y | Y | Y | Frequency VFO A |
| FB | Y | Y | Y | Y | Y | Y | Frequency VFO B |
| FC | — | — | Y | — | Y | Y | Sub / fine freq |
| FD | — | — | Y | — | — | — | (TS-2000) |
| FL | — | — | — | Y | — | — | Filter (TS-590) |
| FR | — | Y | Y | — | Y | — | Receive VFO/function |
| FS | — | Y | Y | Y | Y | Y | Fine step / fine tuning |
| FT | — | Y | Y | — | Y | — | Transmit VFO/function |
| FV | — | — | — | Y | Y | Y | Firmware version |
| FW | Y | Y | Y | Y | Y | Y | Filter width |
| GC | — | — | — | Y | Y | Y | AGC constant |
| GT | Y | Y | Y | Y | Y | Y | AGC time / function |
| ID | Y | Y | Y | Y | Y | Y | Identification |
| IF | Y | Y | Y | Y | — | — | Information (status block) |
| IS | Y | Y | Y | Y | Y | — | IF shift |
| KS | — | Y | Y | Y | Y | Y | Keyer speed |
| KY | Y | Y | Y | Y | Y | Y | CW keyboard keying |
| LK | Y | Y | Y | Y | Y | Y | Lock |
| LM | Y | Y | Y | Y | Y | Y | (message / list) |
| LT | — | — | Y | — | — | — | (TS-2000) |
| MC | Y | Y | Y | Y | — | — | Memory channel select |
| MD | Y | Y | Y | Y | — | — | Operating mode |
| MF | — | Y | Y | Y | Y | Y | Menu A/B select |
| MG | Y | Y | Y | Y | Y | Y | Mic gain |
| MH | — | — | — | — | Y | — | (TS-890S) |
| MI | — | — | — | — | Y | Y | Memory (newer) |
| MK | — | — | — | Y | Y | Y | Mode key |
| ML | — | Y | Y | Y | Y | Y | Monitor level |
| MN | — | — | — | — | Y | Y | Memory name |
| MO | — | — | Y | — | — | — | (TS-2000) |
| MR | Y | Y | Y | Y | — | — | Memory read |
| MS | — | — | — | — | Y | Y | Memory scroll (newer) |
| MT | — | — | — | — | Y | Y | Memory tag (newer) |
| MU | — | — | Y | — | Y | Y | Multi/menu |
| MV | — | — | — | — | Y | Y | (newer) |
| MW | — | Y | Y | Y | — | — | Memory write |
| NB | Y | Y | Y | Y | — | — | Noise blanker |
| ND | — | — | — | — | — | Y | (TS-990S) |
| NL | — | Y | Y | Y | — | — | Noise blanker level |
| NR | Y | Y | Y | Y | Y | Y | Noise reduction |
| NS | — | — | — | — | — | Y | (TS-990S) |
| NT | — | — | Y | Y | Y | Y | Notch (auto) |
| NW | — | — | — | — | Y | Y | Notch width (newer) |
| OF | — | — | Y | — | — | — | (TS-2000) |
| OI | — | — | Y | — | — | — | Opposite-band info (TS-2000) |
| OM | — | — | — | — | Y | Y | Operating-mode extended (newer) |
| OP | — | Y | — | — | — | — | (TS-480) |
| OS | — | — | Y | — | — | — | Offset (TS-2000) |
| PA | — | Y | Y | Y | Y | Y | Pre-amp |
| PB | Y | Y | Y | Y | — | — | Playback (DRV/voice) |
| PC | Y | Y | Y | Y | Y | Y | Output power |
| PI | — | — | Y | — | — | — | (TS-2000) |
| PK | — | — | Y | — | — | — | (TS-2000) |
| PL | — | Y | Y | Y | Y | Y | Speech processor level |
| PM | — | — | Y | — | — | — | Programmable memory (TS-2000) |
| PR | Y | Y | Y | Y | — | — | Speech processor on/off |
| PS | Y | Y | Y | Y | Y | Y | Power switch |
| PT | Y | — | — | — | Y | Y | Pitch / polarity |
| QA | — | — | — | — | Y | Y | Quick memory (newer) |
| QC | — | — | Y | — | — | — | DCS code (TS-2000) |
| QD | — | — | — | Y | Y | Y | Quick data |
| QI | — | Y | Y | Y | Y | Y | Quick memory store |
| QR | — | Y | Y | Y | Y | Y | Quick memory recall |
| QS | — | — | — | — | Y | — | Quick split (TS-890S) |
| RA | Y | Y | Y | Y | Y | Y | RF attenuator |
| RC | — | Y | Y | Y | Y | Y | RIT clear |
| RD | — | Y | Y | — | — | — | RIT down |
| RE | — | — | — | — | Y | Y | (newer) |
| RF | — | — | — | — | Y | Y | RF (newer) |
| RG | — | Y | Y | Y | Y | Y | RF gain |
| RI | — | — | — | Y | — | — | (TS-590) |
| RL | — | Y | Y | Y | — | — | Noise-reduction level |
| RM | — | Y | Y | Y | Y | Y | Meter (read meter / SWR) |
| RS | — | Y | — | — | — | — | (TS-480) |
| RT | — | Y | — | Y | Y | Y | RIT on/off |
| RU | — | Y | Y | — | — | — | RIT up |
| RX | — | Y | Y | Y | Y | Y | Receive (unkey) |
| SA | — | — | Y | — | — | — | Satellite (TS-2000) |
| SB | — | — | Y | — | — | Y | Sub-band |
| SC | Y | Y | Y | Y | — | — | Scan |
| SD | Y | Y | Y | Y | Y | Y | CW break-in delay |
| SF | — | — | — | — | Y | — | (TS-890S) |
| SH | Y | Y | Y | — | Y | Y | Filter high-cut / shift |
| SI | — | — | Y | — | — | — | (TS-2000) |
| SL | Y | Y | Y | — | Y | Y | Filter low-cut |
| SM | Y | Y | Y | Y | Y | Y | S-meter reading |
| SP | — | — | — | Y | Y | Y | Split (newer) |
| SQ | — | Y | Y | Y | Y | Y | Squelch level |
| SR | — | Y | Y | Y | Y | Y | Reset |
| SS | — | Y | Y | Y | Y | Y | (scan/settings) |
| ST | — | Y | Y | — | — | — | Step (multi-ch) |
| SU | — | Y | Y | Y | Y | Y | Scan/step up |
| SV | — | Y | Y | Y | Y | Y | (save / swap) |
| TB | — | — | — | — | Y | Y | (newer) |
| TC | — | — | Y | — | — | — | (TS-2000) |
| TD | — | — | Y | — | — | — | (TS-2000) |
| TI | — | — | Y | — | Y | — | (TNC / info) |
| TN | — | Y | Y | Y | Y | Y | CTCSS/tone number |
| TO | — | Y | Y | Y | Y | Y | Tone on/off |
| TP | — | — | — | Y | — | — | (TS-590) |
| TR | — | — | Y | — | — | — | (TS-2000) |
| TS | — | Y | Y | Y | Y | Y | TF-Set / split-key |
| TX | — | Y | Y | Y | Y | Y | Transmit (key) |
| TY | — | Y | Y | — | — | — | (firmware type) |
| UD | — | — | — | — | Y | — | (TS-890S) |
| UL | — | Y | Y | — | — | — | PLL unlock status |
| UP | — | Y | Y | — | — | — | Mic up |
| VD | Y | Y | Y | Y | Y | Y | VOX delay |
| VG | Y | Y | Y | Y | — | — | VOX gain |
| VR | Y | Y | Y | Y | — | — | (voice) |
| VV | — | Y | — | Y | Y | Y | VFO=VFO (A=B) |
| VX | Y | Y | Y | Y | Y | Y | VOX on/off |
| XI | — | — | — | Y | — | — | (TS-590) |
| XO | — | Y | — | Y | Y | Y | Transverter offset |
| XT | Y | Y | Y | Y | Y | Y | XIT on/off |
| XV | — | — | — | — | Y | Y | Transverter (newer) |

> **Matrix caveats.** Entries marked with a bare "(model)" note are commands whose 2-letter
> mnemonic was found in that model's tables but whose exact function is model-specific;
> confirm against the model manual. The TS-570 column is under-populated because its OM's
> PC-control section uses an older table format that the extractor parsed less completely —
> treat TS-570 "—" cells as "not confirmed present" rather than "definitely absent". Some
> single-model codes may be extraction artifacts; the per-command sections (§4/§5) document
> the well-established commands authoritatively.

---

## 4. Core command reference

Layouts are Kenwood's character-position form (canonical: TS-590SG / TS-480, which share
the classic layout; TS-890S/TS-990S extend it). `Pn` = one character position unless a width
is given; numeric fields are zero-padded. `;` terminates.

### FA / FB — Frequency VFO A / VFO B

| Direction | Message | Notes |
|---|---|---|
| Set | `FA` + P1(11) + `;` | P1 = **11-digit** frequency in Hz |
| Read | `FA;` | |
| Answer | `FA` + P1(11) + `;` | |

**Kenwood frequency is 11 digits (Hz)** — e.g. 14.195000 MHz = `FA00014195000;`. This is
wider than Yaesu's 9-digit field; blank leading digits must be `0`. `FB` is identical for
VFO B. (TS-2000/TS-890S/TS-990S also use 11 digits.)

### MD — Operating Mode

| Direction | Message | Notes |
|---|---|---|
| Set | `MD P1 ;` | P1 = mode code (§2) |
| Read | `MD;` | |
| Answer | `MD P1 ;` | |

P1: `1` LSB, `2` USB, `3` CW, `4` FM, `5` AM, `6` FSK, `7` CW-R, `9` FSK-R; `0`/`8` = "None
(setting failure)". Data/PSK sub-mode on TS-890S/TS-990S is handled by the data commands,
not new `MD` codes.

### IF — Transceiver Status (read)

Read/Answer only. `IF;` → a fixed-width status block. **This is the primary "read the whole
operating state" command.** Canonical layout (TS-590SG):

| Pos | Field | Meaning |
|---|---|---|
| 1–2 | `IF` | command |
| 3–13 | P1 | 11-digit displayed frequency (Hz) |
| 14–18 | P2 | 5 spaces (reserved) |
| 19–23 | P3 | RIT/XIT frequency, signed, ±9990 Hz |
| 24 | P4 | RIT `0` OFF / `1` ON |
| 25 | P5 | XIT `0` OFF / `1` ON |
| 26–28 | P6,P7 | Memory channel number (see `MC`) |
| 29 | P8 | `0` RX / `1` TX |
| 30 | P9 | Operating mode (see `MD`) |
| 31 | P10 | Function (see `FR`/`FT`) |
| 32 | P11 | Scan status (see `SC`) |
| 33 | P12 | `0` simplex / `1` split |
| 34 | P13 | `0` OFF / `1` Tone / `2` CTCSS / `3` Cross-tone |
| 35–36 | P14 | Tone/CTCSS number `00`–`42` (see `TN`/`CN`) |
| 37 | P15 | `0` always |
| 38 | `;` | terminator |

Notes: with AI on, an `IF` response is auto-sent when RIT/XIT or memory frequency changes.
**`IF` cannot read status while the radio is in Data mode** (per the TS-590 manual). The
TS-890S/TS-990S do **not** list `IF` — they use richer per-item reads instead (matrix `—`).

### ID — Identification

Read/Answer only. `ID;` → `ID P1 P1 P1 ;` (3-digit ID). Known: TS-590S = `021`,
TS-590SG = `023`, TS-480 = `020`, TS-2000 = `019`, TS-570 = `018` (per manuals). Software
uses `ID` to distinguish models (e.g. TS-590S vs SG differ in some ranges).

### AC — Antenna Tuner Control

`AC P1 P2 P3 ;`, Read `AC;`. P1 `0` RX-AT thru / `1` RX-AT in; P2 `0` TX-AT thru / `1`
TX-AT in; P3 `0` stop tuning / `1` start tuning (Answer: `1` = tuning active).

### AG — AF Gain

`AG P1 P2 P2 P2 ;`, Read `AG P1 ;`. P1 `0` (main; TS-2000 uses `0`/`1` for main/sub); P2 =
`000`–`255`.

### AN — Antenna Connector

`AN P1 ;`, Read `AN;`. P1 `1` ANT1 / `2` ANT2 (models with the switch). TS-890S/TS-990S add
more antenna options — see model manual.

### PC — Output Power

`PC P1 P1 P1 ;`, Read `PC;`. P1 = 3-digit watts. **Ranges are mode-dependent:** SSB/CW/FM/
FSK `005`–`100`; AM `005`–`025`. Step is 5 W unless "Power Fine" is on (then 1 W; off-fine
values round down to the nearest 5). TS-890S HF = 5–100 W; the 100 W-class radios differ
from the 200 W TS-990S — check the model.

### SM — S-Meter Reading

Read only. `SM P1 ;` → `SM P1 P2 P2 P2 P2 ;`. P1 `0` (main) — TS-2000/dual-RX use `0`/`1`.
**P2 = `0000`–`0030`** (dot count), *not* 0–255 as Yaesu uses. Reads S-meter on RX,
RF-power meter on TX. (See the TS-590 S-meter calibration quirk, §6.)

### RM — Read Meter

`RM P1 ;` → `RM P1 P2 P2 P2 P2 ;`. P1 selects: `0` none, `1` SWR, `2` COMP, `3` ALC. P2 =
`0000`–`0030` dots. The TS-590 returns three response types (SWR, COMP, ALC).

### GT — AGC (Time Constant)

`GT P1 P1 ;`, Read `GT;`. P1 = `01`–`20` (AGC time-constant step). **Cannot be read if AGC
is OFF or in FM mode** (error tone). (Contrast Yaesu's `GT`, which is an AGC fast/mid/slow
selector — same mnemonic, different semantics.)

### FR — Receive VFO / Function

`FR P1 ;`, Read `FR;`. P1 `0` VFO A, `1` VFO B, `2` M.CH (memory). When P1=2, `FT` becomes
invalid. Selects which VFO/memory the *receiver* uses.

### FT — Transmit VFO / Function

`FT P1 ;`, Read `FT;`. Same P1 map as `FR` (`0` VFO A / `1` VFO B / `2` M.CH). **Split is
set by making `FR` and `FT` select different VFOs** (e.g. `FR0;FT1;` = RX on A, TX on B). On
newer radios the `SP`/`TS` commands also participate.

### FS — Fine Tuning

`FS P1 ;`, Read `FS;`. P1 `0` OFF / `1` ON.

### TX — Transmit (key)

`TX P1 ;`, Read → Answer `TX P2 ;`. **P1 selects the TX source:** `0` Normal (SEND) using
**MIC** input, `1` DTS/data transmission using the **ANI** (accessory/line) input, `2` TX
Tune. If P1 omitted, P1=0.

> **Quirk:** Hamlib historically keyed with `TX0;` (or `TX;`), which selects the **MIC**
> input and therefore **mutes the ACC2/USB line-in audio** used for digital modes — the
> radio keys but sends no audio. Use `TX1;` for data/line-in TX. Also note a Hamlib 4.6.5
> regression where the TS-480 stopped replying to the TX command, causing PTT timeouts. (§6)

### RX — Receive (unkey)

`RX;` — returns the radio to receive. No parameters (a response is emitted only when AI is
active).

### SP — Split Operation Frequency (newer)

TS-590 (fw ≥ 2.00), TS-890S, TS-990S. `SP P1 ;` (start/complete) or `SP P1 P2 P3 ;` (set),
Read `SP;`. P1 `0` no-op/complete, `1` setting-in-progress/start, `2` cancel. P2 shift
direction `0` plus / `1` minus. P3 shift value `1`–`9` kHz. (Split LED flashes during setup.)

### TS — TF-Set / Split Key

`TS P1 ;`, Read `TS;`. Emulates the TF-SET key (temporarily swaps RX/TX VFOs to check/set
the split TX frequency). P1 `0` OFF / `1` ON (behavior model-specific).

### FW — Filter Width (DSP bandwidth)

`FW P1 P1 P1 P1 ;`, Read `FW;`. P1 = 4-digit width in Hz, from a **per-mode discrete list**:
CW `0050/0080/0100/0150/0200/0300/0400/0500/0600/1000/2000`; FSK `0250/0500/1000/1500`;
SSB/FM/AM `0000` Normal / `0001` NAR / `0002` NAR2. Use `SL`/`SH` to change slope-tune
frequencies. (TS-890S/TS-990S restructure filter control; `FW` may be absent — use `SH`/`SL`.)

### SH — Filter High-Cut / Slope

`SH P1 P1 ;`, Read `SH;`. P1 = high-cut slope-tune index (per-mode table in the model
manual). Present on TS-570/480/2000/890S/990S.

### SL — Filter Low-Cut / Slope

`SL P1 P1 ;`, Read `SL;`. P1 = low-cut slope-tune index. Companion to `SH`.

### IS — IF Shift

`IS P1 P2 P2 P2 P2 ;` (TS-480/570) or `IS P1… ;` (DSP filter shift on TS-590). Read `IS;`.
On the TS-590 it is "DSP Filter Shift" with a signed Hz value. Encoding varies by model —
confirm the sign/width in the model manual.

### RT / XT — RIT / XIT On-Off

`RT P1 ;` (P1 `0` OFF / `1` ON), Read `RT;`. `XT P1 ;` similarly for XIT. RIT/XIT offset is
read via `IF` (positions 19–23) and cleared with `RC`; stepped with `RD`/`RU`.

### RC / RD / RU — RIT/XIT Clear / Down / Up

`RC;` clears the RIT/XIT offset to 0. `RD P1… ;` / `RU P1… ;` step the offset down/up
(older radios use a step count; some take an explicit Hz value). Present on TS-480/2000/590.

### SQ — Squelch Level

`SQ P1 P2 P2 P2 ;`, Read `SQ P1 ;`. P1 `0` (main; `0`/`1` on dual-RX). P2 = `000`–`255`.

### PS — Power Switch

`PS P1 ;`, Read `PS;`. P1 `0` OFF / `1` ON. As with Yaesu, powering **on** over CAT has
constraints (the radio must be listening; USB/DTR behavior is model-specific).

### PL — Speech Processor Level

`PL P1 P1 P1 P2 P2 P2 ;`, Read `PL;`. P1 = input level `000`–`100`, P2 = output level
`000`–`100`.

### PR — Speech Processor On/Off

`PR P1 ;`, Read `PR;`. P1 `0` OFF / `1` ON (TS-480/570/2000/590). Absent on TS-890S/TS-990S
tables (restructured — see `CP` compressor on TS-890S).

### PA — Pre-Amp

`PA P1 ;`, Read `PA;`. P1 `0` OFF / `1` ON (models with a switchable preamp; TS-2000 uses
`0`/`1` per receiver).

### RA — RF Attenuator

`RA P1 P1 ;`, Read `RA;`. P1 = attenuator step (`00` off, then model-specific dB steps).

### RG — RF Gain

`RG P1 P1 P1 ;`, Read `RG;`. P1 = `000`–`255`.

### MC — Memory Channel Select

`MC P1 P1 P1 ;`, Read `MC;`. P1 = 3-digit memory channel number (range model-specific; e.g.
`000`–`099` plus special channels). TS-890S/TS-990S restructure memory access (matrix `—`
for `MC`/`MR`/`MD`/`IF` on those — they use newer per-item commands).

### MR — Memory Channel Read

`MR P1 P2 P2 P2 ;` → returns the channel's stored frequency/mode/flags block. P1 selects the
data set; P2 = channel number. See model manual for the exact answer layout (analogous to
`IF`).

### MW — Memory Channel Write

`MW P1 P2 P2 P2 <data> ;` writes a memory channel. Large fixed block (channel number + 11-
digit frequency + mode + tone + flags). Present on TS-480/570/2000/590.

### TN / CN — Tone / CTCSS Number

`TN P1 P1 ;` sets the repeater **tone** (encode) frequency number; `CN P1 P1 ;` sets the
**CTCSS** number. Both index a 00–42 tone table (Kenwood's standard 42-tone chart). Read
`TN;` / `CN;`.

### TO / CT — Tone / CTCSS On-Off

`TO P1 ;` enables the encode tone; `CT P1 ;` enables CTCSS (encode+decode). P1 `0` OFF /
`1` ON. (`CT` present on TS-570/480/2000/590; newer radios fold tone control into other
commands.)

### BC — Beat Canceller (auto notch)

`BC P1 ;`, Read `BC;`. P1 `0` OFF, `1` BC1 (auto), `2` BC2. Kenwood's auto-notch/beat-cancel.

### BP — Manual Beat Canceller / Notch Position

`BP P1… ;` sets the manual notch position (TS-2000/590/890/990). Encoding model-specific.

### NB / NL — Noise Blanker / Level

`NB P1 ;` (P1 `0` OFF / `1` ON), Read `NB;`. `NL P1 P1 P1 ;` sets the NB level
(`000`–…). Present on TS-570/480/2000/590; TS-890S/TS-990S restructure (matrix `—`).

### NR — Noise Reduction

`NR P1 ;`, Read `NR;`. P1 `0` OFF, `1` NR1, `2` NR2.

> **Quirk (TS-480SAT / TS-590S):** NR **mode 2 (NR2) cannot be *set* via CAT** — the radio
> reads back NR2 if selected manually, but issuing the set command for level 2 fails; levels
> 0 and 1 work both directions. (§6)

### NT — Auto Notch

`NT P1 ;`, Read `NT;`. P1 `0` OFF / `1` ON (DSP auto-notch, on TS-2000/590/890/990).

### VX / VG / VD — VOX On-Off / Gain / Delay

`VX P1 ;` (P1 `0` OFF / `1` ON). `VG P1 P1 P1 ;` VOX gain (`000`–…). `VD P1 P1 P1 P1 ;`
VOX delay. Read forms drop the value.

### KY — CW Keyboard Keying

`KY P1 <text> ;` sends CW text from the buffer. P1 is a buffer/immediate flag; the manual's
character table lists the allowed characters. Used for CW-over-CAT. Read form reports buffer
availability.

### KS — Keyer Speed

`KS P1 P1 P1 ;`, Read `KS;`. P1 = WPM (e.g. `004`–`060`, model-specific).

### SD — CW Break-In Delay

`SD P1 P1 P1 P1 ;`, Read `SD;`. P1 = break-in delay time (ms, model-specific range).

### CA — CW Auto Zero-Beat

`CA P1 ;`, Read `CA;`. P1 `0` OFF/cancel / `1` ON/activate (CW auto zero-beat/spot).

### AI — Auto Information

`AI P1 ;`, Read `AI;`. P1 levels per §1 (`0`/`1`/`2`/`3` on TS-480; `0`/`1`/`2` on newer).
Controls unsolicited status reporting.

### EX — Menu (Extended Settings)

`EX P1P1P1 P2 P3 P4 <data> ;` — reads/sets a menu item. **Every model's menu map is
different and large**; only the structure is documented here. Typical form: 3-digit menu
number + additional selectors + value. Consult the model's menu list for item numbers and
ranges. (TS-890S/TS-990S have hundreds of menu items.)

### FV / TY — Firmware Version / Type

`FV;` (TS-590/890/990) reads firmware version. `TY;` (TS-480) reads the firmware **type**:
Answer `TY P1 P1 P2 ;` where P2 `0` TS-480HX (200 W), `1` TS-480SAT (100 W + AT), `2`
Japanese 50 W, `3` Japanese 20 W.

### UL — PLL Unlock Status

Read/Answer. `UL;` → `UL P1 ;`. P1 `0` locked / `1` unlocked. (TS-480/2000.)

### UP / DN — Mic Up / Down

`UP P1 P1 ;` / `DN;` emulate the mic UP/DOWN keys. On TS-480, `UP` with no parameter = 1
step; in memory mode with no parameter it steps memory channels, with a parameter it steps
frequency.

---

## 5. Notable cross-command notes

- **`FA`/`FB` are 11 digits** (Hz) on all Kenwood models — a hard difference from Yaesu's 9.
- **`SM`/`RM` return dot counts `0000`–`0030`**, not a 0–255 raw value.
- **Split = `FR`/`FT` mismatch** on classic models; `SP`/`TS` on newer ones.
- **`TX` P1 chooses MIC vs line/ANI input** — the single most common digital-mode pitfall.
- **TS-890S / TS-990S drop the old monolithic `IF`, `MC`, `MR`, `MD`, `NB`** style in favor
  of many finer-grained commands; do not assume a TS-590 program works unmodified on them.

---

## 6. Known quirks & gotchas (Kenwood)

Cross-referenced from `quirks.md` (sources cited there).

**TS-480**
- Hamlib 4.6.5 regression: the TX command stopped getting any reply, so every PTT attempt
  hit the 500 ms timeout and reported a comms error; 4.6.4 worked. (Fixed later.)
- `TX0;` PTT selects MIC input and **mutes ACC2/USB line-in audio** used for digital modes.
- CAT memory writes to a TS-480HX/SAT could silently force the **SPLIT flag** on stored
  channels even when both frequencies were identical.

**TS-570**
- Hamlib's `kenwood_set_mode()` **ignores the width/filter argument** for the TS-570 (also
  K2/K3), so requesting a filter width on mode-set has no effect — set width separately.
- Reliable CAT reportedly **requires RTS/CTS hardware handshake** enabled.

**TS-590 (S / SG)**
- Same `TX0;` MIC-vs-line-in muting issue as the TS-480.
- ATT/PREAMP level get/set reported broken through the Hamlib Python bindings.
- **S-meter (`SM`) readback fails on both S and SG** — the `SM` command is identical between
  them, but Hamlib lacked a calibration curve for either (a capability-table gap, not a
  radio difference).

**TS-890S**
- If **VFO B is active when Hamlib connects**, `rig_set_split_freq_mode()` calls always
  change VFO B even when the TX VFO should be A. Starting from VFO A avoids it.

**TS-2000**
- Intermittent timeouts commonly fixed by enabling **hardware (RTS/CTS) handshake**
  (e.g. `-C serial_handshake=Hardware`).
- Hamlib's TS-2000 backend carries special-case code to tolerate SDR "TS-2000 emulation"
  quirks (real Kenwoods don't reply to some set commands the way emulators do).

**General Kenwood (TS-480SAT / TS-590S confirmed)**
- **NR2 can be read but not set** via CAT (`U VFOA NR 2`-style set fails); NR0/NR1 work both
  directions.

**TS-990S** — no specific, citable CAT quirk found (see `quirks.md`).

---

## Coverage & Methodology Notes (Kenwood)

**Method.** All six manuals were converted with `pdftotext -layout` and read directly.
Dedicated PC-command references (TS-480, TS-590G, TS-890S, TS-990S) were used in full;
the TS-570 and TS-2000 command sets live inside their full operating manuals (89 pp and
152 pp respectively) and only the PC-control command tables were used. The master matrix
(§3) was built programmatically by detecting each command's message rows (the spaced
two-letter code lines) in every manual, then cross-tabulating.

**Fully analyzed:**
- TS-480 (dedicated PC ref, 26 pp) — command tables fully parsed; several layouts
  transcribed as the classic-layout reference.
- TS-590G (TS-590S/SG PC ref, 32 pp) — command tables fully parsed; primary canonical
  source for modern layouts (`FA`, `IF`, `SM`, `PC`, `RM`, `GT`, `SP`, etc.).
- TS-890S (77 pp) and TS-990S (66 pp) dedicated PC references — command presence parsed
  and matrixed; these radios use a substantially larger/finer command set.
- TS-2000 (PC-control section within the 152-pp OM) — command tables parsed and matrixed.
- TS-570 (PC-control section within the 89-pp OM) — parsed and matrixed (see caveat below).

**Partially analyzed / scoped out:**
- **`EX` (menu) item tables** were not transcribed per-item for any model (each is a
  model-specific menu map, dozens-to-hundreds of items). Only the `EX` message structure is
  documented.
- Full answer-block character grids for `MR`/`MW` and the newer TS-890S/TS-990S memory and
  scope commands are summarized (field list) rather than drawn cell-by-cell. The `IF` grid
  is given in full as the representative example.
- Many TS-890S/TS-990S-only and TS-2000-only mnemonics appear in the matrix with a
  "(model)" placeholder function — presence is confirmed but the exact semantics were not
  individually transcribed; the model manual is the authority for those.

**Data-quality caveats:**
- **TS-570 is under-reported.** Its older OM table format parsed less completely, so several
  TS-570 "—" cells mean "not confirmed by the extractor," not "definitely absent." The
  TS-570 genuinely has a smaller command set than the modern radios, but treat its column as
  a lower bound.
- The TS-590G `FA`/`FB` cells were initially mis-parsed (the manual documents them as a
  combined "FA / FB" block) and were corrected by hand to `Y`.
- A few single-model 2-letter codes may be extraction artifacts (parameter fragments that
  matched the message-row pattern). The per-command sections (§4) document the
  well-established commands authoritatively; treat obscure single-model matrix entries as
  "seen in the text, verify in the manual."

**Total commands documented (Kenwood):** **153 distinct 2-letter mnemonics** across the six
radios in the master matrix (§3), with ~55 of the operationally important commands given
full per-command layouts and notes (§4).
