# Config Commands

TBD

## Configuration File Location

TBD

## Initialization -- `init`

```bash
❯ rfham config init --config-file ./ex.toml --interactive K7SKJ

Hi K7SKJ, let's build a new configuration together ...

✓ What is your name? Simon Johnston
⊢ Using the country code derived from your callsign: 'US'
✓ What country are you in? US
⊢ Using the latitude/logitude derived from your IP address: '47.97900000, -122.20210000'
✓ What is your grid square? CN87vx
✓ Do you want to add a mailing address? No
✓ Do you want to set locale-specific defaults? Yes
✓ Use which units for length? imperial
✓ Use which units for temperature? imperial
✓ Use which format for time? military
✓ Do you want to add any equipment records? No
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

* Path to file: "/Users/skj/.config/rfham/rfham-config.toml"

## Locale

* Length Units: imperial
* Temperature Units: imperial
* Time Format: military

## Station

* Callsign: K7SKJ
  * ITU allocation; Country: US
* Operator name: Simon Johnston

### Location

* Grid locator: CN87WO61
* ITU region: 2
* Country: US

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
❯ rfham config show --compact station.location.grid-locator
grid-locator: string = "CN87WO61"
```

If the field requested is optional, but not present, the following is returned.

```bash
❯ rfham config show station.location.mailing-address
field mailing-address is not set
```

If however any name is not valid for the preceding component an error is returned.

```bash
❯ rfham config show station.location
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
