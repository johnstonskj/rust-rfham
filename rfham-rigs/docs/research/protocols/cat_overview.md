# CAT Protocol Overview — Cross-Vendor Comparison

> **Purpose.** `cat_yaesu.md`, `cat_kenwood.md`, and `cat_elecraft.md` each document one
> vendor family's CAT command set in full detail (per-command layouts, per-radio support
> matrices, quirks). This document is the **summary layer on top of those three**: it asks,
> across the three families, which commands are genuinely similar enough to reason about
> together, which share a mnemonic but mean something different (false friends), and what
> stylistic conventions the three protocols share or diverge on. It intentionally omits most
> parameter-level detail — follow the `[details]` links back to the family file for that.

---

## 1. The lineage, in one paragraph

Yaesu's command set is independent and was designed on its own. Kenwood's "PC Control"
command set is a separate independent design. **Elecraft's K-line (K3/K3S/KX3/KX2, K4) command
set is explicitly derived from and extends Kenwood's** — same 2-letter-mnemonic-plus-`;`
shape, the same `FA` 11-digit frequency field, and (K3 family) a Kenwood-shaped `IF` status
block. **Lab599's TX-500 goes further and is essentially a Kenwood clone** — its manual reuses
Kenwood's table format, error-reply set (`?;`/`E;`/`O;`), and field layouts almost verbatim.
So the real family tree is:

```text
Yaesu  (independent)
Kenwood  (independent)
 └─ Elecraft K3/K3S/KX3/KX2, K4  (Kenwood-derived, heavily extended)
 └─ Lab599 TX-500  (Kenwood-derived, minimally extended — near-clone)
```

This means "Kenwood vs Elecraft" comparisons below are mostly about *degree of extension*,
while "Yaesu vs (Kenwood/Elecraft)" comparisons are genuine independent-design differences.

---

## 2. What's genuinely shared across all three (protocol-level conventions)

These are real, load-bearing similarities — a developer moving between families will
recognize the shape immediately, even though the byte-for-byte content differs:

- **ASCII text framing terminated by `;`.** All three (unlike Icom CI-V, which is raw binary
  with `FE FE...FD` framing) send human-readable ASCII commands ending in a semicolon.
- **2-letter (occasionally 3–4 letter) command mnemonic + fixed-width parameters.** All three
  document per-command *character position* layouts and expect zero-padded numeric fields.
- **A three-way message direction model.** Yaesu/Kenwood call it Set / Read / Answer;
  Elecraft calls it SET / GET / RSP. Conceptually identical: send bare mnemonic to read
  (`FA;`), send mnemonic+value to set (`FA...;`), receive the same-shaped message back as the
  answer/response.
- **A "busy, try again" convention.** All three radios can answer `?;` when the command can't
  be processed right now (mid-TX, mid-menu, etc.). Elecraft's reference is explicit that this
  is retriable, not fatal; the same is true in practice for Yaesu and Kenwood (see each
  family's quirks section — this exact "treated as fatal instead of retriable" mistake shows
  up independently in Hamlib backends for **both** the Yaesu newcat family and (functionally)
  Kenwood/Elecraft polling code).
- **Auto-reporting exists in all three**, under the mnemonic `AI` in all three families,
  though the *richness* differs sharply (see §5).
- **A dedicated "read everything about the VFO" status command** exists in all three
  (`IF` in Yaesu and Kenwood/Elecraft/TX-500) — see §4 for how similar it actually is.
- **CTCSS/tone-number tables are conceptually parallel**: all three index a tone frequency by
  a 2-digit number rather than sending raw Hz, though the table contents/size differ
  (Yaesu/Kenwood: ~50 vs ~42 entries — confirm per radio).

---

## 3. Commands that are substantially the same across all three families

"Substantially the same" here means: same mnemonic, same operational concept, same basic
message shape (a 0/1 flag, or a same-order small parameter set), such that porting logic
between families mostly means changing field widths/ranges rather than rethinking the command.
Cross-checked against all three family matrices.

| Cmd                 | Concept                                       | Yaesu                                                                               | Kenwood                           | Elecraft/TX-500                                                         | What differs                                                                                                                                                                                                          |
|---------------------|-----------------------------------------------|-------------------------------------------------------------------------------------|-----------------------------------|-------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `PS`                | Power switch on/off                           | `PS0/1;`                                                                            | `PS0/1;`                          | `PS0/1;` (K3/K4/TX-500)                                                 | Power-on-over-CAT caveats differ (Yaesu: USB-only in practice; Kenwood: model-specific listen behavior) — see each family's quirks                                                                                    |
| `LK`                | Lock on/off                                   | `LK0/1;`                                                                            | `LK0/1;`                          | `LK0/1;` (`$` variant on Elecraft for VFO B)                            | None significant                                                                                                                                                                                                      |
| `RA`                | RF attenuator                                 | `RA` + step code                                                                    | `RA` + step code                  | `RA` + step code (`$` variant)                                          | Number/size of attenuation steps is model-specific in all three                                                                                                                                                       |
| `RG`                | RF gain                                       | `RG` 3-digit `000`–`255`                                                            | `RG` 3-digit `000`–`255`          | `RG` (`$` variant)                                                      | Elecraft's numeric range not confirmed identical — check K-series ref                                                                                                                                                 |
| `AG`                | AF gain                                       | `AG` 3-digit `000`–`255`                                                            | `AG` 3-digit `000`–`255`          | `AG` 3-digit `000`–`060` (K3/K4)                                        | **Elecraft's range is narrower** — same shape, different scale                                                                                                                                                        |
| `MG`                | Mic gain                                      | `MG` 3-digit (scale varies by model)                                                | `MG` 3-digit                      | `MG` 3-digit `000`–`060`                                                | Range varies per radio in every family — check before assuming a scale                                                                                                                                                |
| `SQ`                | Squelch level                                 | `SQ` 3-digit (`000`–`100` or `000`–`255` by model)                                  | `SQ` 3-digit `000`–`255`          | `SQ` (`$` variant)                                                      | Range varies by radio within each family too                                                                                                                                                                          |
| `PA`                | Preamp on/off                                 | `PA` + step (IPO/AMP1/AMP2)                                                         | `PA0/1;`                          | `PA0/1;` (`$` variant)                                                  | Yaesu exposes multiple preamp stages; Kenwood/Elecraft are simple on/off on most models                                                                                                                               |
| `NB`                | Noise blanker on/off                          | `NB0/1;`                                                                            | `NB0/1;`                          | `NB0/1;` (`$` variant)                                                  | None significant                                                                                                                                                                                                      |
| `KS`                | Keyer speed (WPM)                             | `KS` 3-digit WPM                                                                    | `KS` 3-digit WPM                  | `KS` WPM                                                                | Ranges are model-specific in all three                                                                                                                                                                                |
| `VX` / `VG` / `VD`  | VOX on-off/gain/delay                         | present, standard triad                                                             | present, standard triad           | present (K4/TX-500)                                                     | None significant — this triad is one of the most consistent across all three vendors                                                                                                                                  |
| `RT` / `XT`         | RX clarifier(RIT) / TX clarifier(XIT) on-off  | `RT`=RX clarifier, `XT`=TX clarifier, both `P1` 0/1                                 | `RT`=RIT, `XT`=XIT, both `P1` 0/1 | `RT`/`XT` same 0/1 shape                                                | **Naming differs** (Yaesu calls it "clarifier", Kenwood/Elecraft "RIT/XIT") but the on/off mechanic and even the `RT`/`XT` split between RX-side and TX-side is identical across all three — a genuinely strong match |
| `RC`                | Clear RIT/XIT/clarifier offset                | `RC;` zeroes clarifier                                                              | `RC;` zeroes RIT/XIT              | `RC;` zeroes RIT/XIT                                                    | None significant                                                                                                                                                                                                      |
| `ID`                | Radio identification                          | 4-digit model code                                                                  | 3-digit model code                | present, digit width per family reference                               | **Digit width differs** — do not assume a fixed width across vendors                                                                                                                                                  |
| `PC`                | Output power control                          | 3-digit, `005`–`100`/`000`–`255`/up to `200` depending on model                     | 3-digit, mode-dependent range     | 3-digit/model-specific (QRP vs 100 W K4)                                | Same shape, but **the numeric range is a per-radio landmine in every single family** — always check `ID` first                                                                                                        |
| `AI`                | Auto-information toggle                       | binary `0`/`1`                                                                      | multi-level `0`–`3`               | richer multi-level, K4 per-client `0`–`5`                               | Shape (mnemonic + read/set) is shared; **sophistication is not** — see §5                                                                                                                                             |
| `MD`                | Operating mode                                | 1-char mode code                                                                    | 1-char mode code                  | 1-char mode code (`$` variant)                                          | **Code numbering differs** — see §4.2, this is a false-friend-adjacent case: same shape, mostly-incompatible content                                                                                                  |
| `FA`/`FB`           | VFO A/B frequency                             | 9-digit Hz                                                                          | 11-digit Hz                       | 11-digit Hz (K3/TX-500); **K4 uses a flexible non-fixed-width format**  | The headline example — see §4.1                                                                                                                                                                                       |
| `IF`                | Full VFO status block                         | present, Yaesu-native field order                                                   | present, Kenwood field order      | present, **field-for-field the same as Kenwood** on K3 family/TX-500    | Kenwood ↔ Elecraft/TX-500 are near-identical; Yaesu is a different layout entirely — see §4.3                                                                                                                         |

**Reading this table:** the takeaway is that Elecraft/TX-500 track Kenwood closely on
*simple flag and level commands* (the bottom two-thirds of the table above), while the
*headline "read the whole VFO state" commands* (`FA`, `MD`, `IF`) are where the real
compatibility work is concentrated — and that's true both across independent families (Yaesu
vs. everyone) and, to a lesser extent, within the Kenwood-derived branch (K4 vs. K3/TX-500).

---

## 4. The headline compatibility cases, in more detail

### 4.1 `FA`/`FB` — frequency field width

The single most consequential cross-vendor difference. All three use the same *idea*
(2-letter command + zero-padded Hz digits + `;`), but:

- **Yaesu: 9 digits.** `FA014250000;` = 14.250000 MHz.
- **Kenwood: 11 digits.** `FA00014195000;` = 14.195000 MHz.
- **Elecraft K3/K3S/KX3/KX2 and Lab599 TX-500: 11 digits**, matching Kenwood.
- **Elecraft K4: breaks the fixed-width convention entirely** — it accepts multiple numeric
  formats (e.g. `FA7100;` for 7100 kHz) rather than requiring a fixed 11-digit field. This is
  a divergence *within* the Elecraft family, not just from Kenwood.

Any code written against one family's frequency field width will silently miscompute against
another's — this is the #1 thing to check when adapting logic between families.
[Yaesu detail](cat_yaesu.md) · [Kenwood detail](cat_kenwood.md) · [Elecraft detail](cat_elecraft.md)

### 4.2 `MD` — mode code numbering

| Code | Yaesu (modern)                               | Kenwood         | Elecraft/TX-500 |
|------|----------------------------------------------|-----------------|-----------------|
| 1    | LSB                                          | LSB             | LSB             |
| 2    | USB                                          | USB             | USB             |
| 3    | CW-U                                         | CW              | CW              |
| 4    | FM                                           | FM              | FM              |
| 5    | AM                                           | AM              | AM              |
| 6    | RTTY-L                                       | FSK             | DATA            |
| 7    | CW-L                                         | CW-R            | CW-REV          |
| 8    | DATA-L                                       | (none/failure)  | (N/A)           |
| 9    | RTTY-U                                       | FSK-R           | DATA-REV        |
| A–F  | DATA-FM, FM-N, DATA-U, AM-N, PSK, DATA-FM-N  | *(not used)*    | *(not used)*    |

Codes **1–5 and 7 are Kenwood- and Elecraft-compatible with each other** (this is the clearest
evidence of the Kenwood→Elecraft lineage — even `7`=CW-reverse survives unchanged). **Yaesu's
1–5 numbering happens to visually line up** for LSB/USB/FM/AM, but Yaesu splits CW into
CW-U/CW-L rather than CW+CW-reverse, and diverges completely from position 6 onward (Yaesu
uses letter codes `A`–`F` for a much larger mode set that has no Kenwood/Elecraft
counterpart). Treat position 6/7/9 as **not** portable between Yaesu and the other two even
though the mnemonic and message shape match. Full per-radio mode tables:
[Yaesu §3](cat_yaesu.md) · [Kenwood §2](cat_kenwood.md) · [Elecraft §2](cat_elecraft.md)

### 4.3 `IF` — status block

- **Kenwood and Elecraft/TX-500 are field-for-field nearly identical**: 11-digit frequency,
  5-space pad, signed RIT/XIT offset, RIT flag, XIT flag, memory channel, TX/RX flag, mode,
  function, scan status, split flag, tone mode, tone number — same order, same widths, on
  both. Elecraft's reference literally documents `IF` as Kenwood-shaped for compatibility
  with "Kenwood-era software." A Kenwood `IF` parser will parse a K3-family or TX-500 `IF`
  response correctly with zero changes (aside from the K3-extended `d` DATA-submode
  trailing field under `K31` mode).
- **Yaesu's `IF` is a structurally different layout**: memory-channel-or-PMS-code field first,
  then 9-digit frequency, then clarifier as `sign+4-digit`, separate RX/TX clarifier flags,
  mode, VFO/memory/QMB state, CTCSS state, then a fixed `00` pad, then simplex/shift state.
  Different field count, different order, different widths — there is no shortcut here; a
  Yaesu `IF` parser is a separate implementation, not a relabeling of the Kenwood one.
- **TS-890S/TS-990S dropped `IF` entirely** in favor of finer-grained per-item read commands —
  so even within the Kenwood family, "does this radio have `IF`" is not universal.

Full layouts: [Yaesu §5 "IF"](cat_yaesu.md) · [Kenwood §4 "IF"](cat_kenwood.md) ·
[Elecraft §4 "IF"](cat_elecraft.md)

---

## 5. Auto-Info (`AI`) — same mnemonic, very different sophistication

All three use `AI` for unsolicited status push, but the granularity tells you something real
about each vendor's design philosophy:

- **Yaesu:** binary. `AI0` off, `AI1` on (any front-panel state change triggers an Answer
  message). Resets to off at power-off.
- **Kenwood:** three or four levels depending on model. `AI0` off, `AI1` "old format" auto-
  report, `AI2` "extended format", (TS-480) `AI3` both. Still resets at power-off.
- **Elecraft:** the richest of the three. K3 family: `AI0`–`AI3` (off / VFO-class changes /
  all changes / reserved). **K4: `AI0`–`AI5`**, distinguishing *periodic* reporting from
  *immediate* reporting, plus a separate `AIDxxx;` command to tune the report delay
  (60–999 ms), and — uniquely — **AI mode is tracked per-client**, since the K4 supports
  multiple simultaneous control connections (USB/RS-232/Ethernet) that may each want
  different reporting behavior.

This is a case where the *concept* is fully shared but the *design sophistication* increases
monotonically Yaesu → Kenwood → Elecraft K3 → Elecraft K4, tracking each design's era and the
K4's explicit multi-client architecture.

---

## 6. Same mnemonic, incompatible meaning (false friends)

These are the dangerous cases: identical 2-letter code, present in two or more families, that
does **something functionally different**. Code written against one family that happens to
compile/run against another's serial stream will silently do the wrong thing.

| Cmd         | Yaesu meaning                                                                                                                                     | Kenwood meaning                                                                                                                       | Elecraft meaning                                                                                      | Risk                                                                                                                                                                                |
|-------------|---------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `ST`        | **Split** (modern radios) *or* **tuning Step** (FT-450/450D) — this is even a false friend **within** the Yaesu family itself                     | "Step" (multi-channel step, TS-480/2000 only)                                                                                         | Not used (split is set via `FT1;` instead)                                                            | High — sending `ST1;` to an FT-450D sets a tuning step, not split; sending it to a Kenwood sets a channel step, not split either. Only on modern Yaesu radios does `ST` mean split. |
| `GT`        | AGC **speed selector**: OFF/FAST/MID/SLOW/AUTO (discrete states)                                                                                  | AGC **time constant**: continuous step `01`–`20`, and cannot be read when AGC is off or in FM                                         | AGC "speed/on-off" (K3fam/K4/TX-500) — closer in spirit to Yaesu's discrete-state model               | Medium — a Kenwood driver treating `GT` as a small integer selector rather than a time-constant scale will send nonsense values                                                     |
| `TX`        | Simple PTT state: `0`/`1`/`2` (RADIO/CAT TX flags)                                                                                                | **Selects the TX audio source** — `TX0;`/`TX;` picks the **MIC** input, `TX1;` picks the **ANI/line-in** input used for digital modes | Simple key/unkey (`TX;`/`RX;`)                                                                        | **High, and a real-world recurring bug** —  **See note**                                                                                                                            |
| `SM`        | S-meter, raw scale `000`–`255`                                                                                                                    | S-meter, **dot count** `0000`–`0030`                                                                                                  | S-meter, Elecraft-specific scaling (not "dots")                                                       | Medium — treating a Kenwood dot-count as a Yaesu-style 0–255 raw reading (or vice versa) misrepresents signal strength by roughly an order of magnitude                             |
| `PC`        | Power, scale is **model-dependent within Yaesu itself** (`000`–`255` on plain FT-450 vs `005`–`100` watts on FT-450D vs up to `200` on FTDX101MP) | Power, watts, mode-dependent range (`005`–`100` SSB/CW/FM, `005`–`025` AM)                                                            | Power, QRP-scale on KX-series vs 100 W-scale on K4                                                    | High within *every* family, not just across them — always confirm via `ID`/model before trusting a `PC` range assumption                                                            |
| `AC`        | **Antenna tuner control**                                                                                                                         | **Antenna tuner control** (same concept — not actually a false friend)                                                                | Not in K3fam/K4 matrix under this exact mnemonic; **present on TX-500** matching the Kenwood meaning  | Low — flagged here only because it's easy to assume `AC` means something electrical/AC-power-related; it doesn't, in any family                                                     |
| `EX` / `ME` | `EX` = Menu (item number + value, two structurally different sub-formats depending on radio generation)                                           | `EX` = Menu (extended settings), same general shape                                                                                   | K3 uses `MN`/`MP`/`MQ` for menu access; **K4 uses `ME`/`MEDF`**, a different mnemonic entirely        | Medium — even the *mnemonic itself* for "access a menu item" is not stable, let alone the item-numbering scheme, which is 100% model-specific in every family                       |

**Note**: software (including historical Hamlib) that keys a Kenwood with `TX0` expecting plain PTT instead silently switches the radio to mic input, muting the USB/ACC digital-mode audio path. This exact issue is documented in `quirks.md` for both TS-480 and TS-590.

---

## 7. Style differences worth knowing about

- **VFO-B addressing style differs philosophically.** Yaesu and Kenwood give VFO-B its own
  parallel mnemonic (`FB` alongside `FA`, a separate command per VFO). **Elecraft uses a `$`
  suffix modifier** on the *same* mnemonic (`FA$;` / `AG$;` / `MD$3;`) to mean "apply this to
  VFO B / the sub receiver" — a more compositional design that avoids doubling the command
  set, but means an Elecraft parser has to handle the modifier as a cross-cutting concern
  rather than a fixed list of "VFO-B commands."
- **Error-reply richness differs.** Yaesu and (implicitly) Kenwood/Elecraft all use `?;` for
  "busy or invalid." **Kenwood and the Kenwood-derived TX-500 additionally distinguish**
  `E;` (communication/framing error) from `O;` (received but not yet processed) — a
  three-state error vocabulary that Yaesu's manuals don't document an equivalent for.
- **K4 introduces operator syntax beyond GET/SET.** Toggle (`RA/;`), increment (`BN$+;`), and
  decrement (`BN$-;`) suffixes exist only on the K4 — a stateful-adjustment shorthand that has
  no equivalent anywhere in Yaesu, Kenwood, or the K3/TX-500 command sets.
- **Meta-commands for backward compatibility exist only in the Elecraft family.** `K2`, `K3`
  command-mode switches (and the K4's own compatibility handling) let a K-series radio
  reinterpret its own command set for older software. Yaesu and Kenwood have no equivalent —
  instead, Yaesu/Kenwood backward compatibility is handled by keeping legacy commands present
  (and just documenting the model differences per-command, as in most of the table in §3).
- **Serial framing details are the most similar thing between Yaesu and Kenwood, ironically.**
  Both are typically 8N2-or-8N1 depending on baud, with menu-selectable rates in the same
  4800–38400+ bps neighborhood, and both note that RTS/CTS hardware handshaking matters for
  reliability (documented explicitly for Kenwood; a real-world Yaesu quirk — RTS/DTR
  misinterpreted as PTT — is the flip side of the same underlying hardware-handshake concern).
  This is a physical-layer similarity, not a protocol-content one, but worth knowing when
  debugging "CAT sort of works but drops"-class problems across any of these radios.
- **Busy/"can't process now" handling is conceptually shared but not formally specified as
  retriable in most manuals.** Only the Elecraft reference explicitly tells the programmer to
  treat `?;` as retriable and to poll a status command (`TQ;`/`IC;`) instead of hammering the
  busy command. The Yaesu and Kenwood manuals mostly just document that `?;` *can* happen;
  the "this is retriable, not fatal" lesson in those two families comes from quirks/bug
  reports (see each family file's quirks section), not the manuals themselves.

---

## 8. What this document deliberately leaves out

- Per-radio value ranges and menu-item maps — every family file scopes those out too; they're
  too large and too model-specific to summarize usefully.
- The Yaesu FT-857D/FT-817ND **binary** protocol is not compared here at all — it's a
  different framing model (5-byte binary opcodes, no ASCII, no `;` terminator) with no
  meaningful stylistic overlap to Kenwood/Elecraft/modern-Yaesu ASCII CAT. See
  [cat_yaesu.md §6](cat_yaesu.md) for that protocol on its own terms.
- This document does not re-derive anything from the manuals directly — it is entirely a
  synthesis of what `cat_yaesu.md`, `cat_kenwood.md`, and `cat_elecraft.md` already
  established. If a comparison here looks wrong, the fix is most likely needed in the
  underlying family file first.
