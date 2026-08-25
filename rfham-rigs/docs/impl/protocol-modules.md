# Formatting rules for protocol modules

This document lays out the rules for formatting the CAT and CI-V protocol modules
using the macros defines at different levels of the module hierarchy. The initial
focus (and therefore all examples) will be on the CAT based transcerivers as there
are currently more available to work with.

## Module Structure

Modules MUST be organized so that type definitions are grouped together before
implementations. To accomplish this the following basic template should be used
with the two comment blocks REQUIRED to separate the two sections of the module.

```rust
//! see #header

// imports here

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// see #defining-commands

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// see #implementing-commands
```

### Header

Every header should follow the following basic outline although additional
details may be added. Add any reference documents with links to sources by URL.
Specifically call out revisions and dates of specifications used as the source
for the implementation.

```rust
//!
//! {protocol} commands for the {Brand} {Models} transceivers.
//! 
//! {add additional commentary here}
//! 
//! Commands follow the specification in reference **1** unless otherwise noted.
//!
//! # References
//! 
//! 1. Primary specification Link
//! 
```

## Defining Commands

Commands MUST use the macro `define_command!`, unless there are specific reasons
why this macro cannot capture the needs of a specific command. Additionally, the
macros `define_command_struct` and `define_command_enum` ensure that all the
currect derive macros and other features are provided in common across all types.

```rust
// ------------------------------------------------------------------------------------------------
// Public Types: GetRxAntenna, SetRxAntenna
// ------------------------------------------------------------------------------------------------

define_command!("Query the RX-only antenna state (K3/K3S only).

# Command format

> `AR;` 

# Response format

> `AR{n};` 

Where *n* is one of:

* `0`; use Transmit antenna,
* `1`; use Receive-only antenna." =>
    GetReceiveAntenna
);

define_command!("Set RX-only antenna.

# Command format

> `AR{n};`

Where *n* is one of:

* `0`; use Transmit antenna,
* `1`; use Receive-only antenna." =>
    SetReceiveAntenna {
        rx_only: bool
    }
);
```

Any additional types used as responses or parameters to set should be defined
*after* the commands themselves.

### Naming Commands

```bnf
CommandName ::= 
    VfoCommand


VfoCommand ::=
    VfoProperties | VfoActions

VfoProperties ::=
    ( 'Get' | 'Set' ) ( 'VfoA' | 'VfoB' )
    (
        ( 'Band' 'Number' ) |
        ( 'DisplayText' ) |
        ( 'Filter' 'Preset' ) |
        ( 'IF' 'Shift' ) |
        ( 'Legacy'? 'Filter' 'BandWidth' ) |
        ( 'Lock' ) |
        ( 'Operating' ( 'Frequency' | 'Mode' ) ) |
        ( 'Noise' ( 'Reduction' | ( 'Blanker' 'Level'? ) ) ) |
        ( 'Preamp' ) |
        ( 'Receive' 'Attenuator' ) |
        ( 'RfGain' ) |
        ( 'SMeter' ) |
        ( 'Squelch' ) |
        ( 'Text' 'Decoder' 'Mode' ) |
        ( 'Transverter' 'Mode' )
    )

VfoActions ::=
    ( 'Set' 'Vfo' 'Link' )
    ( 'Set' ( 'VfoA' 'to' 'VfoB' ) | () 'VfoB' 'to' 'VfoA'  )
    ( 'Swap' 'VfoA 'and' 'VfoB' )
    (
        ( 'Move') ( 'VfoA' | 'VfoB' )
        ( 'Frequency' )
        ( 'Up | 'Down' )
    )

```

Even if the result of naming seems verbose, the advantages in consistency and
clarity are worh it. For example the direct translation of the CAT command
*VFO-A up* into `VfoAUp` is not as clear as the more correctly formed
`MoveVfoAFrequencyUp`.

### Documentation structure

All documentation should follow the format above.

1. A one-line description, which SHOULD start with either **Get** or **Set**.
2. *Optionally* any additionall information about the command as separate
   paragraphs of text.
3. A description of the command sent to the device:
   1. The heading "Command format", with
   2. a block quoted paragraph,
   3. the command string in backquotes,
   4. any literals as-is,
   5. any variable names between "{" and "}".
   6. *Optionally* a variable description block, if variables are present, as
      described below.
4. *Optionally* a description of the response sent from the device:
   1. The heading "Command format", with
   2. a block quoted paragraph,
   3. the command string in backquotes,
   4. any literals as-is,
   5. any variable names between "{" and "}".
   6. *Optionally* a variable description block, if variables are present, as
      described below.

Variables should be described in separate sections along the lines of these
examples. Note that variable names MUST always be rendered in italics and literal
values in monospace/code.

For enumerated values:

```text
Where *n* is one of:

* `0`; use the TX antenna.
* `1`; use the RX-only antenna.
```

For state variables:

```text
Where `n` is the boolean state `0` (off) or `1` (on).
```

For ranges:

```text
Where *nn* is the compression percentage between `00` (off) and `40`.

Where *nnnn* is the bandwidth value between `0000` and `9999` in Hz.

Where *nn* is meter value, between `00` and `21`, approximately 0 to 100% of the
full scale in 5% steps.
```

**IF** more than one variable exists **AND** all are simple values then a more
compact form may be used:

```text
> `AK{ii}{ccc}{lll}{aaa};`

Where:

* *ii* is the input switch.
* *ccc* is the capacitor.
* *lll* is the inductor.
* *aaa* is the antenna.
```

## Implementing Commands

The commands defined above need to implement the trait `Command` and optionally
`CommandWithResponse`. To do this the macros `impl_command!` and `impl_command_with_response!`
SHOULD be used. There are a number of cases where these macros are not flexible
enough to cover all cases and various levels of encapsulation can be accomplished
following an approximate 80/20 rule.

```rust
// ------------------------------------------------------------------------------------------------

impl_command!(GetRxAntenna => b"AR");
impl_command_with_response!(GetRxAntenna => boolean);

impl_command!(SetRxAntenna => b"AR" for boolean rx_only);
```

## Get/Set State

Use the `=> boolean` on the get command to return a boolean value directly as the
response, and use `for state` to add a boolean field named `on` to the set
command.

```rust
impl_command!(GetThingState => b"AR");
impl_command_with_response!(GetThingState => boolean);

impl_command!(SetThingState => b"AR" for state);
```

## Get/Set Boolean field

Use the `=> boolean` on the get command to return a boolean value directly as the
response, and use `for boolean some` to add a boolean field named `some` to the
set command.

```rust
impl_command!(GetThingSome => b"AR");
impl_command_with_response!(GetThingSome => boolean);

impl_command!(SetThingSome => b"AR" for boolean something);
```

## Get/Set command enum field

Use the `=> try_from enum ThingEnum` on the get command to return a `u8` value
directly as the response.  This works because `define_command_enum` marks the
type as having a `repr` of `u8` and also derives an implementation of the
trait `strum::FromRepr`.

Use `for as byte thing` on the set command to take the field named `thing` and
convert to the usual `&[u8]` argument value using a simple conversion
`thing as u8`.

```rust
impl_command!(GetThingEnum => b"AR");
impl_command_with_response!(GetThingEnum => try_from enum ThingEnum);

impl_command!(SetThingEnum => b"AR" for as byte thing);
```

## Get with custom parser

The `impl_command_with_response` can also take a function which will be provided
a *validated* byte buffer to parse. The numeric literal tells the validator how
many bytes of *actual* data the parser expects and the validator will strip off
the message header and terminator befor passing it to this function.

```rust
impl_command!(GetAtuNetworkValues => b"AK");
impl_command_with_response!(GetAtuNetworkValues => 11, |bytes| {
    Ok(AtuNetworkValues {
        input_switch: parse_decimal_u8(&bytes[0..2])?,
        capacitor: parse_decimal_u8(&bytes[2..5])?,
        inductor: parse_decimal_u8(&bytes[5..8])?,
        antenna: parse_decimal_u8(&bytes[8..11])?,
    })
} => AtuNetworkValues);
```

There are a number of existing parsers for common values which can be passed
directly as shown below.

```rust
impl_command!(GetBargraph => b"BG");
impl_command_with_response!(GetBargraph => 2, u8_from_ascii => u8);
```

## Set with custom argument bytes

The `impl_command` can also take a function to be used by the trait method
`argument_bytes`. Using the macro keyword `with` you add a function to return
an `Result<Option<Vec<u8>>, RigError>` as shown below.

```rust
impl_command!(SetVfoBDisplayText => b"DB" with |cmd: &SetVfoBDisplayText| {
    Ok(Some(cmd.text.to_vec()))
});
```

As this adds some *annoying* boilerplate for simple cases, include the additional
keyword `Some` to change the expected typpe to just `Vec<u8>` for infallible
functions.

```rust
impl_command!(SetVfoBDisplayText => b"DB" with Some |cmd: &SetVfoBDisplayText| {
    cmd.text.to_vec()
});
```

Again, there are some pre-defined *formatting* functions that are commonly used
in argument functions.

```rust
impl_command!(SetKeyerSpeed => b"KS" with Some |cmd: &SetKeyerSpeed| {
    format_u8_ascii_3(cmd.wpm)
});
```
