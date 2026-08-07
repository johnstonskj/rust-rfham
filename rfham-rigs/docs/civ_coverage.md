# Icom CI-V Computer-Control Protocol — Technical Reference

A unified command reference for the Icom Communications Interface V (CI-V) protocol,
built by directly transcribing and cross-referencing the official CI-V / control-command
manuals for the following radios:

| Radio           | Type                                  | Manual filename                           |
|-----------------|---------------------------------------|-------------------------------------------|
| IC-705          | HF/VHF/UHF all-mode portable (D-STAR) | IC-705_ENG_CI-V_1_20200721.pdf            |
| IC-905          | VHF/UHF/SHF all-mode (D-STAR)         | IC-905_ENG_CI-V_2.pdf                     |
| IC-9700         | VHF/UHF/1.2 GHz all-mode (D-STAR)     | IC-9700_ENG_CI-V_1.pdf                    |
| IC-R8600        | Wideband communications receiver      | IC-R8600_ENG_CI-V_3a.pdf                  |
| ID-50A / ID-50E | 2 m/70 cm D-STAR handheld             | ID-50A_ID-50E_ENG_CI-V_1.pdf              |
| IC-7610         | HF/50 MHz SDR transceiver             | IC-7610_ENG_CI-V_2021.pdf                 |
| IC-7100         | HF/VHF/UHF all-mode (D-STAR)          | IC-7100_ENG_Advanced_ControlCommand.pdf   |
| IC-7300         | HF/50 MHz SDR transceiver             | IC-7300_ENG_Advanced_CIV.pdf              |
| IC-7000         | HF/VHF/UHF all-mode                   | IC-7000_ENG_Manual_ControlCommand.pdf     |
| IC-7851         | HF/50 MHz flagship transceiver        | IC-7851_ENG_Manual_ControlCommand.pdf     |

Legacy radios (IC-706, IC-7800, IC-756 family, etc.) are cross-referenced from the
`icom_civ_reference_v32_2002.pdf` general reference where noted; see the
*Coverage & Methodology Notes* section at the end for the limits of that source.

Known per-radio quirks are pulled inline from the companion `quirks.md` and flagged
with **Quirk:**.

---

## 1. Shared CI-V frame structure

Unlike Yaesu/Kenwood CAT (ASCII text, `;`-terminated), Icom CI-V is a genuine single
binary protocol shared across the whole Icom line. Every message — command or reply —
is a byte sequence with the same envelope:

```text
FE FE <to-addr> <from-addr> <Cn> [<Sc>] [<data …>] FD
```

| Field               | Bytes       | Meaning                                                                                                                                                                                                   |
|---------------------|-------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Preamble            | `FE FE`     | Fixed start-of-message flag (two bytes).                                                                                                                                                                  |
| To address          | 1 byte      | CI-V address of the destination device.                                                                                                                                                                   |
| From address        | 1 byte      | CI-V address of the sender.                                                                                                                                                                               |
| Command (`Cn`)      | 1 byte      | Top-level command number.                                                                                                                                                                                 |
| Sub-command (`Sc`)  | 0–2 bytes   | Present only for commands that define sub-commands (e.g. `07`, `0E`, `14`, `15`, `16`, `19`, `1A`, `1B`, `1C`, `1E`, `1F`, `20`–`28`). Some commands (notably `1A 05`) use a **2-byte** BCD sub-command.  |
| Data area           | 0..n bytes  | Command payload — BCD digits, binary bytes, or ASCII, depending on the command.                                                                                                                           |
| Terminator          | `FD`        | Fixed end-of-message flag.                                                                                                                                                                                |

### Controller ↔ radio addressing

- The **controller** (PC) conventionally uses address `E0`. (`0E` is also seen on some
  Icom software; `E1` is used for a second controller.)
- The **radio** uses its own default CI-V address (see §2).
- Address `00` is a **broadcast**: a message addressed to `00` is accepted by every
  device on the bus (used by the transceive broadcasts).

A controller-to-radio frame therefore looks like `FE FE <radio> E0 …  FD`, and the
radio's reply comes back as `FE FE E0 <radio> … FD`.

### Acknowledgement replies

For a *set* command the radio answers with a fixed status frame instead of data:

| Reply     | Bytes                     | Meaning                                         |
|-----------|---------------------------|-------------------------------------------------|
| OK / ACK  | `FE FE E0 <radio> FB FD`  | Command accepted.                               |
| NG / NAK  | `FE FE E0 <radio> FA FD`  | Command rejected (bad value, wrong mode, etc.). |

*Read* commands echo the command/sub-command back followed by the requested data and `FD`.

### CI-V Transceive (unsolicited broadcasts)

With **CI-V Transceive** enabled (the factory default on most modern radios), the radio
spontaneously broadcasts frequency (`00`) and mode (`01`) changes to address `00`
whenever the operator turns the dial or changes mode. This lets a second radio or logger
track it without polling. It is also the single most common source of CI-V trouble — see
the cross-model quirks in §3 notes and `quirks.md`.

> **Quirk (cross-model):** With Transceive left on, the radio's own broadcasts can collide
> on the bus with commands from control software, producing "protocol error"/"command
> rejected" symptoms (reported on IC-705, IC-7100 over USB, and generally). The standard
> fix is to disable Transceive and have the software poll. On USB-CI-V radios (IC-7100,
> IC-7300), "CI-V USB Echo Back" usually has to be turned **on** as well.

### Data encoding conventions

- **Frequency and other numeric data are BCD** (binary-coded decimal), **little-endian by
  byte** — the least-significant pair of digits comes first. Each byte holds two decimal
  digits (high nibble = the higher digit). See command `00`/`03`/`05` for the exact
  5-byte frequency layout.
- **Level data** (command `14`, meters `15`) is a **2-byte BCD** value `0000`–`0255`
  (a 0–255 scale carried as four decimal digits).
- **On/off and enumerated settings** (command `16`, most `1A 05` registers) are **1-byte
  binary** codes (`00`=off, `01`=on, etc.).
- **Sub-command `Sc` for `1A 05`** is a **2-byte BCD register number** (`0000`–`0xxx`).
- **ASCII** is used for call signs, memory names, messages and CW/keyer text (see the
  character-code tables under command `1A 00` / `17`).
- Where a digit position is marked "fixed" in a manual, that nibble must be sent as the
  stated constant (e.g. the 1 GHz digit on HF radios).

---

## 2. Default CI-V address table

Every radio of a given model ships with the **same** factory-default address; all of them
are user-changeable in the radio's Set/Menu mode (Connectors → CI-V → CI-V Address).

| Radio           | Default CI-V address (hex)  | User-changeable?  | Source                                  |
|-----------------|:---------------------------:|:-----------------:|-----------------------------------------|
| IC-705          | `A4`                        | Yes               | IC-705 CI-V manual, data-format diagram |
| IC-905          | `AC`                        | Yes               | IC-905 CI-V manual                      |
| IC-9700         | `A2`                        | Yes               | IC-9700 CI-V manual                     |
| IC-R8600        | `96`                        | Yes               | IC-R8600 CI-V manual                    |
| ID-50A / ID-50E | `AE`                        | Yes               | ID-50 CI-V manual                       |
| IC-7610         | `98`                        | Yes               | IC-7610 CI-V manual                     |
| IC-7100         | `88`                        | Yes               | IC-7100 Advanced manual                 |
| IC-7300         | `94`                        | Yes               | IC-7300 Advanced manual                 |
| IC-7000         | `70`                        | Yes               | IC-7000 manual                          |
| IC-7851         | `8E`                        | Yes               | IC-7851 manual                          |
| Controller (PC) | `E0`                        | —                 | Convention (all manuals)                |
| Broadcast       | `00`                        | —                 | All-devices target                      |

**Legacy default addresses — verified from the v3.2 general reference (Table 2-2):**

| Radio           | Addr | Radio            | Addr | Radio            | Addr | Radio          | Addr |
|-----------------|:----:|------------------|:----:|------------------|:----:|----------------|:----:|
| IC-735          | `04` | IC-R71A/E/D      | `1A` | IC-725           | `28` | IC-728         | `38` |
| IC-R7000        | `08` | IC-751A          | `1C` | IC-R9000         | `2A` | IC-729         | `3A` |
| IC-275A/E/H     | `10` | IC-761           | `1E` | IC-765           | `2C` | IC-737         | `3C` |
| IC-375A         | `12` | IC-271A/E/H      | `20` | IC-970A/E/H      | `2E` |                |      |
| IC-475A/E/H     | `14` | IC-471A/E/H      | `22` | IC-726           | `30` |                |      |
| IC-575A/H       | `16` | IC-1271A/E       | `24` | IC-R72           | `32` |                |      |
| IC-1275A/E      | `18` | IC-781           | `26` | IC-R7100         | `34` |                |      |

The v3.2 reference also carries an addendum for newer-at-the-time models
(IC-706/706MkII/706MkIIG, IC-707, IC-718, IC-746/746PRO, IC-756/756PRO/756PROII,
IC-820H, IC-821H, IC-910H, IC-R10, IC-R8500, IC-703, IC-7800) whose addresses live in the
model-specific appendices rather than Table 2-2. Community-cited values for those (not
re-verified byte-for-byte here): IC-706 `48`, IC-706MkIIG `58`, IC-718 `5E`, IC-746 `56`,
IC-746PRO `66`, IC-756PROII `64`, IC-7800 `6A`, IC-703 `68`, IC-R8500 `4A`, IC-910H `60`.

**Reserved addresses (must NOT be assigned to a radio):** `00`, `E0`, and `F0`–`FF` are
reserved for the controller and system functions; the assignable range is `01`–`7F`
(with per-model restrictions).

> **Framing note — frequency data length is not universal:** Most radios use a **5-byte**
> BCD frequency field, but the **IC-735 uses a fixed 4-byte** field, and a few
> (IC-R71, IC-R72, IC-R7100, IC-R9000) are switchable between 4-byte and 5-byte. Software
> that hard-codes 5 bytes will misparse an IC-735 (v3.2 Table 2-5).

> **Quirk (cross-model):** Because two radios of the same model share a default address,
> putting two identical rigs on one CI-V bus requires manually reassigning one radio's
> address first, or the controller cannot address them individually.

---

## 3. Command reference

The support matrix in each section covers the ten primary radios, abbreviated:
**705** (IC-705), **905** (IC-905), **9700** (IC-9700), **R86** (IC-R8600),
**ID50** (ID-50A/E), **7610** (IC-7610), **7100** (IC-7100), **7300** (IC-7300),
**7000** (IC-7000), **7851** (IC-7851). Cell values: **Y** = supported,
**—** = not supported / not documented, **(n)** = supported, see note *n*.

The IC-R8600 is a **receive-only** radio: it does not implement any transmit-related
command (RF power, TX bandwidth, VOX, break-in, PTT, split-transmit, DV TX, etc.).
The ID-50A/E is a **D-STAR handheld**: it has the full DV/GPS command set but omits
HF-transceiver features (PBT, notch, band-scope waveform, TX passband, etc.).

---

### 00 — Send Operating Frequency (Transceive)

Writes the operating frequency to the radio; also the form the radio **broadcasts**
unsolicited when Transceive is on. Same 5-byte BCD frequency payload as command `05`
(set) and `03` (read).

**Byte layout (10-byte frame):**

| Byte #  | Value             | Description                                                 |
|:-------:|:-----------------:|-------------------------------------------------------------|
| 0       | `FE`              | Preamble                                                    |
| 1       | `FE`              | Preamble                                                    |
| 2       | `<to>`            | To address (radio, or `00` for a broadcast)                 |
| 3       | `<from>`          | From address (controller `E0`, or radio when broadcasting)  |
| 4       | `00`              | Command                                                     |
| 5       | 10 Hz · 1 Hz      | BCD, least-significant pair first                           |
| 6       | 1 kHz · 100 Hz    | BCD                                                         |
| 7       | 100 kHz · 10 kHz  | BCD                                                         |
| 8       | 10 MHz · 1 MHz    | BCD                                                         |
| 9       | 1 GHz · 100 MHz   | BCD (GHz digit fixed on HF radios)                          |
| 10      | `FD`              | Terminator                                                  |

Frequency = concatenation of the BCD digits, e.g. **145.500.00 MHz** →
`00 50 45 00 45 01` for bytes 5–9 (`00`=00, `50`=1kHz/100Hz, …). Unit is Hz; the lowest
digit is 1 Hz on radios with 1 Hz tuning (705/905/9700/7610/7300/7851/R8600) and
effectively 10 Hz on older/handheld models.

**Wider-frequency radios:** The IC-905 reaches into the SHF (up to 10 GHz +
transverter). Icom keeps the classic 5-byte field but the manuals add a paired
**5-byte high-order extension** on some 905/9700 frequency commands for the GHz range;
where a radio exceeds the 5-byte range the manual documents the extra bytes in that
radio's frequency-format page. Standard HF/VHF/UHF radios use exactly 5 data bytes.

> **Quirk (IC-905):** Manually retuning the VFO on the radio can cause the frequency read
> back over CI-V to drop leading digits (e.g. 10386.100.000 read as 386.099.000),
> producing false out-of-band errors in logging/WSJT-X.

**Support:** universal — supported on all radios.

| Cmd/Sub         | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|-----------------|:---:|:---:|:----:|:---:|:----:|:----:|:----:|:----:|:----:|:----:|
| `00` Send freq  | Y   | Y   | Y    | Y   | Y    | Y    | Y    | Y    | Y    | Y    |

---

### 01 — Send Operating Mode (Transceive)

Writes (and broadcasts) the operating mode + filter. 1–2 data bytes.

**Byte layout:**

| Byte #  | Value               | Description                                                                                                                     |
|:-------:|:-------------------:|---------------------------------------------------------------------------------------------------------------------------------|
| 0–3     | `FE FE <to> <from>` | Envelope                                                                                                                        |
| 4       | `01`                | Command                                                                                                                         |
| 5       | mode code           | `00`=LSB, `01`=USB, `02`=AM, `03`=CW, `04`=RTTY, `05`=FM, `06`=WFM, `07`=CW-R, `08`=RTTY-R, `17`=DV (DV only on D-STAR radios)  |
| 6       | filter              | `01`=FIL1, `02`=FIL2, `03`=FIL3 — **optional**; if omitted, FIL1 is selected                                                    |
| 7       | `FD`                | Terminator                                                                                                                      |

Mode codes are shared across the whole line. Radios without D-STAR (IC-7610, IC-7300,
IC-7000, IC-7851) do not accept `17`=DV. The IC-R8600 adds wide-band receive modes
(WFM, S-AM/synchronous-AM variants) documented in its own mode-code table.

**Support:** universal.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `01` Send mode | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 02 — Read Band Edge Frequencies

Read-only. Returns the lower and higher band-edge frequencies for the currently selected
band (and, on radios with programmable band edges, an edge-number index).

**Data format** (per IC-705 "Band edge frequency settings", commands `02`, `1E 01`,
`1E 03`): a 1-byte **edge number** (`01`–`30`), then a 5-byte BCD lower-edge frequency,
a fixed **`2D`** separator, then a 5-byte BCD higher-edge frequency. When reading with
`02`, the edge-number byte is **not** returned.

| Byte # | Value | Description |
|:--:|:--:|-------------|
| 0–4 | `FE FE <to> <from> 02` | Envelope + command |
| 5–9 | lower edge (5-byte BCD) | Same digit order as command `00` |
| 10 | `2D` | Separator (fixed) |
| 11–15 | higher edge (5-byte BCD) | |
| 16 | `FD` | Terminator |

**Support:** documented on 705/905/9700/R86/7610/7100/7300/7851 (read band edges).
The IC-7000 and ID-50 use the simpler band-edge behavior; `1E 01`/`1E 03` (TX band edges)
are the transmit-side variants — see §command `1E`.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `02` Read band edges | Y | Y | Y | Y | (1) | Y | Y | Y | (1) | Y |

*Note 1:* Present but with a reduced/legacy edge set on IC-7000; on ID-50 the band-edge
read reflects the handheld's fixed amateur/GENE bands.

---

### 03 — Read Operating Frequency

Read-only request; the radio replies with the same 5-byte BCD frequency payload as
command `00`. The request frame carries no data (`FE FE <to> <from> 03 FD`).

**Support:** universal.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `03` Read freq | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 04 — Read Operating Mode

Read-only; reply carries the mode + filter payload of command `01`.

**Support:** universal.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `04` Read mode | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 05 — Set Operating Frequency

Set (write) form of the frequency; identical 5-byte BCD payload as command `00`. Unlike
`00`, `05` is a directed set command and does not participate in transceive broadcast.

**Support:** universal.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `05` Set freq | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 06 — Set Operating Mode

Set form of the mode; same payload as command `01`. With `06`, if the filter byte is
omitted the radio auto-selects the default filter for that mode.

**Support:** universal.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `06` Set mode | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 07 — Select VFO Mode (sub-command)

Selects VFO mode and manipulates the VFOs. Sub-command byte(s):

| Sub | Data | Description |
|:--:|:--:|-------------|
| *(none)* | — | Select VFO mode |
| `00` | — | Select VFO A |
| `01` | — | Select VFO B |
| `A0` | — | Equalize VFO A ↔ VFO B (copy) |
| `B0` | — | Exchange VFO A ↔ VFO B |
| `D0` | — | Select MAIN band (dual-receiver radios) |
| `D1` | — | Select SUB band (dual-receiver radios) |

On dual-receiver radios (IC-9700, IC-7610, IC-7851, IC-905) the `D0`/`D1` sub-commands
select Main vs Sub. `A0`/`B0` return NG in Memory/Call mode when split is off.

> **Quirk (IC-9700):** Doppler-correction software repeatedly sending frequency updates
> can make Main/Sub VFOs swap and swap back, and can toggle SPLIT unexpectedly.

> **Quirk (IC-7851):** Some Hamlib driver versions do not declare a VFO B for the 7851,
> so split/`07 B0`-style operations fail through that path even though the radio supports
> them.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `07` Select VFO | Y | Y | Y | Y | (2) | Y | Y | Y | Y | Y |
| `07 00/01` A/B | Y | Y | Y | Y | (2) | Y | Y | Y | Y | Y |
| `07 A0/B0` eq/xchg | Y | Y | Y | (3) | (2) | Y | Y | Y | Y | Y |
| `07 D0/D1` Main/Sub | — | Y | Y | Y | — | Y | — | — | — | Y |

*Note 2:* ID-50 is single-VFO handheld; `07` selects VFO (A/B) but has no dual-band
Main/Sub. *Note 3:* R8600 is a receiver with VFO A/B; equalize/exchange apply to RX VFOs.

---

### 08 — Select Memory Mode / Memory Channel

Selects the Memory mode and (with data) a memory channel or group.

| Sub / Data | Description |
|:--:|-------------|
| *(none)* | Select Memory mode |
| `0000`–`0099` | Select memory channel (4-digit BCD) |
| `A0` `0000`–`0100` | Select memory group (radios with memory groups) |

Call-channel encodings vary per radio (e.g. IC-705: `0000`=144C1, `0001`=144C2,
`0002`=430C1, `0003`=430C2). Memory-channel count differs: 99 (705/9700/7300…),
up to 99+call and D-STAR banks on DV radios, and R8600 has 100 ch × banks.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `08` Select memory | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `08 A0` memory group | Y | Y | Y | Y | Y | — | — | — | — | — |

---

### 09 — Memory Write

Writes the current VFO contents into the selected memory channel. No data bytes.
**Support:** universal.

### 0A — Memory Copy → VFO

Copies the selected memory channel contents to the VFO. No data bytes.
**Support:** universal.

### 0B — Memory Clear

Clears the selected memory channel. No data bytes.
**Support:** universal.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `09` Memory write | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `0A` Memory→VFO | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `0B` Memory clear | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 0C — Read Frequency Offset  ·  0D — Send Frequency Offset

Read/write the duplex (repeater) offset frequency. 3-byte BCD payload.

**Data format** (IC-705 "Duplex Offset frequency setting", commands `0C`, `0D`):

| Byte # | Value | Description |
|:--:|:--:|-------------|
| 5 | 10 Hz · 1 Hz *(1 kHz·100 Hz per layout)* | BCD, LSB pair first |
| 6 | 100 kHz · 10 kHz | BCD |
| 7 | 10 MHz · 1 MHz | BCD (10 MHz digit fixed) |

The offset is a 3-byte BCD value in the same little-endian-by-byte order as the main
frequency but only three bytes wide (offsets are < 100 MHz). `0C` is read, `0D` is send.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `0C` Read offset | Y | Y | Y | — | Y | (4) | Y | (4) | Y | (4) |
| `0D` Send offset | Y | Y | Y | — | Y | (4) | Y | (4) | Y | (4) |

*Note 4:* HF-only radios (7610/7300/7851) expose the offset command for their limited
duplex/split-offset use; the VHF/UHF radios use it for repeater shift. R8600 (RX) has no
transmit offset.

---

### 0E — Scan (sub-command)

Starts/stops scanning and configures scan parameters. Common sub-commands:

| Sub | Description |
|:--:|-------------|
| `00` | Cancel scan |
| `01` | Start Programmed / Memory scan |
| `02` | Start Programmed scan |
| `03` | Start ∂F scan |
| `12` | Start Fine Programmed scan |
| `13` | Start Fine ∂F scan |
| `22` | Start Memory scan |
| `23` | Start Select-Memory scan |
| `24` | Start Mode-Select scan |
| `Ax` (x=1–7) | Select ∂F scan span (±5 kHz … ±1 MHz) |
| `B0` | Clear Select-channel setting |
| `B1` | Set as Select channel |
| `B2 00`–`03` | Set Select-memory scan channel (ALL/SEL1/2/3) |
| `D0` | Scan resume OFF |
| `D3` | Scan resume ON |

The exact sub-command set differs by radio; the R8600 (a scanning receiver) has the
richest scan set including Auto-Memory-Write scan and duplicated-frequency scan.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `0E` Scan | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 0F — Split / Duplex (sub-command)

| Sub | Direction | Description |
|:--:|:--:|-------------|
| `00` | read | Read Split OFF status |
| `01` | read | Read Split ON status |
| `11` | read | Read DUP− operation |
| `12` | read | Read DUP+ operation |
| `00` | set | Split function OFF |
| `01` | set | Split function ON |
| `10` | set | Simplex operation |
| `11` | set | DUP− operation |
| `12` | set | DUP+ operation |

(The read vs set meaning of `00`/`01` is disambiguated by the read-only vs send-only
annotation in each manual; on the IC-705 the read forms are `0F 00`/`0F 01` read-only and
the set forms `0F 00`/`0F 01` send-only.)

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `0F` Split/Duplex | Y | Y | Y | — | Y | Y | Y | Y | Y | Y |

---

### 10 — Send/Read Tuning Step

1-byte code selecting the VFO tuning step.

| Data | Step | Data | Step |
|:--:|:--:|:--:|:--:|
| `00` | OFF (10 Hz/1 Hz) | `07` | 9 kHz |
| `01` | 100 Hz | `08` | 10 kHz |
| `02` | 500 Hz | `09` | 12.5 kHz |
| `03` | 1 kHz | `10` | 20 kHz |
| `04` | 5 kHz | `11` | 25 kHz |
| `05` | 6.25 kHz | `12` | 50 kHz |
| `06` | 8.33 kHz | `13` | 100 kHz |

The exact code list varies: HF radios stop at fewer steps; the R8600 and VHF/UHF radios
include the AM-broadcast/airband steps (9 kHz, 8.33 kHz). Values shown are the IC-705 set.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `10` Tuning step | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 11 — Send/Read Attenuator

1-byte code. On radios with a single ATT: `00`=OFF, `20`=20 dB (IC-705, HF portion).
Radios with switchable multi-step attenuators encode the dB value directly:
IC-7610/IC-7851 accept `00`,`03`,`06`,`09`,`12`,`15`,`18`,`21`,`24`,`27`,`30`,`33`,`36`,`39`,`42`,`45`
(dB in hex-of-decimal steps); the R8600 has a wide ATT range as well.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `11` Attenuator | Y | Y | Y | Y | (5) | Y | Y | Y | Y | Y |

*Note 5:* ID-50 handheld has limited/absent ATT depending on firmware.

---

### 12 — Send/Read Antenna Selection

1-byte antenna selector on radios with switchable antenna jacks (IC-7610: ANT1/ANT2,
IC-7851: 4 antennas + RX-antenna routing). Not present on single-antenna radios (705,
9700 per-band, ID-50). Data `00`=ANT1, `01`=ANT2, … plus RX-ANT sub-bytes on 7851/7610.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `12` Antenna select | — | — | — | (6) | — | Y | — | — | Y | Y |

*Note 6:* R8600 selects between its antenna connectors for different bands.

---

### 13 — Speech (voice synthesizer)

Announces status via the radio's voice synthesizer. Sub-commands:

| Sub | Description |
|:--:|-------------|
| `00` | Speak all data (S-meter level, frequency, mode) |
| `01` | Speak frequency + S-meter level |
| `02` | Speak the operating mode |

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `13` Speech | Y | Y | Y | Y | (7) | Y | Y | Y | Y | Y |

*Note 7:* Voice-synthesizer availability depends on the optional/built-in speech unit.

---

### 14 — Set Level (sub-command)

Read/write analog control levels. **Payload = 2-byte BCD `0000`–`0255`** (0–255 scale)
unless noted. Sub-command is 1 byte.

| Byte # | Value | Description |
|:--:|:--:|-------------|
| 0–3 | `FE FE <to> <from>` | Envelope |
| 4 | `14` | Command |
| 5 | `Sc` | Sub-command (level selector) |
| 6–7 | `0000`–`0255` | 2-byte BCD level |
| 8 | `FD` | Terminator |

| Sub | Level | Range notes |
|:--:|-------|-------------|
| `01` | AF (volume) | 0=min … 255=max |
| `02` | RF gain | 0=min … 255=max |
| `03` | Squelch | 0=min … 255=max |
| `06` | NR level | 0=0% … 255=100% |
| `07` | PBT1 (TWIN PBT) | 0=max CCW, 128=center, 255=max CW |
| `08` | PBT2 (TWIN PBT) | 0=max CCW, 128=center, 255=max CW |
| `09` | CW pitch | 0=300 Hz, 128=600 Hz, 255=900 Hz (5 Hz steps) |
| `0A` | RF power | 0=min … 255=max |
| `0B` | MIC gain | 0=min … 255=max |
| `0C` | Keying speed | 0=6 WPM … 255=48 WPM |
| `0D` | Notch position | 0=max CCW, 128=center, 255=max CW |
| `0E` | COMP level | 0=0 … 255=10 |
| `0F` | Break-in delay | 0=2.0d … 255=13.0d |
| `12` | NB level | 0=0% … 255=100% |
| `15` | Monitor (MONI) level | 0=0% … 255=100% |
| `16` | VOX gain | 0=0% … 255=100% |
| `17` | Anti-VOX gain | 0=0% … 255=100% |
| `19` | LCD backlight brightness | 0=0% … 255=100% |

Sub-command allocation is broadly shared but not identical: the R8600 (RX) omits RF-power,
MIC-gain, keying-speed, COMP, break-in, VOX, anti-VOX; the ID-50 omits PBT/notch/COMP.
Some radios add or move levels (e.g. IC-7300/7610 backlight is `19`; IC-7851 adds
sub-levels not on portables). Always confirm the sub-code against the specific radio's
table before writing.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `14` Set level | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `14 01` AF | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `14 02` RF gain | Y | Y | Y | Y | (8) | Y | Y | Y | Y | Y |
| `14 03` Squelch | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `14 06` NR | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `14 07/08` PBT1/2 | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `14 09` CW pitch | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `14 0A` RF power | Y | Y | Y | — | Y | Y | Y | Y | Y | Y |
| `14 0B` MIC gain | Y | Y | Y | — | Y | Y | Y | Y | Y | Y |
| `14 0C` Keying speed | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `14 0D` Notch | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `14 0E` COMP | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `14 0F` BK-IN delay | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `14 12` NB level | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `14 15` MONI level | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `14 16/17` VOX/Anti-VOX | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `14 19` Backlight | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

*Note 8:* ID-50 uses a simplified RF/SQL control.

---

### 15 — Read Meter (sub-command)

Read-only meter readouts. Payload = 2-byte BCD `0000`–`0255`. Sub-commands:

| Sub | Meter | Scale |
|:--:|-------|-------|
| `01` | Noise/S-meter squelch status | `00`=closed, `01`=open |
| `02` | S-meter level | 0=S0, 120=S9, 241=S9+60 dB |
| `05` | Various squelch (tone/etc.) status | `00`=closed, `01`=open |
| `07` | OVF (overflow) status | `00`=off, `01`=on |
| `11` | Po (power) meter | 0=0%, 143=50%, 213=100% |
| `12` | SWR meter | 0=SWR1.0, 48=1.5, 80=2.0, 120=3.0 |
| `13` | ALC meter | 0=min, 120=max |
| `14` | COMP meter | 0=0 dB, 130=15 dB, 210=25.5 dB |
| `15` | Vd meter (drain V) | 0=0 V, 75=5 V, 241=16 V |
| `16` | Id meter (drain I) | 0=0 A, 121=2 A, 241=4 A |

> **Quirk (IC-7610):** The real-time RF-power-out meter (`RFPOWER_METER_WATTS`) was not
> implemented in some Hamlib builds, so software reported the power *setting* rather than
> measured output.

The R8600 (RX) exposes only the receive-relevant meters (S-meter, squelch, OVF); TX
meters (Po/SWR/ALC/COMP/Vd/Id) are transceiver-only.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `15 01` SQL status | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `15 02` S-meter | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `15 05` Various SQL | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `15 07` OVF | Y | Y | Y | Y | — | Y | Y | Y | — | Y |
| `15 11` Po meter | Y | Y | Y | — | Y | Y | Y | Y | Y | Y |
| `15 12` SWR meter | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `15 13` ALC meter | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `15 14` COMP meter | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `15 15` Vd meter | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `15 16` Id meter | Y | Y | Y | — | — | Y | Y | Y | Y | Y |

---

### 16 — Set Function ON/OFF (sub-command)

Toggles DSP/audio functions. Payload = 1 byte (`00`=OFF, `01`=ON) unless a wider
enumeration is noted. Sub-commands (IC-705 set; shared broadly):

| Sub | Function | Values |
|:--:|----------|--------|
| `02` | Preamp | `00`=OFF, `01`=P.AMP1, `02`=P.AMP2 (VHF/UHF: 00=OFF/01=ON) |
| `12` | AGC time constant | `01`=FAST, `02`=MID, `03`=SLOW |
| `22` | Noise Blanker | 00/01 |
| `40` | Noise Reduction | 00/01 |
| `41` | Auto Notch | 00/01 |
| `42` | Repeater tone | 00/01 |
| `43` | Tone squelch | 00/01 |
| `44` | Speech compressor | 00/01 |
| `45` | Monitor (MONI) | 00/01 |
| `46` | VOX | 00/01 |
| `47` | BK-IN | `00`=OFF, `01`=Semi, `02`=Full |
| `48` | Manual Notch | 00/01 |
| `4B` | DTCS | 00/01 |
| `4F` | Twin Peak Filter | 00/01 |
| `50` | Dial lock | 00/01 |
| `56` | DSP IF filter type | `00`=SHARP, `01`=SOFT |
| `57` | Manual Notch width | `00`=WIDE, `01`=MID, `02`=NAR |
| `58` | SSB TX bandwidth | `00`=WIDE, `01`=MID, `02`=NAR |
| `5B` | DSQL/CSQL (DV) | `00`=OFF, `01`=DSQL, `02`=CSQL |
| `5C` | GPS TX mode | `00`=OFF, `01`=D-PRS, `02`=NMEA |
| `5D` | Tone squelch type | `00`=OFF,`01`=TONE,`02`=TSQL,`03`=DTCS,`06`–`09`=combos |

> **Quirk (IC-9700 / IC-7300):** There is no dedicated "AGC off." To fully disable AGC you
> set the AGC time-constant register (`1A 04`) to 0, not `16 12`.

> **Quirk (IC-7300):** Setting a custom filter/passband width can be silently ignored —
> the set appears to succeed but the width reverts on the next query.

The AGC constant here (`16 12`) is 3-step; radios with finer AGC use `1A 04` (see below).
R8600 has NB/NR/preamp/AGC but no TX functions (COMP, VOX, BK-IN). D-STAR-only functions
(`5B`/`5C`/`5D` tone/DV) are absent on non-DV radios (7610/7300/7851/7000/R8600).

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `16 02` Preamp | Y | Y | Y | Y | (9) | Y | Y | Y | Y | Y |
| `16 12` AGC | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `16 22` NB | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `16 40` NR | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `16 41` Auto Notch | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `16 42/43` Rpt tone/TSQL | Y | Y | Y | (10) | Y | Y | Y | Y | Y | Y |
| `16 44` COMP | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `16 45` MONI | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `16 46` VOX | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `16 47` BK-IN | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `16 48` Man Notch | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `16 4B` DTCS | Y | Y | Y | (10) | Y | — | Y | — | Y | — |
| `16 5B–5D` DV/tone type | Y | Y | Y | (10) | Y | — | Y | — | (11) | — |

*Note 9:* ID-50 preamp is 00/01. *Note 10:* R8600 supports the receive-side tone/DTCS
squelch functions. *Note 11:* IC-7000 has analog tone squelch (`5D`) but no DV.

---

### 17 — Send CW Message

Send-only. Transmits up to 30 ASCII characters as CW when keyed. Character codes:
`0`–`9` = 0x30–0x39, `A`–`Z` = 0x41–0x5A, plus `/ ? . − , : ' ( ) = + " @` and space.
`FF` stops sending; `^` (0x5E) joins characters with no inter-character space.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `17` Send CW | Y | Y | Y | — | — | Y | Y | Y | Y | Y |

---

### 18 — Power ON/OFF

| Sub | Description |
|:--:|-------------|
| `00` | Turn OFF the transceiver |
| `01` | Turn ON the transceiver (send `FE` wake-up preamble bytes first) |

To wake a powered-off radio, the controller must precede `18 01` with a run of `FE`
padding bytes (the number depends on baud rate) to wake the CI-V UART.

> **Quirk (IC-7300):** CI-V power-on to a fully powered-off radio is unreliable across
> several firmware/driver versions — the wake command frequently times out.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `18 00` Power OFF | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `18 01` Power ON | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 19 — Read Transceiver ID

`19 00` (read-only) returns the radio's own CI-V address in the data byte — used to
auto-detect the address on the bus.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `19 00` Read ID | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |

---

### 1A — Various (multi sub-command)

Command `1A` is the catch-all for structured data and the entire Set-mode menu. Its
sub-command byte selects a sub-function; `1A 05` further takes a **2-byte BCD register
number** whose meaning is **radio-specific**.

| Sub | Description |
|:--:|-------------|
| `00` | Send/read memory contents |
| `01` | Send/read band-stacking register contents |
| `02` | Send/read memory-keyer contents |
| `03` | Send/read the selected IF filter width |
| `04` | Send/read the selected AGC time constant |
| `05` | Send/read a Set-mode menu register (2-byte register number) |
| `06` | Send/read the DATA mode setting |
| `07`+ | (IC-705/905/9700) NTP access, OVF, power-supply-type, share-pictures, etc. |

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `1A 00` Memory contents | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `1A 01` Band stacking | Y | Y | Y | — | Y | Y | Y | Y | Y | Y |
| `1A 02` Memory keyer | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `1A 03` IF filter width | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `1A 04` AGC time const | Y | Y | Y | Y | — | Y | Y | Y | Y | Y |
| `1A 05` Menu register | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
| `1A 06` DATA mode | Y | Y | Y | — | — | Y | Y | Y | Y | Y |

---

#### 1A 00 — Send/Read Memory Contents

A large fixed-layout record describing one memory channel. On the IC-705 the record is
up to 68 bytes and is organized (byte groups shown 1-indexed within the data area):

| Field bytes | Contents |
|:--:|----------|
| 1–2 | Memory group number (`0000`–`0099` channel group; `0100` call-channel group) |
| 3–4 | Memory channel number (`0000`–`0099`; call channels 0000/0001=144C1/C2, 0002/0003=430C1/C2) |
| 5 | Split & Select-memory flags (low nibble = select 0–3, high bit = split on/off) |
| 6–10 | Operating frequency (5-byte BCD, as command `00`) |
| 11–12 | Operating mode + filter (as command `01`) |
| 13 | Data-mode flag (`00`=off, `01`=on) |
| 14 | Duplex + tone flags (low nibble tone: 0=OFF/1=TONE/2=TSQL/3=DTCS; high nibble duplex: 0=OFF/1=−/2=+) |
| 15 | DV digital-squelch setting (0=off, 1=DSQL, 2=CSQL) |
| 16–18 | Repeater tone frequency |
| 19–21 | TSQL tone frequency |
| 22–24 | DTCS code |
| 25 | DV digital code-squelch |
| 26–28 | Duplex offset frequency (3-byte BCD) |
| 29–36 | UR (destination) call sign (8 ASCII) *(DV radios)* |
| 37–44 | R1 (access repeater) call sign (8 ASCII) *(DV radios)* |
| 45–52 | R2 (gateway/link) call sign (8 ASCII) *(DV radios)* |
| 53–68 | Memory name (16 ASCII) |

To **clear** a channel with `1A 00`: send bytes 2–3 = channel (0001–0099), byte 4 = `FF`,
no further data. Non-DV radios omit the DV call-sign fields (their record is shorter);
the R8600's record is a receiver channel (no TX/DV call-sign fields but with attenuator,
ATT, and skip/select flags). This is exactly the record subject to the R8600 flow-control
quirk below.

> **Quirk (IC-R8600):** Reading memory channels 11 or 13 via `1A 00` returns malformed
> data when the PC serial port has **software (XON/XOFF) flow control** enabled — 0x11 and
> 0x13 are the XON/XOFF bytes and the OS strips them from the CI-V stream. Disable software
> flow control to fix.

---

#### 1A 01 — Send/Read Band-Stacking Register

Reads/writes the band-stacking memory. Data = **band code** (1 byte) + **register code**
(1 byte), followed by the frequency/mode payload (as in memory content bytes 6–52) when
sending.

Band codes (IC-705): `01`=1.9, `02`=3.5, `03`=7, `04`=10, `05`=14, `06`=18, `07`=21,
`08`=24, `09`=28, `10`=50, `11`=WFM, `12`=Air, `13`=144, `14`=430, `15`=GENE.
Register codes: `01`=most-recent (left), `02`=2nd (center), `03`=3rd (right).
Example: read the 21 MHz center register = data `07 02`. Band/register code sets differ
by radio band coverage (9700 uses 144/430/1200; 7300/7610/7851 use HF+50 codes only).

---

#### 1A 02 — Send/Read Memory-Keyer Contents

CW keyer memory text. Data = 1-byte channel (`01`=M1 … `08`=M8) + up to 70 ASCII
characters. Codes: `0`–`9`, `A`–`Z`, space, `/ ? , . @`; `^` joins two chars into a
prosign (e.g. `^4254` = BT); `*` inserts the contest serial number (one channel only).
Sending one or more spaces clears the channel.

Present only on radios with a CW keyer memory (not R8600/ID-50).

---

#### 1A 03 — Send/Read Selected IF Filter Width

1-byte data selecting the DSP filter width for the current mode:

| Mode | Data | Width steps |
|------|:--:|-------------|
| SSB/CW/RTTY | `00`–`09` | 50–500 Hz (50 Hz steps) |
| SSB/CW | `10`–`40` | 600 Hz – 3.6 kHz (100 Hz steps) |
| RTTY | `10`–`31` | 600 Hz – 2.7 kHz (100 Hz steps) |
| AM | `00`–`49` | 200 Hz – 10.0 kHz (200 Hz steps) |

> **Quirk (IC-7300):** width sets can be silently ignored (see command `16`).

---

#### 1A 04 — Send/Read Selected AGC Time Constant

1-byte data giving the AGC time constant for the current mode. The **value → seconds**
mapping is mode-dependent:

| Data | SSB/CW/RTTY | AM |
|:--:|:--:|:--:|
| `00` | OFF | OFF |
| `01` | 0.1 | 0.3 |
| `02` | 0.2 | 0.5 |
| `03` | 0.3 | 0.8 |
| `04` | 0.5 | 1.2 |
| `05` | 0.8 | 1.6 |
| `06` | 1.2 | 2.0 |
| `07` | 1.6 | 2.5 |
| `08` | 2.0 | 3.0 |
| `09` | 2.5 | 4.0 |
| `10` | 3.0 | 5.0 |
| `11` | 4.0 | 6.0 |
| `12` | 5.0 | 7.0 |
| `13` | 6.0 | 8.0 |

> **Quirk (IC-9700 / IC-7300):** setting `1A 04` = `00` is the *only* way to fully turn
> AGC off over CI-V (there is no `16`-level AGC-off toggle).

---

#### 1A 05 — Send/Read Set-Mode Menu Register (2-byte register)

This is the largest and most radio-divergent part of the protocol. The sub-command is
`05`, followed by a **4-digit (2-byte) BCD register number**, followed by that register's
data (1 byte for on/off/enum, 2-byte BCD for a 0–255 level, ASCII for text, or a
frequency/color field). **The register numbers are NOT portable between radios** — they
are assigned in the order each radio's Set menu happens to be laid out.

**Byte layout:**

| Byte # | Value | Description |
|:--:|:--:|-------------|
| 0–3 | `FE FE <to> <from>` | Envelope |
| 4 | `1A` | Command |
| 5 | `05` | Sub-command |
| 6–7 | `nnnn` | Register number (2-byte BCD) |
| 8..n | data | Register value (type depends on register) |
| n+1 | `FD` | Terminator |

**Concrete illustration of the divergence** — the same register number means different
things on different radios:

| Register | IC-705 | IC-7300 | IC-9700 |
|:--:|--------|---------|---------|
| `0027` | TX > DV > Tone (Bass) level | (Twin peak filter / RTTY area) | Beep Level |
| `0030` | Beep Level Limit | Quick Split set | (DV/beep area) |
| `0033` | Band Edge Beep | Split Lock set | — |
| `0055` | SPEECH speed | Screen-capture image format | — |
| `0071` | Charging (Power ON) | CI-V Transceive | (Connectors area) |
| `0072` | USB Power Input | CI-V USB Echo Back | REF Adjust |
| `0089` | REF Adjust | Screen saver | — |

Because of this, software must key `1A 05` registers off the specific model, never reuse
another model's map. The register **count** differs enormously: the IC-705 defines
~360+ registers (`0001`–`0362`, spanning Tone/TBW, Function, Connectors, Display, Time,
SD, Scope, Audio-scope, Keyer, RTTY-decode, Record, Scan, GPS/D-PRS, DTMF, NB, VOX, CD);
the IC-7300 ~130; the IC-9700 covers dual-band-specific items (Beep Sound MAIN/SUB,
REF Adjust + REF Adjust FINE, DV/DD Set).

**Representative register types and their data encodings** (from IC-705 command formats,
applicable structure across radios):

- **On/off & enums:** 1 byte (`00`=OFF, `01`=ON, or `00`–`0n` enumeration).
- **0–255 levels** (e.g. Beep Level, SPEECH Level, backlight, MOD/AF/IF output levels):
  2-byte BCD `0000`–`0255`.
- **UTC Offset** (705 reg `0170`): 4-digit BCD offset `0000`–`1400` + 1-byte direction
  (`00`=+, `01`=−).
- **Split offset** (705 reg `0046`): 3-byte BCD (1 kHz…1 MHz digits) + direction byte
  (`00`=+, `01`=−).
- **Color** (705 regs `0180`–`0182`, `0240`, `0242`, `0258`, `0262`, `0263`): three
  2-byte BCD fields R/G/B, each `0000`–`0255`.
- **Band-scope FIX edge frequencies** (705 regs `0188`–`0238`): a lower + higher 5-byte
  BCD frequency pair per edge (same digit order as command `00`).
- **Text** (e.g. NTP server address reg `0168`): ASCII, restricted to `A`–`Z`,`a`–`z`,
  `0`–`9`,`.`,`-` for network fields; full ASCII for names.
- **Date/Time** (705 regs `0165`/`0166`): date as BCD `20000101`–`20991231`, time as BCD
  `0000`–`2359`.

> **Quirk (IC-7300 Mk2):** the USB **AF output level** register moved from `0x60` to
> `0x70` on the Mk2, so IC-7300-Mk1 register maps target the wrong parameter on a Mk2.

> **Note (register maps not fully transcribed):** The full per-register enumeration for all
> ten radios (well over 1,500 distinct registers in total) is beyond a single unified
> table; this document gives the byte-layout, data-typing rules, and divergence examples,
> and defers the exhaustive per-model register list to each radio's own `1A 05` command
> table. See *Coverage & Methodology Notes*.

---

#### 1A 06 — Send/Read DATA Mode

Selects the data (packet/digital) sub-mode and, on most radios, the filter. Data = 1 byte
DATA on/off (`00`/`01`) + 1 byte filter (`00`=none/`01`=FIL1…). On the IC-705 the format
is documented under "Data mode with filter width settings."

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `1A 06` DATA mode | Y | Y | Y | — | — | Y | Y | Y | Y | Y |

---

#### 1A 07+ — Extended sub-commands (705 / 905 / 9700 family)

The newest portables add higher `1A` sub-commands: `1A 07` NTP server access
(`00`=terminate, `01`=initiate), `1A 08` read NTP result, `1A 09` read OVF indicator,
`1A 0A` Share-Pictures status, `1A 0B` read power-supply type (external vs battery pack).
These are absent from older radios (7000/7100/7300/7610/7851).

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `1A 07`–`0B` extended | Y | Y | (12) | — | (12) | — | — | — | — | — |

*Note 12:* Subset present; e.g. ID-50 has power-supply/battery reads, 9700 lacks the
battery read (mains-powered).

---

### 1B — Tone / Squelch Codes (sub-command)

Read/write repeater and squelch tone/code data.

| Sub | Description | Data |
|:--:|-------------|------|
| `00` | Repeater tone frequency | 3-byte BCD tone (e.g. `00 88 5` = 88.5 Hz), format below |
| `01` | TSQL tone frequency | 3-byte BCD tone |
| `02` | DTCS code + polarity | polarity byte + 3-digit BCD DTCS code |
| `07` | CSQL code (DV mode) | digital code-squelch value (DV radios) |

**Tone frequency format** (`1B 00`/`01`): two data bytes of BCD giving the tone in tenths
of Hz, e.g. `07 44 00`-style layout where digits encode `074.4`. **DTCS format**
(`1B 02`): byte 1 = polarity (bit pattern for TX/RX normal/reverse), bytes 2–3 = BCD
3-digit octal DTCS code.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `1B 00` Rpt tone | Y | Y | Y | (13) | Y | (14) | Y | (14) | Y | (14) |
| `1B 01` TSQL tone | Y | Y | Y | (13) | Y | (14) | Y | (14) | Y | (14) |
| `1B 02` DTCS | Y | Y | Y | (13) | Y | — | Y | — | Y | — |
| `1B 07` CSQL (DV) | Y | Y | Y | — | Y | — | Y | — | — | — |

*Note 13:* R8600 has the receive-side tone/DTCS squelch. *Note 14:* HF SDR radios expose
tone data mainly for FM on 50 MHz/repeater use.

---

### 1C — Transceiver Status / PTT / Tune (sub-command)

| Sub | Direction | Description | Data |
|:--:|:--:|-------------|------|
| `00` | read/set | Transceiver RX/TX status (software PTT) | `00`=RX, `01`=TX |
| `01` | read/set | Antenna-tuner status | `00`=OFF, `01`=ON, `02`=start Tune |
| `02` | read/set | Transmit-frequency monitor (XFC) | `00`=OFF, `01`=ON |
| `03` | read | Read the transmit frequency | 5-byte BCD (as command `00`) |

`1C 00 01` is the standard **software PTT key-down** (`1C 00 00` = release). `1C 01`
controls the internal antenna tuner (radios with one: 705/9700/7300/7610/7851/7000/905;
not R8600/ID-50). `1C 03` reads the actual TX (split) frequency.

> **Quirk (IC-705):** PTT over CI-V can fail independently of RX/CAT with a serial re-open
> error (`serial_open status=-6`) even when frequency/mode tracking works, because the PTT
> path re-negotiates the port separately.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `1C 00` RX/TX (PTT) | Y | Y | Y | — | Y | Y | Y | Y | Y | Y |
| `1C 01` Tuner | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `1C 02` XFC | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `1C 03` Read TX freq | Y | Y | Y | — | — | Y | Y | Y | Y | Y |

---

### 1E — TX Band Edges (sub-command)

| Sub | Direction | Description |
|:--:|:--:|-------------|
| `00` | read | Read number of available TX frequency bands |
| `01` | read | Read TX band-edge frequencies (lower/higher 5-byte BCD, `2D` separator, as command `02`) |
| `02` | read | Read number of user-set TX bands |
| `03` | set/read | Send/read user-set TX band-edge frequencies |

Present on transceivers with programmable TX limits; format identical to command `02`.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `1E 00`–`03` TX edges | Y | Y | Y | — | (15) | Y | Y | Y | (15) | Y |

*Note 15:* Reduced/legacy edge behavior on ID-50 / IC-7000.

---

### 1F — DV My/UR Call Signs (sub-command)  *(D-STAR radios)*

| Sub | Description | Data |
|:--:|-------------|------|
| `00` | My call sign (SET > My Station) | 8+4 ASCII |
| `01` | UR, R1, R2 (CS) call signs | 3 × 8 ASCII |
| `02` | TX message | up to 20 ASCII |

Call-sign fields are fixed-length ASCII, space-padded. Only on DV radios (705/905/9700/
7100/ID-50).

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `1F 00`–`02` DV call signs | Y | Y | Y | — | Y | — | Y | — | — | — |

---

### 20 — DV RX Call Sign / Message / Status Output (sub-command)  *(D-STAR radios)*

Reads received-DV metadata and controls whether the radio auto-outputs it on receive.
Nested `<sub> <sub2>` structure, e.g. `20 00 00` = auto-output of RX call signs on/off,
`20 00 01` = output RX call signs (transceive), `20 00 02` = read RX call signs;
`20 01 xx` = RX message; `20 02 xx` = RX status; `20 03`/`04` = auto GPS/D-PRS and
D-PRS message output, with `0100`–`0203` selectors for Position/Object/Item/Weather/
message. Output-enable flags auto-reset to OFF at power-off.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `20` DV RX output | Y | Y | Y | — | Y | — | Y | — | — | — |

---

### 21 — RIT / ∂TX (sub-command)

| Sub | Description | Data |
|:--:|-------------|------|
| `00` | RIT frequency | signed BCD offset (2-byte BCD magnitude + sign), format below |
| `01` | RIT on/off | `00`=OFF, `01`=ON |
| `02` | ∂TX (XIT) on/off | `00`=OFF, `01`=ON |

**RIT frequency format** (`21 00`): 2-byte BCD magnitude in Hz (0000–9999) plus a 1-byte
sign (`00`=+, `01`=−); range is typically ±9.999 kHz.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `21 00` RIT freq | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `21 01` RIT on/off | Y | Y | Y | — | — | Y | Y | Y | Y | Y |
| `21 02` ∂TX on/off | Y | Y | Y | — | — | Y | Y | Y | Y | Y |

---

### 22 — DV TX Data (sub-command)  *(D-STAR radios)*

`22 00` sets the DV low-speed/fast TX data (up to 30 bytes). `22 01 00/01` controls
auto-output of RX data; `22 02`–`05` mirror the DV-Set fast-data menu (DV Data TX
PTT/Auto, Fast Data on/off, GPS Data speed, TX Delay).

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `22` DV TX data | Y | Y | Y | — | Y | — | (16) | — | — | — |

*Note 16:* IC-7100 supports DV but with a smaller fast-data feature set than the 705/9700.

---

### 23 — GPS Position / GPS Set (sub-command)  *(GPS-equipped radios)*

`23 00` reads position status; `23 01` GPS Select (`00`=OFF, `01`=ON, `03`=Manual);
`23 02` manual position (latitude/longitude/altitude BCD fields). GPS position data uses
the D-PRS/NMEA field layouts documented in the "GPS/D-PRS data" command-format pages.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `23` GPS position | Y | Y | (17) | — | Y | — | (17) | — | — | — |

*Note 17:* 9700/7100 have GPS-related reads but no internal GPS receiver on all variants;
position may be manual-entry only.

---

### 24 — TX Output Power ON/OFF (sub-command)

`24 00 00/01` reads/sets whether TX output is enabled; `24 00 01` (with second byte
`00/01`) sets it for transceive. Distinct from the RF-power *level* (`14 0A`). Not on
R8600 (RX) or ID-50 in the same form.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `24 00` TX output on/off | Y | Y | Y | — | — | (18) | (18) | (18) | (18) | (18) |

*Note 18:* Older/HF radios may not implement the `24` transmit-enable toggle; use `1C 00`
PTT plus `14 0A` power instead. Confirm against the specific manual.

---

### 25 — Selected/Unselected VFO Frequency (sub-command)

Read/write the frequency of the selected or unselected VFO directly (Main/Sub or A/B)
without switching VFOs. Data = 1-byte VFO selector + 5-byte BCD frequency (as command
`00`). Introduced on the SDR/dual-receiver generation.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `25` (un)selected VFO freq | Y | Y | Y | Y | — | Y | — | Y | — | Y |

---

### 26 — Selected/Unselected VFO Mode + Filter (sub-command)

Read/write the operating mode, data-mode flag and filter of the selected/unselected VFO.
Data = VFO selector + mode + data-mode + filter bytes. Same generation as command `25`.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `26` (un)selected VFO mode | Y | Y | Y | Y | — | Y | — | Y | — | Y |

---

### 27 — Band-Scope Waveform / Control (sub-command)

Reads the band-scope waveform data and controls the scope. Key sub-commands (IC-705):

| Sub | Description | Data |
|:--:|-------------|------|
| `00` | Read scope waveform data | streamed only when `27 10`+`27 11` are ON |
| `10` | Scope ON/OFF | 00/01 |
| `11` | Scope waveform data output | 00/01 |
| `12` | Main/Sub scope | 00 (fixed on single-scope radios) |
| `13` | Single/Dual scope | 00 (fixed on single-scope radios) |
| `14` | Center/Fixed mode | `0000`=Center, `0001`=Fix |
| `15` | Span (Center mode) | frequency span BCD |
| `16` | Edge number (Fixed mode) | `0001`–`0003` |
| `17` | Scope hold ON/OFF | `0000`/`0001` |
| `19` | Reference level | signed BCD |
| `1A` | Sweep speed | `0000`=FAST, `0001`=MID, `0002`=SLOW |
| `1B` | Scope during TX | 00/01 |
| `1C` | Center-type display | `00`=Filter center, `01`=Carrier point, `02`=Carrier point (abs.) |
| `1D` | VBW (video bandwidth) | `0000`=NAR, `0001`=WIDE |
| `1E` | Fixed edge frequencies | lower/higher 5-byte BCD pair |
| `1F` | RBW (resolution bandwidth) | IC-7610/R8600 |
| `20` | Marker position (FIX/SCROLL) | `00`=Filter center, `01`=Carrier point (IC-7610) |

The waveform payload (`27 00`) is a block of amplitude bytes plus header (scope mode,
edge/center info, waveform-data division count). Dual-scope radios (7610/7851/905/9700)
use `27 12`/`27 13` to pick Main/Sub and Single/Dual; single-scope radios (705/7300) fix
them at `00`.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `27` Band-scope | Y | Y | Y | Y | — | Y | — | Y | — | — |

The IC-7100 and IC-7000 have no waterfall/band-scope waveform output over CI-V; the ID-50
handheld has no band scope. The **IC-7851** predates command `27`: its spectrum/waterfall
scope is configured entirely through `1A 05` registers (waveform color, edges, waterfall,
Main/Sub scope arrangement, etc.) and it has **no** CI-V waveform-data streaming command.

---

### 28 — Voice TX Memory (sub-command)  *(radios with a voice-TX recorder)*

`28 00` transmits a recorded voice-TX memory: data `00`=Stop, `01`=T1 … `08`=T8.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `28 00` Voice TX | Y | Y | Y | — | — | Y | — | Y | — | (19) |

*Note 19:* The IC-7851 has a voice-TX recorder but transmits it via the `1A 05 0146`
register ("voice memory transmission"), not a top-level command `28`.

---

### 29 — Band-Direct Command Prefix (IC-7610)

A wrapper command introduced on the IC-7610 that lets the controller explicitly target the
Main or Sub band for a following supported command, **regardless of which band is currently
active**. Data = 1 selector byte (`00`=MAIN, `01`=SUB) followed by the *supported command*
(command byte + its sub-command/data) to be executed against that band.

**Byte layout:**

| Byte # | Value | Description |
|:--:|:--:|-------------|
| 0–4 | `FE FE <to> <from> 29` | Envelope + command |
| 5 | `00`/`01` | Band selector (MAIN/SUB) |
| 6..n | `<Cn> [<Sc>] [data]` | The wrapped supported command |
| n+1 | `FD` | Terminator |

This avoids the "swap VFO, act, swap back" dance on the dual-receiver SDR. Only a defined
subset of commands is valid inside the `29` wrapper (the 7610 manual marks those with a
"Command 29 supported" flag). This command is **IC-7610-specific** among the radios
covered here; other dual-receiver radios instead use `07 D0`/`D1` (Main/Sub select) or the
`25`/`26` selected/unselected-VFO commands.

| Cmd/Sub | 705 | 905 | 9700 | R86 | ID50 | 7610 | 7100 | 7300 | 7000 | 7851 |
|--------|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| `29` Band-direct prefix | — | — | — | — | — | Y | — | — | — | — |

---

## 4. Cross-model quirk index

Quirks pulled inline from `quirks.md`, indexed to the command they affect:

| Radio | Command affected | Quirk summary |
|-------|------------------|---------------|
| IC-705 | `1C 00` (PTT) | PTT fails with `serial_open status=-6` while RX/CAT works. |
| IC-705 | Transceive | Transceive-on causes protocol/command-rejected errors over the Remote Utility; disable it. |
| IC-905 | `00`/`03` (freq) | Manual VFO retune can truncate leading digits in the read-back frequency. |
| IC-9700 | `07`, `0F` (VFO/split) | Doppler updates cause Main/Sub swap and spurious SPLIT. |
| IC-9700 / IC-7300 | `1A 04` (AGC) | No AGC-off toggle; set AGC time constant to `00`. |
| IC-R8600 | `1A 00` (memory) | Channels 11/13 misread when PC uses XON/XOFF flow control (0x11/0x13 collision). |
| IC-7610 | `15 11` (Po meter) | Real-time power-out meter unimplemented in some drivers; reads the setting instead. |
| IC-7100 | Transceive / echo | Over USB CI-V, turn Transceive off and USB Echo Back on; ~19200 baud practical ceiling. |
| IC-7300 | `1A 03`/`16 58` (width) | Filter/passband width sets silently ignored (revert on next read). |
| IC-7300 | `1A 05` USB AF | USB AF-level register moved `0x60`→`0x70` on the Mk2. |
| IC-7300 | `18 01` (power on) | CI-V power-on to a powered-off radio is unreliable. |
| IC-7851 | `07`/split | Some drivers don't declare VFO B, so split can't engage through that path. |
| Cross-model | addressing | Two same-model radios share a default address — reassign one for a shared bus. |
| Cross-model | Transceive | Unsolicited broadcasts collide with polled commands; disable Transceive, poll instead. |

---

## 5. Coverage & Methodology Notes

### Manuals fully analyzed (text-extracted and transcribed)

- **IC-705** (`IC-705_ENG_CI-V_1_20200721.pdf`) — analyzed in full; used as the flagship
  reference for byte layouts (frequency, mode, band edge, duplex offset, memory content,
  band-stacking, keyer, IF filter width, AGC, `1A 05` register formats, DV/GPS). It is the
  most feature-complete modern radio and defines the superset command structure.
- **IC-905, IC-9700, IC-R8600, ID-50A/E, IC-7610** — dedicated CI-V reference guides,
  text-extracted cleanly and cross-referenced for the support matrix, default addresses,
  and per-radio command/sub-command presence.
- **IC-7100, IC-7300, IC-7000, IC-7851** — full "Advanced Instructions / Control Command"
  manuals; the CI-V command-table and data-content chapters were located by line offset and
  transcribed. Default addresses and command ceilings verified directly
  (IC-7100 `88`, IC-7300 `94`, IC-7000 `70`, IC-7851 `8E`).

### Verification performed

- Default CI-V addresses were read directly from each manual's data-format diagram, not
  from memory: IC-705 `A4`, IC-905 `AC`, IC-9700 `A2`, IC-R8600 `96`, ID-50 `AE`,
  IC-7610 `98`, IC-7100 `88`, IC-7300 `94`, IC-7000 `70`, IC-7851 `8E`.
- Command ceilings were verified per radio to avoid over-claiming: the **IC-7851** tops out
  at command `26` (its band scope and voice-TX memory are driven through `1A 05` registers,
  not commands `27`/`28`); the **IC-7610** uniquely adds command `29` (band-direct prefix);
  the **IC-7000/IC-7100** have no band-scope waveform command (`27`); the **IC-R8600** is
  receive-only and implements no TX command; the **ID-50** omits HF-transceiver features.
- Command `28` (Voice TX) presence was confirmed by direct grep for
  705/905/9700/7300/7610 and its absence confirmed for 7851/7100/7000/R86/ID50.

### Legacy radios (from the v3.2 general reference)

- `icom_civ_reference_v32_2002.pdf` is an **image-only (scanned) PDF** — `pdftotext`
  extracted **zero** text. It was therefore read **visually, page by page**, which limited
  how much could be transcribed economically.
- From it, the **verified legacy default-address table (Table 2-2)** was transcribed
  (IC-735, IC-R7000, IC-275/375/475/575/1275, IC-R71, IC-751A, IC-761, IC-271/471/1271,
  IC-781, IC-725, IC-R9000, IC-765, IC-970, IC-726, IC-R72, IC-R7100, IC-728, IC-729,
  IC-737) — see §2. The framing details it confirms (shared preamble/terminator, BCD
  frequency, CSMA/CD bus, up to 4 radios via CT-17, reserved addresses, and the IC-735's
  4-byte frequency length) are folded into §1–§2.
- The v3.2 command structure (its Section 7) was confirmed to match the modern universal
  core: transceive `00`/`01`, edge readout `02`, freq/mode readout `03`/`04`, freq/mode
  write `05`/`06`, VFO select `07`, memory `08`–`0B`, offset `0C`/`0D`, scan `0E`,
  split/duplex `0F`, tuning step `10`, plus "other commands." Full per-command transcription
  of the scanned legacy tables was **not** performed (cost/time vs. the primary scope); the
  addendum radios' (706 family, 746PRO, 756 family, 7800, R8500, etc.) individual command
  appendices were not transcribed and their addresses are community-cited, not re-verified.

### Scope limits and known gaps (flagged uncertainties, not fabricated)

- **`1A 05` register maps were NOT exhaustively transcribed.** Across the ten radios there
  are well over 1,500 distinct Set-menu registers, and — critically — the same register
  number means different things on different models (documented with concrete examples in
  the `1A 05` section). This document gives the **byte layout, the data-typing rules**
  (on/off, 0–255 BCD level, color RGB, frequency, ASCII, date/time), and **divergence
  examples**, and defers the complete per-model register enumeration to each radio's own
  `1A 05` table. This was a deliberate scoping decision, called out here.
- The support-matrix cells for a handful of older-radio commands (`24` TX-output-enable on
  HF radios; `0C`/`0D` offset usage on the SDR radios) are annotated with "see note" and
  hedged where the exact per-radio behavior could not be pinned to an unambiguous table row
  in the multi-column PDF extraction. These are marked rather than asserted.
- Some data-field bit-level details (e.g. the exact `1B 02` DTCS polarity byte encoding and
  the `21 00` RIT sign/magnitude packing) are described structurally from the flagship
  format pages; where a specific radio packs them differently, its own format page governs.
- The `Icom_IC_Cable_3.pdf` is a cabling/interface reference (not a command reference) and
  was not needed for command content.

### Totals

- **Top-level command bytes documented:** 42 — `00`–`19` (the classic universal core:
  `00`,`01`,`02`,`03`,`04`,`05`,`06`,`07`,`08`,`09`,`0A`,`0B`,`0C`,`0D`,`0E`,`0F`,`10`,
  `11`,`12`,`13`,`14`,`15`,`16`,`17`,`18`,`19`), plus `1A`,`1B`,`1C`,`1E`,`1F`,`20`,`21`,
  `22`,`23`,`24`,`25`,`26`,`27`,`28`,`29`.
- **Sub-commands documented:** ~180+ across `07`, `0E`, `0F`, `13`, `14`, `15`, `16`, `18`,
  `19`, `1A` (00–0B, including the `1A 05` register framework), `1B`, `1C`, `1E`, `1F`,
  `20`, `21`, `22`, `23`, `24`, `27`, `28`.
- **Radios covered:** 10 primary radios with full per-command support matrices, plus 20+
  legacy radios by verified default address (and the framing/command-core they share).
- **Manuals with extraction trouble:** only `icom_civ_reference_v32_2002.pdf` (scanned,
  image-only — no text layer; read visually with the limits noted above). All ten primary
  manuals extracted cleanly with `pdftotext -layout`.
