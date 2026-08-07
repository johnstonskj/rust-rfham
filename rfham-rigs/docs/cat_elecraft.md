# Elecraft & Lab599 Computer-Control ("CAT") Command Reference

> **Scope of this file.** This document covers the **Elecraft** K-line (K3, K3S, KX3, KX2)
> and **K4**, plus the **Lab599 TX-500**. Yaesu, Kenwood, and Elecraft/Lab599 each publish a
> separate, non-interoperable command set sharing only the "CAT" umbrella name. Yaesu is in
> `cat_yaesu.md`; Kenwood in `cat_kenwood.md`.
>
> **Lineage matters here.** Elecraft's command set is **derived from and extends the Kenwood
> "PC control" style** (2-letter, semicolon-terminated ASCII; `FA` = 11-digit Hz frequency;
> a Kenwood-shaped `IF` status block) — then adds a large set of Elecraft-only commands and
> operators. The **Lab599 TX-500 is Kenwood-compatible/derived** even more directly: its
> manual reproduces the Kenwood table format verbatim, uses the Kenwood `?;`/`E;`/`O;` error
> replies, and its mnemonics overlap the Kenwood set (`AC`, `CN`, `CT`, `TO`, `MR`, `MW`,
> `NT`, `VV`, …) far more than the Elecraft set. It is documented here per task scope, in its
> own Part (§Part C), with the Kenwood relationship flagged.
>
> So this file has three related-but-distinct dialects:
>
> - **Part A — Elecraft K3 / K3S / KX3 / KX2** (one shared Programmer's Reference).
> - **Part B — Elecraft K4** (its own reference; a superset-style evolution with new syntax).
> - **Part C — Lab599 TX-500** (Kenwood-derived).

---

## 1. Protocol fundamentals (Elecraft)

**Command format.** 2-, 3-, or 4-letter prefix + parameters + terminator `;`. Two
directions, in Elecraft's terminology:

- **GET** — computer → radio, prefix only (e.g. `FA;`). The radio's reply (**RSP**) uses the
  SET format.
- **SET** — computer → radio, prefix + parameters (e.g. `FA00014060000;`).

Commands are case-insensitive **except** the `KY <text>` CW/DATA text-send command in PSK
mode.

**Elecraft-specific syntax extensions:**

- **`$` suffix = VFO B / sub-receiver.** Append `$` to the prefix of an applicable command to
  target VFO B / the sub RX (e.g. `MD$3;` sets CW on VFO B; `AG$;` gets sub-RX AF gain).
  Applicable commands are flagged in the references.
- **Switch emulation.** `SWT nn;` / `SWH nn;` emulate a *tap* / *hold* of a front-panel
  switch (K3 family). The **K4** adds `SW nn;` (2-digit, tap/hold-agnostic) and still accepts
  a compatible subset of `SWT`/`SWH`.
- **K4 TOGGLE / INCR / DECR operators.** On the K4, `XX/;` toggles, `XX+;` increments, `XX-;`
  decrements a setting (e.g. `RA/;` toggles the attenuator; `BN$+;` next band on sub RX).
- **`#` display commands** (K4) are prefixed with `#` to disambiguate from radio commands.
- **Meta-commands `K2`, `K3`, `K4`** switch on legacy/extended interpretations of the
  2-letter commands (e.g. `K31` enables K3 extensions such as the DATA sub-mode field in `IF`).

**Auto-Info (`AI`).** Elecraft defines several AI levels (richer than Yaesu, similar spirit
to Kenwood):

- **K3 family:** `AI0` off, `AI1` VFO/RIT/etc. changes generate periodic `IF;`, `AI2` all
  changes generate periodic responses, `AI3` reserved.
- **K4:** `AI0`–`AI5` — `0` off, `1` periodic `IF;` on VFO/RIT/etc., `2` periodic on all
  changes, `4` *immediate* notification of non-client changes, `5` immediate on all changes;
  plus `AIDxxx;` sets the AI delay (60–999 ms, default 500). AI mode is **per-client** on
  the K4 (multiple clients over USB/RS-232/Ethernet).

**Busy / error handling.** A command the radio can't process while busy (transmitting, in
BSET, VFO-reverse, etc.) returns `?;`. This is a *retriable busy* signal — the K-series
reference recommends checking `TQ;` (TX query) / `IC;` (icon status) rather than fast-polling,
and warns against continuous polling during TX. Switch-emulation commands (`SWT`) need a
short delay before a follow-up status read (`TQ;`) reflects the change. (See §quirks.)

---

## 2. Mode codes (Elecraft `MD` / `IF` mode field)

Elecraft's single-digit mode field (K3 family and K4 identical):

| Code  | Mode  | Code  | Mode      |
|-------|-------|-------|-----------|
| 1     | LSB   | 6     | DATA      |
| 2     | USB   | 7     | CW-REV    |
| 3     | CW    | 8     | (N/A)     |
| 4     | FM    | 9     | DATA-REV  |
| 5     | AM    | 0     | (N/A)     |

This is Kenwood-shaped but **`6` = DATA** (Kenwood `6` = FSK) and **`9` = DATA-REV**
(Kenwood `9` = FSK-R). The specific **data sub-mode** (DATA A / AFSK A / FSK D / PSK D) is
selected separately with the **`DT`** command; `MD6`/`MD9` recall the last data sub-mode for
the band. In K2 command modes (`K21`/`K23`), the RSP converts modes 6/7 back to 1/2 for
legacy software. FM does not apply to the KX2.

---

## Part A — Elecraft K3 / K3S / KX3 / KX2

The K3, K3S, KX3, and KX2 share one Programmer's Reference. The **KX3/KX2 accept all K3
commands**, though some have no functional effect (marked `*` below); a few commands are
KX3/KX2-only (marked `**`). This part's matrix column "K3fam" therefore represents the
shared set; per-radio functional differences are noted where the reference flags them.

---

## 3. Master command support matrix

`Y` = command documented for that dialect; `—` = not present. **K3fam** = K3/K3S/KX3/KX2
shared set (Table 1 of the K-series reference). **K4** = K4 Programmer's Reference. **TX-500**
= Lab599 TX-500 (Kenwood-derived — note how its column tracks the *Kenwood* mnemonics, not
Elecraft's). Many K4-only and K3-only entries are Elecraft extensions; the well-known
commands are documented in §4/Part C.

| Cmd | K3fam | K4 | TX-500 | Notes |
|-----|:-----:|:--:|:------:|-------|
| ! / @ | Y | — | — | Direct DSP control (K3) |
| AB | — | Y | — | VFO copy/swap/init (K4) |
| AC | — | — | Y | Antenna tuner (TX-500, Kenwood-style) |
| AF | — | Y | — | Audio feedback tones (K4) |
| AG | Y | Y | Y | AF gain (`$` VFO B) |
| AI | Y | Y | — | Auto-info mode |
| AK | Y | — | — | ATU network values (K3) |
| AL | — | Y | Y | AF limiter (K4) / auto-notch level (TX-500) |
| AN | Y | Y | — | Antenna selection |
| AP | Y | Y | — | CW APF on/off |
| AR | Y | Y | — | RX antenna on/off |
| AT | — | Y | — | ATU control (K4) |
| BC | Y | — | — | Internal (K3) |
| BG | Y | Y | — | Bargraph read |
| BI | — | Y | — | Break-in (K4) |
| BL | — | Y | — | Balance control (K4) |
| BN | Y | Y | — | Band number (`$` VFO B) |
| BR | Y | Y | — | Baud rate set |
| BW | Y | Y | — | Filter bandwidth (`$`) |
| BY | — | — | Y | Busy status (TX-500) |
| CG | — | — | Y | Carrier gain (TX-500) |
| CN | — | — | Y | CTCSS number (TX-500) |
| CP | Y | Y | — | Speech compression |
| CT | — | — | Y | CTCSS on/off (TX-500) |
| CW | Y | Y | — | CW sidetone pitch |
| DA | — | Y | — | (K4) |
| DB | Y | Y | — | VFO B text |
| DE | Y | Y | — | Command processing delay |
| DL | Y | — | — | DSP command trace (K3) |
| DM | Y | Y | — | Internal use |
| DN | Y | — | — | VFO move down (K3; DNB variant) |
| DO | — | Y | — | DIGOUT state (K4) |
| DR | — | Y | — | (K4) |
| DS | Y | — | — | VFO A text/icons (K3) |
| DT | Y | Y | — | DATA sub-mode |
| DV | Y | Y | — | Diversity mode |
| DW | — | Y | — | (K4) |
| EC | — | Y | — | (K4) |
| EL | Y | — | — | Error logging (KX3/KX2) |
| ER | Y | Y | — | Internal use |
| ES | Y | Y | — | ESSB mode |
| EW | Y | — | — | Internal use (K3) |
| FA | Y | Y | Y | VFO A frequency (11-digit Hz; K4 flexible) |
| FB | Y | Y | Y | VFO B frequency |
| FC | — | Y | — | (K4) |
| FI | Y | Y | — | I.F. center frequency (GET) |
| FL | — | — | Y | Filter (TX-500) |
| FN | Y | — | — | Internal (K3) |
| FP | — | Y | — | (K4) |
| FR | Y | Y | — | Receive VFO select |
| FT | Y | Y | — | Transmit VFO select |
| FW | Y | — | — | Filter bandwidth & # (K3) |
| FX | — | Y | — | (K4) |
| GT | Y | Y | Y | AGC speed/on-off |
| HD | — | Y | — | (K4) |
| IC | Y | — | — | Icon & misc status (K3) |
| ID | Y | Y | Y | Radio identification |
| IF | Y | Y | Y | General information (status block) |
| IO | Y | Y | — | Internal use |
| IP | — | Y | — | (K4) |
| IS | Y | Y | Y | IF shift |
| K2 | Y | — | — | K2 command mode (meta) |
| K3 | Y | — | — | K3 command mode (meta) |
| KE | Y | — | — | Internal use (KX3/KX2) |
| KP | — | Y | — | (K4) |
| KS | Y | Y | Y | Keyer speed |
| KT | Y | — | — | Internal use (KX3/KX2) |
| KY | Y | Y | — | Keyboard CW/DATA send |
| LB / LC / LI / LO | — | Y | — | (K4 internal/link) |
| LD | Y | — | — | Internal use (K3) |
| LK | Y | Y | Y | VFO lock (`$`) |
| LN | Y | Y | — | Link VFOs |
| MA | — | Y | Y | Mode-select group (K4) / M→VFO (TX-500) |
| MB | — | Y | — | (K4) |
| MC | Y | Y | Y | Memory channel |
| MD | Y | Y | Y | Operating mode (`$`) |
| ME / MEDF | — | Y | — | Menu entry (K4) |
| MG | Y | Y | Y | Mic gain |
| MI | — | Y | — | (K4) |
| ML | Y | Y | Y | Monitor level |
| MN | Y | — | — | Menu entry number (K3) |
| MO | — | Y | Y | Menu open (K4) / (TX-500) |
| MP / MQ | Y | — | — | Menu param read/set (K3) |
| MR | — | — | Y | Memory read (TX-500) |
| MS | — | Y | — | (K4) |
| MW | — | — | Y | Memory write (TX-500) |
| MX | — | Y | — | Audio mix (K4) |
| NA / NM | — | Y | — | (K4 notch) |
| NB | Y | Y | Y | Noise blanker on/off (`$`) |
| NL | Y | — | Y | Noise blanker level (`$`) |
| NR | — | Y | Y | Noise reduction (K4/TX-500) |
| NT | — | — | Y | Auto notch (TX-500) |
| OM | Y | Y | — | Option modules |
| OV | — | Y | — | (K4) |
| PA | Y | Y | Y | RX preamp on/off (`$`) |
| PB | — | Y | — | (K4) |
| PC | Y | Y | Y | Power control |
| PK / PM / PN / PO / PP | Y/4 | Y | — | Various (PO=power out read) |
| PL | — | Y | Y | Speech proc level (K4/TX-500) |
| PR | — | — | Y | Speech processor on/off (TX-500) |
| PS | Y | Y | Y | Power on/off |
| PT | — | — | Y | Pitch/polarity (TX-500) |
| RA | Y | Y | Y | RX attenuator (`$`) |
| RC | Y | Y | — | RIT/XIT offset clear |
| RD | Y | Y | — | RIT down |
| RE | — | Y | — | (K4) |
| RG | Y | Y | Y | RF gain (`$`) |
| RO | Y | Y | — | RIT/XIT offset (absolute) |
| RP | — | Y | — | (K4) |
| RT | Y | Y | Y | RIT on/off |
| RU | Y | Y | — | RIT up |
| RV | Y | Y | — | Firmware revisions |
| RX | Y | Y | Y | Enter RX mode (unkey) |
| SB | Y | Y | — | Sub or dual watch |
| SC | — | Y | — | (K4 scan) |
| SD | Y | Y | — | QSK delay |
| SG / SI / SN | — | Y | — | (K4) |
| SM | Y | Y | Y | S-meter (`$`) |
| SMH | Y | Y | — | High-res S-meter |
| SP | Y | Y | Y | Internal (K3) / split (TX-500) |
| SQ | Y | Y | Y | Squelch level (`$`) |
| SS | — | Y | — | (K4) |
| SW / SWT / SWH | Y | Y | — | Switch tap/hold emulation |
| TA / TD / TG / TS / TU | — | Y | — | (K4) |
| TB | Y | Y | — | Buffered text |
| TE | Y | Y | — | TX EQ |
| TM | Y | Y | — | TX meter mode |
| TO | — | — | Y | Tone on/off (TX-500) |
| TP | — | — | Y | (TX-500) |
| TQ | Y | Y | — | TX query |
| TT | Y | — | — | Text-to-terminal (K3) |
| TX | Y | Y | Y | Enter TX mode (key) |
| UP | Y | — | — | VFO move up (K3; UPB variant) |
| VD | — | — | Y | VOX delay (TX-500) |
| VG | — | Y | Y | VOX gain (K4/TX-500) |
| VI / VO / VT / WM | — | Y | — | (K4) |
| VL / VV | — | — | Y | (TX-500; VV = A=B) |
| VX | Y | Y | Y | VOX state |
| XF | Y | Y | — | XFIL (crystal filter) number (`$`) |
| XL | Y | — | — | Internal use (K3) |
| XT | Y | Y | Y | XIT on/off |
| XV | — | Y | — | Transverter (K4) |

> **Matrix caveats.** The K3fam column is from Table 1 of the K-series Programmer's Reference
> (rev G4). "Internal use only" commands are listed for completeness but are not for general
> application use. K4 entries are parsed from the K4 reference's per-command headings (rev
> C7). Several rows collapse closely related mnemonics (`DN/DNB`, `UP/UPB`, `SWT/SWH`,
> `MP/MQ`). The TX-500 column reflects the Lab599 CAT manual (rev 2). A handful of one-dialect
> codes carry a terse "(model)" gloss where the exact function is dialect-specific.

---

## 4. Elecraft command reference (K3 family & K4)

Elecraft documents commands as free-form SET/GET/RSP format strings rather than
character-position grids. Layouts below use that style; `$` marks the VFO-B/sub variant.

### FA / FB — VFO A / B Frequency (GET/SET)

- **K3 family:** `FA` + 11-digit Hz + `;` (e.g. `FA00014060000;` = 14060 kHz). The 1-Hz
  digit is ignored unless the radio is in FINE mode (`SWT49`). Setting a frequency in another
  band **auto-changes band** (≈0.5 s, during which command handling is deferred); an
  out-of-band frequency snaps to the closest amateur/transverter band. If VFOs are linked
  (non-split), `FA` also sets VFO B.
- **K4:** `FAxxxxx;` accepts **multiple numeric formats** (e.g. `FA7100;` = 7100 kHz) — it is
  *not* restricted to a fixed 11-digit field like the K3/Kenwood. `FA;` GET returns the
  canonical form.

This K4 flexibility is a notable divergence from the otherwise-Kenwood-style fixed-width
frequency field.

### MD / MD$ — Operating Mode (GET/SET)

`MDn;` (or `MD$n;` for VFO B/sub). n per §2 (1 LSB … 9 DATA-REV). K4 adds `MD/;` (toggle last
two modes) and `MD+;`/`MD-;` (cycle SSB/CW/AM/FM/DATA). Use `DT` to pick the data sub-mode.

### DT — DATA Sub-Mode (GET/SET)

`DTn;` selects the data sub-mode used when `MD` = 6/9: `0` DATA A, `1` AFSK A, `2` FSK D,
`3` PSK D (K3). This is the command whose Sub-VFO variant (`DT$0;`) caused a Hamlib/K4
regression (see quirks).

### IF — Transceiver Information (GET only)

`IF;` → a fixed-layout status string, Kenwood-shaped:
`IF[f]*****+yyyyrx*00tmvspbd1;`

| Field | Meaning |
|---|---|
| `[f]` | Operating frequency, 11 digits (see `FA`), excludes RIT/XIT |
| `*` | space (0x20) padding |
| `+`/`-` | sign of RIT/XIT offset |
| `yyyy` | RIT/XIT offset in Hz (−9999…+9999 under computer control) |
| `r` | RIT on(1)/off(0) |
| `x` | XIT on(1)/off(0) |
| `t` | TX(1)/RX(0) |
| `m` | operating mode (see `MD`) |
| `v` | RX VFO: A(0)/B(1) |
| `s` | scan in progress |
| `p` | split on(1) |
| `b` | K2-extended: 1 if this `IF` is due to a band change (else 0) |
| `d` | K3-extended (`K31`): DATA sub-mode (0 DATA A,1 AFSK A,2 FSK D,3 PSK D) |

The fixed `*`/`0`/`1` fields exist for syntactic compatibility with Kenwood-era software.

### IS — IF Shift (GET/SET)

`IS␠nnnn;` (a space then a 4-digit AF center frequency Fc in Hz). To center the passband,
send `IS 9999;` then GET to read the resolved Fc. In AM-Sync, `IS 1400`/`IS 1600` pick LSB/
USB. Not applicable to FM or QRQ CW. In AI2/3, moving the physical SHIFT emits both `IS` and
`FW` responses.

### FR / FT — RX / TX VFO Select (GET/SET)

`FRn;` — on the K3, VFO A is always the receive VFO (n ignored); **any `FR` SET cancels
split**. `FTn;` — TX VFO: `0` VFO A, `1` VFO B; setting `FT1;` enables split. (Split on the
K-line = `FT1;`; on the K4 the same idea plus richer sub-RX handling.)

### FW / BW — Filter Bandwidth (K3)

`FW` reports/sets the filter bandwidth **and** crystal-filter number and is *modal* (affected
by the `K2`/`K3` meta-commands). `BW` is the **non-modal** bandwidth-only form preferred in
switch macros and when AI is off. K4 uses `BW`/`BW$`.

### AG / AG$ — AF Gain

`AGnnn;` where nnn = `000`–`060` (K3/K4). K4 adds `AG/;` toggle (mute/restore).

### MG — Mic Gain

`MGxxx;` where xxx = `000`–`060`.

### PC / PO — Power Control / Output

`PC` sets/reads the power-control (drive) setting; `PO` (KX3/KX2, K4) reads actual power
output. Ranges are model-specific (QRP KX-series vs 100 W K4).

### SM / SMH — S-Meter / High-Res S-Meter

`SM;` / `SM$;` return the S-meter; `SMH;` returns a high-resolution value (K3 `*` = not on
KX3/KX2 for the high-res form — check). Value scaling is Elecraft-specific (not Kenwood dots).

### SWT / SWH / SW — Switch Emulation

`SWTnn;` = tap, `SWHnn;` = hold a front-panel switch (K3 family); `SWnn;` (K4) is
tap/hold-agnostic. Example: `SWT16;` = tap XMIT (same code on K3/K3S and K4). A short delay is
needed before the effect shows in `TQ;`.

### TQ / IC — TX Query / Icon Status

`TQ;` returns transmit status; `IC;` returns front-panel icon/misc status. The reference
recommends these for state polling instead of continuous fast polling (especially in TX).

### AI — Auto-Info

`AIn;` per §1 (K3: 0–3; K4: 0–5 plus `AIDxxx;` delay). K4 AI is per-client.

### KY — Keyboard CW / DATA Send

`KY <text>;` buffers CW/DATA text for transmission. The `KY` prefix + one space + text; this
is the one command that is **case-sensitive** in PSK mode. A buffered variant (`TB`) exists.

### RC / RD / RU / RO / RT / XT — RIT/XIT

`RC;` clears the RIT/XIT offset; `RD;`/`RU;` step it down/up; `ROnnnn;` sets an absolute
offset; `RTn;` / `XTn;` turn RIT / XIT on/off (`0`/`1`).

---

## Part C — Lab599 TX-500

**The TX-500 command set is Kenwood-derived**, not Elecraft-derived. The Lab599 "CAT
Protocol" manual reproduces the Kenwood character-position table format, uses the Kenwood
error replies, and its `FA`/`IF`/`MD`/`SM` layouts match Kenwood's almost field-for-field.
It is included in this file per task scope, but a program that already speaks Kenwood
(TS-480/TS-590-class) will be far closer to the TX-500 than an Elecraft driver.

**Fundamentals.** 2-letter command + params + `;`; Set / Read / Answer directions.
**Error messages** (a Kenwood trait, spelled out in the TX-500 manual):

| Reply | Meaning |
|---|---|
| `?;` | Syntax error, **or** command valid but not executable in the current state |
| `E;` | Communication error (overrun/framing during serial transfer) |
| `O;` | Receive data sent but processing not completed |

Do not use control chars 00–1Fh (ignored or cause `?`). Program execution may lag while the
tuning knob is turned rapidly; received data isn't processed while a frequency is being
entered on the keypad.

### FA / FB — VFO A / B Frequency

`FA` + **11 digits (Hz)** + `;` — identical to Kenwood (e.g. `FA00014195000;` = 14.195 MHz;
blank digits `0`). Read `FA;`.

### MD — Operating Mode

`MD P1 ;`, Read `MD;`. **P1:** `0` none/failure, `1` LSB, `2` USB, `3` CW, `4` FM, `5` AM,
**`6` DIG**, `7` CW-R. (Note `6` = DIG here, a Lab599 label — the base 1–5 map matches
Kenwood; there is no FSK/FSK-R pair in the TX-500's list.)

### IF — Transceiver Status (read)

`IF;` → status block, **Kenwood-shaped, field-for-field like the TS-590** (§ Kenwood file):

| Pos | Field | Meaning |
|---|---|---|
| 3–13 | P1 | 11-digit displayed frequency (Hz) |
| 14–18 | P2 | 5 spaces |
| 19–23 | P3 | RIT/XIT frequency ±9990 Hz |
| 24 | P4 | RIT 0/1 |
| 25 | P5 | XIT 0/1 |
| 26–28 | P6,P7 | Memory channel (see `MC`) |
| 29 | P8 | 0 RX / 1 TX |
| 30 | P9 | Mode (see `MD`) |
| 31 | P10 | Function (see `FR`/`FT`) |
| 32 | P11 | Scan status |
| 33 | P12 | 0 simplex / 1 split |
| 34 | P13 | 0 OFF / 2 CTCSS ON |
| 35–36 | P14 | Tone/CTCSS number 00–42 |
| 37 | P15 | 0 always |

As on Kenwood, **`IF` cannot read status in Data mode**.

### Other TX-500 commands (Kenwood-style)

- **`FL`** — current filter (`FL P1 P2 ;`, P1 = filter number 0–3).
- **`MA`** — **DIG gain** `000`–`100` (Lab599-specific meaning; *not* the Kenwood `MA`).
- **`MG`** — mic gain `000`–`100`.
- **`ML`** — monitor level. **`PC`** — output power. **`PL`/`PR`** — speech processor
  level / on-off. **`PA`** — preamp. **`RA`** — attenuator. **`RG`** — RF gain.
- **`TX`/`RX`** — key / unkey. **`SP`** — split. **`SM`** — S-meter. **`RM`** — read meter.
- **`CN`/`CT`/`TN`/`TO`/`PT`** — CTCSS/tone control (Kenwood-style tone number + on/off).
- **`NB`/`NL`/`NR`/`NT`** — noise blanker (+level) / noise reduction / auto-notch.
- **`VX`/`VG`/`VD`** — VOX on-off / gain / delay. **`VV`** — VFO A=B. **`LK`** — lock.
- **`GT`** — AGC. **`IS`** — DSP IF set (`0` not set / `1` set + value). **`KS`** — keyer
  speed. **`ID`** — identification. **`BY`** — busy. **`PS`** — power on/off.
- **`MC`/`MR`/`MW`** — memory channel select / read / write. **`CG`** — carrier gain.
  **`AC`** — antenna tuner. **`AL`** — auto-notch level. **`MO`** — (menu/monitor).

Exact parameter widths and value maps follow the Kenwood conventions in the corresponding
`cat_kenwood.md` sections; where the TX-500 diverges (e.g. `MA` = DIG gain, `MD6` = DIG) it
is noted above.

---

## 5. Known quirks & gotchas (Elecraft / Lab599)

Cross-referenced from `quirks.md` (sources cited there).

**K3 / K3S / KX3 / KX2 (shared)**

- **No dedicated band-change command** — band change happens by setting frequency (`FA`),
  and the radio remembers the last mode *per band*. Sending `MD` before `FA` (or without a
  settle delay) can leave the wrong mode after a band change (e.g. auto-tuning to a DX spot).
- **Switch-emulation needs a settle delay** — after `SWT16;` (XMIT tap) etc., wait before a
  `TQ;` status read will reflect the change.
- **Busy → `?;`, not queued** — TX, BSET, and VFO-reverse limit which commands run; a busy
  radio returns `?;`. The reference recommends checking `TQ;`/`IC;` instead of fast-polling,
  and avoiding fast polling during TX.
- **KX3 split via Hamlib rejects the Sub VFO** — `kenwood_set_split_vfo` has failed with
  "unsupported VFO Sub" on the KX3 (a regression from earlier Hamlib).
- Historic reports of the Hamlib K3 driver hanging after extended use while other CAT
  software worked fine.

**K4**

- **Wrong Sub-RX data-mode command in a Hamlib build** — WSJT-X 2.5's bundled Hamlib sent
  `DT0;` (main VFO data mode) instead of `DT$0;` (sub VFO) when asked to put the sub RX in
  DATA mode, so the *main* VFO switched instead — breaking FT8/data split. The `$`-prefixed
  Sub-VFO variant was the one that was wrong.

**Lab599 TX-500**

- No specific, citable CAT bug was found (compatibility questions exist on the Lab599
  groups.io list, but nothing rising to a documented, reproducible quirk). Because the set
  is Kenwood-cloned, the general Kenwood cautions (hardware handshake, `TX` input selection)
  are reasonable starting assumptions.

---

## Coverage & Methodology Notes (Elecraft / Lab599)

**Method.** All three manuals were converted with `pdftotext -layout` and read directly. The
K3-family command set was taken from **Table 1** of the K3S/K3/KX3/KX2 Programmer's Reference
**rev G4** (33 pp), with individual command formats read from the body; the older **rev F2**
was available as a fallback but rev G4 was sufficient. The K4 command set was parsed from the
per-command headings of the **K4 Programmer's Reference rev C7** (45 pp). The TX-500 set was
parsed from the **Lab599 CAT Protocol rev 2** (11 pp), whose Kenwood-clone structure let the
`cat_kenwood.md` layouts carry most of the parameter detail.

**Fully analyzed:**

- K3/K3S/KX3/KX2 Programmer's Reference (rev G4) — command list (Table 1) fully captured;
  key commands (`FA`, `MD`, `DT`, `IF`, `IS`, `FR`/`FT`, `FW`/`BW`, `AI`, `SWT`, `TQ`) read
  in full and transcribed.
- K4 Programmer's Reference (rev C7) — all 121 command headings captured into the matrix;
  syntax model (GET/SET/TOGGLE/INCR/DECR, `$`, `#`, `SW`, meta-commands) and key commands
  transcribed.
- Lab599 TX-500 CAT Protocol (rev 2) — full command list captured; `FA`/`MD`/`IF`/`FL`/`MA`
  and the error-reply table transcribed; Kenwood relationship established from the `IF`/`FA`
  layouts.

**Partially analyzed / scoped out:**

- **Menu tables** (`MN`/`MP`/`MQ` on the K3, `ME`/`MEDF` and the `#` display commands on the
  K4) were not transcribed item-by-item — each is a large model-specific menu map; only the
  command structure and a pointer to the reference's menu tables are given.
- The full per-command parameter detail for the many **K4-only** commands (e.g. `AT`, `BL`,
  `DO`, `FP`, `MX`, `SG`, `TU`, `VT`, `WM`) is not individually transcribed; they appear in
  the matrix with a short gloss, and the K4 rev C7 reference is the authority.
- "Internal use only" K3 commands (`BC`, `EW`, `FN`, `KE`, `KT`, `LD`, `XL`, …) are listed
  for completeness but intentionally not detailed.
- TX-500 per-command parameter widths were mostly deferred to the Kenwood conventions rather
  than re-transcribed, given the clone relationship.

**Data-quality caveats:**

- The K3fam column is the Table 1 enumeration; a few closely related mnemonics are collapsed
  (`DN/DNB`, `UP/UPB`, `SWT/SWH`, `MP/MQ`). `*`/`**` functional applicability to KX3/KX2 is
  noted from Table 1 but not exhaustively re-verified per command.
- The TX-500 `FA`/`FB` were added to its parsed set by hand (documented as a combined
  "FA / FB" header the extractor skipped, same as on Kenwood).

**Total commands documented (Elecraft / Lab599):** **165 distinct mnemonics** across the
three dialects in the master matrix (§3) — K3 family ≈ 91 (Table 1), K4 = 121, Lab599 TX-500
≈ 52 — with the operationally important commands given per-command detail in §4 and Part C.
