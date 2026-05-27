# Config Commands

TBD

## Configuration File Location

TBD

## Initialization -- `init`

```text
? What kind of station should be your default?  
❯ Home
  Alternate
  Remote
  Club
  Temporary
[↑↓ to move, enter to select, type to filter]
```

```text
? Label this station (esc to skip)? (My home/primary location)
```

```text
? Do you want to set locale-specific defaults? (y/N)  
```

```text
? Used in?  
❯ [ ] Local
  [ ] Qrp
  [ ] Dx
  [ ] EmComm
  [ ] Activation
  [ ] Satellite
  [ ] Scanning
[↑↓ to move, space to select one, → to all, ← to none, type to filter]
```

```text
? Max Power in Watts (esc to skip)?  
```

```bash
❯ rfham config init --config-file ./ex.toml --interactive K7SKJ

Hi K7SKJ, let's build a new configuration together ...

✓ What is your name? Simon Johnston

Now, let's talk about your station ...

✓ What kind of station should be your default? Home
✓ Label this station (esc to skip)? My home/primary location
⊢ Using the country code derived from your callsign: 'US'
✓ What country are you in? US
⊢ Using the latitude/logitude derived from your IP address: '48.05180000, 122.17710000'
✓ What is your grid square? PN18cb
✓ Do you want to add a mailing address? No

Next, we can set some locale-specific defaults ...

✓ Do you want to set locale-specific defaults? Yes
✓ Use which units for length? imperial
✓ Use which units for temperature? imperial
✓ Use which format for time? military

Finally, we can set some optional connections ...

✓ Do you want to add any equipment records? Yes
✓ Brand Name? Icom
✓ Model Name/ID? 705
✓ Label this record (esc to skip)? IC-705
✓ Used in? Local, Qrp, Dx, EmComm
✓ Used for? Fm, Ssb, Digital
✓ Mobility kind? StationFixed
✓ Supported Bands? HF, VHF, UHF
✓ Max Power in Watts (esc to skip)? 10
✓ Add another? No
✓ Do you want to connect to any web services? Yes
✓ Do you have an account on qrz.com for callsign lookup? Yes
✓ Qrz user name? K7SKJ
✓ Password: ********
✓ Are you sure you wish to write this configuration? Yes
✓ Configuration file saved as "./ex.toml"
```

## Current Configuration -- `show`

```bash
❯ rfham config show

# Current Configuration

* Path to file: "./ex.toml"

* Operator callsign: K7SKJ
  * Callsign's ITU allocation; Country: US
* Operator name: Simon Johnston

## Locale

* Length Units: imperial
* Temperature Units: imperial
* Time Format: military

## My home/primary location (default)

### Location

* Grid locator: CN87wo
* Country: US

## Equipment

### IC-705

* Make: Icom
* Model: 705
* Usage: Local, DX, QRP
* Modes: Digital, FM, SSB
* Mobility: StationFixed
* Operating Bands: UHF, HF, VHF
* Max Power: 10 W

## Services

* Credential storage: plain-text

### Credentialed Services

* **qrz-api** for K7SKJ
```

## Current Configuration Field Access

Adding a *config path* to the end of the show command will retrieve only the specified field
by traversing each name in the path as a field from the file root. So the `station` component is
referenced as `station`, the call-sign field within this component is `station.call-sign`.

The following retrieves the Maidenhead grid locator configured for the station.

```bash
❯ rfham config show station.location.grid-locator
field: grid-locator
 type: string
value: "CN87WO61"
```

The flag `-c` or `--compact` will display the same information is a compact format of the form
*name `:` type `=` value*. This provides a more easily machine-readable format for extensions.

```bash
❯ rfham config show --compact station.0.location.grid-locator
grid-locator: string = "CN87WO61"
```

If the field requested is optional, but not present, the following is returned.

```bash
❯ rfham config show station.0.location.mailing-address
field mailing-address is not set
```

If however any name is not valid for the preceding component an error is returned.

```bash
❯ rfham config show station.0.location
🛑 Error: the config path name `location` expected additional path elements
   └── 🔎 Component `station`
   └── ℹ️  Help Possible names: call-sign, operator-name, location
```

Similarly, if a component is selected by there are no more elements in the path the overall
path is invalid as not field can be selected.

```bash
❯ rfham config show station
🛑 Error: the config path name `station` expected additional path elements
   └── 🔎 Component `<<root>>`
   └── ℹ️  Help Possible names: call-sign, operator-name, location
```

Finally, if a component is selected but there are no more elements in the path the overall
path is invalid as not field can be selected.

```bash
❯ rfham config show station.name     
🛑 Error: the config path name `name` is not valid in the matching component
   ├── 🔎 Component `station`
   └── ℹ️  Help Possible names: call-sign, operator-name, location
```

## Schema

TBD
