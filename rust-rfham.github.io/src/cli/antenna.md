# Antenna Commands

## Length Calculations -- `length`

```bash
❯ rfham antenna length --band 6m --country US

# Classic 6m single-band dipole antenna.

~~~text
|<────────────────── λ/2 = 2.883 meters ───────────────────>|
|<─── λ/4 = 1.441 meters ───>| |<─── λ/4 = 1.441 meters ───>|
──────────────────────────────┳──────────────────────────────
                              ⎅    1:1 balun
                              │  ∧
                              │  │ λ/2 = 2.883 meters
                              │  │ 50Ω feed line
                              │  │
                              │  ∨
                              └┄┄┄ > to transceiver
~~~

Notes:

1. Frequency range for 6m band is 50.000 MHz - 54.000 MHz.
   1. From the *US Amateur Radio Bands* by The American Radio Relay League (ARRL).
2. Mid-point of band is 52.000 MHz.
3. Wavelength of mid-point is 5.765 m.
4. Half-wave length is λ/2 = 2.883 meters for overall antenna.
5. Quarter-wave length is λ/4 = 1.441 meters for each antenna pole.
6. Include a 1:1 Current **Balun**, often called a 1:1 *Choke* or *Line Isolator*.
```
