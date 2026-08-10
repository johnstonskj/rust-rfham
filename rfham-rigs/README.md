# Crate rfham-radios

Ham Radio libraries providing radio models and controls.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![crates.io](https://img.shields.io/crates/v/rfham-radios.svg)](https://crates.io/crates/rfham-radios)
[![docs.rs](https://docs.rs/rfham-radios/badge.svg)](https://docs.rs/rfham-radios)

Part of the [rfham project](https://rust-rfham.github.io).

The goal of this crate is to produce a new model for ham (amateur) radio
control that encompasses the capabilities of the two major protocols that
exist today, namely Computer Aided Transciver (CAT) and Communications
Interface-V (CI-V) \[2]. According to \[4]:

> Yaesu introduced their (C)omputer (A)ided (T)ransceiver protocol in the
> early 1980's. The generic name seems to have stuck despite the protocols
> and interfaces being very different between manufacturers and transceivers.
>
> Yaesu transceivers have a mix of physical CAT ports. Some are TTL level,
> some RS232 and more modern Yaesu radios have a USB port and the radio
> presents to > your PC as an external USB Comm port.  Ultimately, an
> interface (or cable) is required between the radio and the USB port on
> your PC.  
>
> Icom followed suit with the CI-V interface for their products. CI-V I hear
> you ask? Well this actually stands for (C)ommunications (I)nterface version 5.
> Icom took a different route where the Tx/Rx data is on a common wire and
> up to 4 transceivers can exist on the same CI-V bus. Icom radios and software
> use a CSMA/CD (Carrier Sense Multiple Access with Collision Detection) and
> will resend the data if they detect collisions. Again, the levels are nominally
> TTL and an interface is required to connect between the radio and your PCs USB
> port. To allow multiple transceivers to exist on the CI-V bus, Icom transceivers
> are allocated a CI-V address and software generally expects the transceiver to
> be set to its default address for communications to work.

## Architecture

```text
              ,-------,   ,-------, ,---,
              | Actor |   | Async | |   |
              '-------'   '-------' | C |
,---------------------------------, | o |
| Rig Entity+Feature API          | | n |
'---------------------------------' | f |
,---------------------------------, | i |
| Protocol                        | | g |
|   ,----------,   ,----------,   | | u |
|   |   CAT*   |   |   CI-V   |   | | r |
|   '----------'   '----------'   | | a |
'---------------------------------' | t |
,---------------------------------, | i |
| Transport                       | | o |
|   ,----------,   ,----------,   | | n |
|   |  Serial  |   |  TCP/IP  |   | |   |
|   '----------'   '----------'   | |   |
'---------------------------------' '---'
```

```text
rfham-rigs::
├── entities::
├── features::
├── replies::
├── errors::
├── protocols::
│   ├── cat::
│   └── civ::
├── rigs::
│   ├── elecraft::
│   │   └── kx3()
│   ├── icom::
│   │   ├── ic705()
│   │   └── ic905()
│   └── // ...
└── transports::
    ├── serial::
    └── ip::
```

## Rig *Entity+Feature* API

Instead of a new protocol the goal is to define a new high-level application
progrmmng interface (API) that can translate to either CAT or CI-V. Not only is
this in itself an exercise in abstraction but there are also, obviously,
differences in implementation of each protocol for any given radio based on it's

specific feature set. Therefore, to derive this new API we will focus on three
primary aspects, and two supporting ones.

1. **Entities**; An entity is some *compound thing* with structure, behavior and state.
   * For example, a *Rig* is an entity as it clearly has structure (VFOs, Filters,
     etc.), behavior, and state.
2. **Features**; A feature is a way of describing a sub-set of the structure of one or
   more entities; specifically, a feature may have behavior or state implemented in
   multiple entities.
   * For example, the HasVFO feature is implemented by the *Rig* entity, it allows
     the rig to switch between VFOs, exchange VFO settings, and to fetch a specific
     VFO entity; the VFO entity has it's own behavior such as *move-up* or *move-down*
     and state such as the current *operating frequency*.
3. **Functions**; A function is a behavior that provided by the remote radio, note
   that not all methods on entities are mapped to functions.
   * For example, the *get vfo number 1* method on the *Rig* entity probably only
     returns a descriptor used for functions such as *read operating frequency*.
4. **Configuration**; Some of the functions that are included in CAT/CI-V are for
   managing settings on the radio, not for direct operation. In this case we identify
   which entity is being configured and add a *configuration object* to it which has
   the accessors for those properties.
   * For example, the VFO behaviors *move-up* and *move-down* are controlled by the
     *move-frequency-step* configuration property.
5. **Conditions**; A condition is an expression that allows the identification of
   significant changes in an entity's state values.

Additionally, it is important to note that both protocols are serial, and message
based, assuming that you send a message and then wait/poll for a reply. While this
is, of course, a reasonable approach the proposed API should provide three *flavors*:

* **Messaging**; essentially the original message-based protocol. All methods return
  a unit type, response values are read by the client either by calling a *get next
  response* method, or reading from a queue.
  * All messages are modeled in an eenum/union type so that a single type can be
    returned by this method.
* **Threaded**; in this case entity functions actually provide response values and
  appear synchronous.
  * It is expected that a thread is started for each device connection and that
    thread communicates exclusively with the device.
  * Client messages are sent to this thread over a *request* queue along with the
    write port of a *response* queue.
  * Device responses are read by the thread and sent back to the client over the
    provided queue.
  * Return types **must** be the same as any message read by the *messaging* flavor.
* **Asynchronous**; for languages that support true asynchronous models, all methods
  on all entities become async functions that return promises.
  * Return types **must** be the same as any message read by the *messaging* flavor.

### Examples (Proposed)

The following is a fragment of hand-written API that demonstrates the expected style.
Details and discussion follow the listing.

```rust
pub trait Entity {}

pub trait Configurable: Entity {
    type Config;

    fn config(&self) -> &Self::Config;
    fn config_mut(&mut self) -> &mut Self::Config;
}

pub trait Rig: Entity + Configurable<Config = Self::RigConfig> {
    type RigConfig: RigConfig;

    fn has_af_gain_control(&self) -> bool;
    fn af_gain_control(&self) -> impl AfGainControl;

    fn af_gain(&self) -> Result<Level, RigError>;
    fn set_af_gain(&mut self, gain: Level) -> Result<(), RigError>;

    fn vfo_count(&self) -> Result<usize, RigError>;
    fn vfo(&self, vfo: usize) -> Result<usize, RigError>;
    fn current_vfo(&self) -> Result<usize, RigError>;
    fn set_current_vfo(&self, vfo: usize) -> Result<usize, RigError>;

    fn antenna_count(&self) -> Result<usize, RigError>;
    fn antenna(&self, antenna: usize) -> Result<(), RigError>;
    fn current_antenna(&self) -> Result<usize, RigError>;
    fn set_current_antenna(&self, antenna: usize) -> Result<(), RigError>;
}

pub trait RigConfig {
    fn brand_name(&self) -> &str;
    fn model_name(&self) -> &str;
    fn model_version(&self) -> Option<&str>;
    fn serial_number(&self) -> Option<&str>;
}

pub trait AfGainControl: Feature {
    fn level(&self) -> Result<()), RigError>;
    fn set_level(&mut self, level: Level) -> Result<()), RigError>;
}

pub trait Vfo: Configurable<Config = Self::VfoConfig> {
    type VfoConfig: VfoConfig;

    fn frequency(&self) -> Result<(), RigError>;
    fn set_frequency(&mut self, frequency: Frequency) -> Result<(), RigError>;
    fn move_frequency_up(&mut self) -> Result<(), RigError>;
    fn move_frequency_down(&mut self) -> Result<(), RigError>;
}

pub trait VfoConfig {
    fn move_increment(&self) -> Result<(), RigError>;
    fn set_move_increment(&self, increment: Frequency) -> Result<(), RigError>;
}
```

1. The trait `Entity` is effectively a marker only at this point.
2. The trait `Configurable` is used to denote an *entity* that has an associated
   configuration object, the type of which is `Config` and unconstrained.
   1. The two methods on this trait allow for the retrieval of both mutrable and
      immutable copies of the entity's configuration state.
3. The `Rig` trait is our first actual *entity*, ...

## API *Styles*

### Messaging-Based API

This is the primary API and consists of the entity and configuration traits as
well as the reply messages as enumerations. Note that these APIs *do not* use
low-level types and any enumeration values *will not* have any numeric
representation.

```rust
pub trait Rig: Entity + Configurable<Config = Self::RigConfig> {
    type RigConfig: RigConfig;

    fn af_gain(&self) -> Result<(), RigError>;
    fn set_af_gain(&mut self, gain: Level) -> Result<(), RigError>;
}

pub enum Reply {
    Rig(RigReply),
    // ...
}

pub enum RigReply {
    AfGain(Level),
    // ...
}

pub trait ReplySource {

    fn receive(&mut self) -> Result<Reply, RigError>;
    fn receive_or_timeout(&mut self, timeout: Duration) -> Result<Reply, RigError>;
}
```

To use the API we need tro bring together a connection object and a rig entity
as shown in the example below.

```rust
let connection = connect::usb_default();
let rig /* impl Rig + MessageSource */ = icom::ic705();
let (mut rig /* impl Rig */, mut replies /* impl MessageSource */) = sync(rig, connection);

proxy.af_gain();
if let Ok(Reply(Rig(AfGain(level)))) = replies.receive() {
    println!("current volume: {level}/255");
} else {
    panic!();
}
```

### Actor-like API

This will be a feature-gated capability (feature = "actors"). The only difference
using this API is that the `sync` function called above is replaced by a call to
`actors::spawn`.

```rust
let (mut proxy /* impl Rig */, rmut eplies /* impl MessageSource */) = actors::spawn(rig, connection);
```

### Async API

This will be a feature-gated capability (feature = "async") using the Tokio runtime.
Copies of the traits with async modifiers will be used, and a new `asyncs::spawn`
function used to create new instances.

```rust
pub trait AsyncRig: Entity + Configurable<Config = Self::AsyncRigConfig> {
    type RigConfig: AsyncRigConfig;

    async fn af_gain(&self) -> Result<Level, RigError>;
    async fn set_af_gain(&mut self, gain: Level) -> Result<(), RigError>;
}
```

## Errors

Note that *any* method may return the `E_CONN` error which will require the application
to reconnect to the target radio. For this reason this error *is not* listed in the
errors section of the method documentation.

| ID        | Error                     | Rig   | Function  | Argument  | Value | Comments                                                      |
| --------- | ------------------------- | ----- | --------- | --------- | ----- | ------------------------------------------------------------- |
| E_INVCONN | Invalid Connection        | Yes   | No        | No        | No    | The specified connection is invalid and cannot be opened.     |
| E_NOCONN  | No Connection             | Yes   | No        | No        | No    | The connection to the target radio has failed.                |
| E_UNSRIG  | Unsupported Rig           | Yes   | No        | No        | No    | *Rig* profile not found.                                      |
| E_UNSFUN  | Unsupported Function      | Yes   | Yes       | No        | No    | This function not supported by the current *entity*.          |
| E_UNSARG  | Unsupported Argument      | Yes   | Yes       | Yes       | No    | This argument not supported by the current *function*.        |
| E_UNKFUN  | Unknown Function          | Yes   | No        | No        | Yes   | The protocol function ID not recognized by the target radio.  |
| E_UNKARG  | Unknown Argument          | Yes   | No        | No        | Yes   | The protocol argument not recognized by the target radio.     |
| E_INDEX   | Index Out of Bounds       | Yes   | Yes       | Yes       | Yes   | An argument meant to be an index is out of bounds.            |
| E_LENGTH  | Invalid Length            | Yes   | Yes       | Maybe     | Yes   | A message, or field, is an incorrect length.                  |
| E_CHAR    | Invalid Character         | Yes   | Yes       | Yes       | Yes   | The character is not valid in this field.                     |
| E_UNKREP  | Unknown Reply             | Yes   | No        | No        | No    | The target radio returned a response we didn't understand.    |
| E_TIMEOUT | Receive Timeout           | Yes   | No        | No        | No    | The local receive operation timed out before reading a reply. |
