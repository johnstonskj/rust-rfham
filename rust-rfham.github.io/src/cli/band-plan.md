# Band-Plan Commands

## ITU Allocations -- `itu`

```bash
❯ rfham band-plan itu
# IARU/ITU Frequency Allocations

|       |       |   Region 1 |            |   Region 2 |            |  Region. 3 |            |
| Name  | Band  |      Start |        End |      Start |        End |      Start |        End |
| ----- | ----- | ---------: | ---------: | ---------: | ---------: | ---------: | ---------: |
| 2200m | LF    | 0.1357 MHz | 0.1378 MHz | 0.1357 MHz | 0.1378 MHz | 0.1357 MHz | 0.1378 MHz |
| 630m  | MF    |  0.472 MHz |  0.479 MHz |  0.472 MHz |  0.479 MHz |  0.472 MHz |  0.479 MHz |
| 160m  | MF    |   1.81 MHz |      2 MHz |    1.8 MHz |      2 MHz |    1.8 MHz |      2 MHz |
| 80m   | HF    |    3.5 MHz |    3.8 MHz |    3.5 MHz |      4 MHz |    3.5 MHz |    3.9 MHz |
| 60m   | HF    | 5.3515 MHz | 5.3665 MHz | 5.3515 MHz | 5.3665 MHz | 5.3515 MHz | 5.3665 MHz |
| 40m   | HF    |      7 MHz |    7.3 MHz |      7 MHz |    7.3 MHz |      7 MHz |    7.3 MHz |
| 30m   | HF    |   10.1 MHz |  10.15 MHz |   10.1 MHz |  10.15 MHz |   10.1 MHz |  10.15 MHz |
| 20m   | HF    |     14 MHz |  14.35 MHz |     14 MHz |  14.35 MHz |     14 MHz |  14.35 MHz |
| 17m   | HF    | 18.068 MHz | 18.168 MHz | 18.068 MHz | 18.168 MHz | 18.068 MHz | 18.168 MHz |
| 15m   | HF    |     21 MHz |  21.45 MHz |     21 MHz |  21.45 MHz |     21 MHz |  21.45 MHz |
| 12m   | HF    |  24.89 MHz |  24.99 MHz |  24.89 MHz |  24.99 MHz |  24.89 MHz |  24.99 MHz |
| 10m   | HF    |     28 MHz |   29.7 MHz |     28 MHz |   29.7 MHz |     28 MHz |   29.7 MHz |
| 6m    | VHF   |     50 MHz |     54 MHz |     50 MHz |     54 MHz |     50 MHz |     54 MHz |
| 2m    | VHF   |    144 MHz |    148 MHz |    144 MHz |    148 MHz |    144 MHz |    148 MHz |
| 1.25m | VHF   |          - |          - |    220 MHz |    230 MHz |          - |          - |
| 70cm  | UHF   |    430 MHz |    440 MHz |    430 MHz |    440 MHz |    430 MHz |    440 MHz |
| 33cm  | UHF   |          - |          - |    902 MHz |    928 MHz |          - |          - |
| 23cm  | UHF   |   1240 MHz |   1300 MHz |   1240 MHz |   1300 MHz |   1240 MHz |   1300 MHz |
| 13cm  | UHF   |   2300 MHz |   2450 MHz |   2300 MHz |   2450 MHz |   2300 MHz |   2450 MHz |
| 9cm   | SHF   |   3300 MHz |   3500 MHz |   3300 MHz |   3500 MHz |   3300 MHz |   3500 MHz |
| 5cm   | SHF   |   5650 MHz |   5850 MHz |   5650 MHz |   5925 MHz |   5650 MHz |   5850 MHz |
| 3cm   | SHF   |  10000 MHz |  10500 MHz |  10000 MHz |  10500 MHz |  10000 MHz |  10500 MHz |
| 1.2cm | SHF   |  24000 MHz |  24250 MHz |  24000 MHz |  24250 MHz |  24000 MHz |  24250 MHz |
| 6mm   | EHF   |  47000 MHz |  47200 MHz |  47000 MHz |  47200 MHz |  47000 MHz |  47200 MHz |
| 4mm   | EHF   |  76000 MHz |  81500 MHz |  76000 MHz |  81500 MHz |  76000 MHz |  81500 MHz |
| 2.5mm | EHF   | 122250 MHz | 123000 MHz | 122250 MHz | 123000 MHz | 122250 MHz | 123000 MHz |
| 2mm   | EHF   | 134000 MHz | 141000 MHz | 134000 MHz | 141000 MHz | 134000 MHz | 141000 MHz |
| 1mm   | EHF   | 241000 MHz | 250000 MHz | 241000 MHz | 250000 MHz | 241000 MHz | 250000 MHz |

For more information, see:

* [Amateur and Amateur-satellite Service Spectrum](https://www.iaru.org/wp-content/uploads/2020/01/Amateur-Services-Spectrum-2020_.pdf), IARU 2020.
* [Regions](https://www.iaru.org/about-us/organisation-and-history/regions/), IARU.
```

## List Band Plans -- `list`

```bash
❯ rfham band-plan list

# Configured/Known Band Plans

| Country  | Plan Name                      | Region   |
| -------- | ------------------------------ | -------- |
| UK       | UK Amateur Radio Band Plan     | 1        |
| US       | US Amateur Radio Band Plan     | 2        |
```

## Show Band Plan -- `show`

```bash
Show a given country's band plan

Usage: rfham band-plan show [OPTIONS] <COUNTRY>

Arguments:
  <COUNTRY>  Show the band plan for this country [env: RFHAM_COUNTRY=]

Options:
  -b, --band <BAND>   Show only this band
  -v, --verbose...    Increase logging verbosity by one level per occurance
  -q, --quiet...      Decrease logging verbosity by one level per occurance
      --color <WHEN>  Controls when to use color [default: auto] [possible values: auto, always, never]
  -h, --help          Print help
```

```bash
❯ rfham band-plan --band 6m US
# US Amateur Radio Bands

* Regulatory Agency: [Federal Communications Commission (FCC)](https://www.fcc.gov)
* Maintaining Agency: [The American Radio Relay League (ARRL)](http://www.arrl.org)
* Region: IARU/ITU Region 2
* Countries: US
* Default maximum power: 1500 W

Notes:

* An amateur station must use the minimum transmitter power necessary to carry out the desired communications.

## License Classes

0. Novice (N) inactive class
1. Technician (T)
2. General (G)
3. Advanced (A) inactive class
4. Amateur Extra (E)

## 6m Band (50 MHz - 54 MHz)

Note: amateur operators are the primary users on this band.

Band restrictions:

| Start      | End        | License Class  | Usage/Mode           | Power          | Max Bandwidth  |
| ---------: | ---------: | -------------- | -------------------- | -------------- | -------------- |
|     50 MHz |   50.1 MHz | any            | CW only              | full power     | -              |
|   50.1 MHz |     54 MHz | any            | any                  | full power     | -              |
```
