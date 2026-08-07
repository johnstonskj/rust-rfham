# Rig Control Protocols

The goal of this activity is to produce a new model for ham (amateur) radio
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

## Goal

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

## Approach

### Identification Codes

### Granularity Issues

### Example

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

## Output

Module structure:

```text
rfham-rigs::
├── // entity traits
├── // config traits
├── // concrete types
├── // errors
├── protocols::
│   ├── cat::
│   │   └── // protocol implementation and configuration
│   └── civ::
│   │   └── // protocol implementation and configuration
├── rigs::
│   ├── elecraft::
│   │   └── kx3()
│   ├── icom::
│   │   ├── ic705()
│   │   └── ic905()
│   └── // ...
├── actors::
│   └── spawn() // & runtime support
├── asyncs::
│   └── spawn() // & runtime support
└── connect::
    ├── bluetooth:: // connect(...), connect_profile(...), and connect_default()
    ├── usb::
    └── wifi::

```

The output should be a set of traits in the same format as above, but with detailed
comments that describe both the mapping to CAT and CI-V. The structure of these
have a fixed structure shown in the listing below. The tables breakdown the message
structure into fields with any necessary comments and for CIV-V a byte map shows
how messages are actually transmitted.

```rust
pub trait Vfo: Configurable<Config = Self::VfoConfig> {
    type VfoConfig: VfoConfig;

    ///
    /// Read the *operating frequency* of this VFO.
    /// 
    /// ## Errors
    /// 
    /// * `E_INDEX` -- the identifier of this VFO does not match the index of a VFO
    ///   on the target radio.
    ///
    /// ## Mapping to CAT (Elecraft KX3)
    ///
    /// | Chars   | Meaning   | Value(s)   | Comments                       |
    /// | ------- | --------- | ---------- | ------------------------------ |
    /// | `0`     | command   | `F`        |                                |
    /// | `1`     | command   | `A` or `B` | indicates which VFO            |
    /// | `2`     | trailer   | `;`        | fixed *end-of-message*.        |
    ///
    ///
    /// ## Mapping to CI-V
    ///
    /// | Bytes   | Meaning   | Value(s) | Comments                       |
    /// | ------- | --------- | -------- | ------------------------------ |
    /// | `0..=1` | header    | `FE FE`  | fixed *start-of-message.       |
    /// | `2`     | to-addr   |          | address of radio.              |
    /// | `3`     | from-addr | `E0`     | reserved *controller* address. |
    /// | `4`     | command   | `03`     | command number.                |
    /// | `5`     | trailer   | `FD`     | fixed *end-of-message*.        |
    ///
    /// ```text
    /// 0  1  2  3  4  5
    /// FE FE A4 E0 03 FD
    /// |==== |= |= |= |=
    /// |     |  |  |  '-- trailer
    /// |     |  |  '-- command
    /// |     |  '-- from address (controller)
    /// |     '-- to address (radio)
    /// '-- header
    /// ```
    ///
    fn frequency(&self) -> Result<Frequency, RigError>;

    ///
    /// Set the *operating frequency* of this VFO.
    /// 
    /// ## Errors
    /// 
    /// * `E_INDEX` -- the identifier of this VFO does not match the index of a VFO
    ///   on the target radio.
    /// * `E_LENGTH` -- the encoding of the frequency was not the length expected
    ///   by the target radio.
    ///
    /// ## Mapping to CAT (Elecraft KX3)
    ///
    /// | Chars   | Meaning   | Value(s)      | Comments                              |
    /// | ------- | --------- | ------------- | ------------------------------------- |
    /// | `0`     | command   | `F`           |                                       |
    /// | `1`     | command   | `A` or `B`    | indicates which VFO                   |
    /// | `2..=n` | frequency | `[0-9]{m,n}`  | the frequency **in Hz**, zero padded. |
    /// | `n+1`   | trailer   | `;`           | fixed *end-of-message*.               |
    ///
    /// ### Variants
    ///
    /// Different radios will have different min(`m`)/max(`n`) values, for example:
    ///
    /// * Elecraft KX3, Lab599: `m = n = 11`
    /// * Yaesu FTX-1: `m = n = 9`
    ///
    /// ## Mapping to CI-V
    /// 
    /// The operating frequency is encoded in BCD.
    ///
    /// | Bytes   | Meaning   | Value(s) | Comments                       |
    /// | ------- | --------- | -------- | ------------------------------ |
    /// | `0..=1` | header    | `FE FE`  | fixed *start-of-message.       |
    /// | `2`     | to-addr   |          | address of radio.              |
    /// | `3`     | from-addr | `E0`     | reserved *controller* address. |
    /// | `4`     | command   | `05`     | command number.                |
    /// | `5`     | trailer   | `FD`     | fixed *end-of-message*.        |
    ///
    /// ```text
    /// 0  1  2  3  4  5  6  7  8  9  A
    /// FE FE A4 E0 05 00 00 00 50 14 FD
    /// |==== |= |= |= |============= |=
    /// |     |  |  |  |              '-- trailer
    /// |     |  |  |  '-- frequency, in nybbles
    /// |     |  |  '-- command
    /// |     |  '-- from address (controller)
    /// |     '-- to address (radio)
    /// '-- header
    /// ```
    /// 
    /// ### Variants
    /// 
    /// In *[Icom Communications Interface-V Reference Manual, ver 3.2](./icom_civ_reference_v32_2002.pdf)*
    /// it notes that "For the IC-735, the 5th byte CANNOT be specified."
    /// 
    /// In *[CI-V Addressing](https://plicht.de/ci-v/civ-bus-adressing/)*
    /// it notes that "When the 5600 MHz or lower band is selected, the number
    /// of digits is 10 (1~5). When the 10 GHz band is selected, the number of
    /// digits is 12 (1~6)."
    ///
    fn set_frequency(&mut self, frequency: Frequency) -> Result<(), RigError>;
}
```

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

### Connection API

For each connection kind, which *may be* USB, Bluetooth, and Wi-Fi, or *may be*
Serial and Network, there will be a module.

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

## References

1. [Icom IC-705 CI-V Reference Guide](./icom_civ_reference_v32_2002.pdf), Icom Inc, Jun 2020.
2. [Icom Communications Interface-V Reference Manual, ver 3.2](./icom_civ_reference_v32_2002.pdf), Icom Inc, 2002.
3. [Elecraft K3S/K3/KX3 Programmer's Reference, Rev. F2](./K3S&K3&KX3%20Pgmrs%20Ref,%20F2.pdf), Elecraft, Jul 2015.
4. [What is CAT Control or what is Rig Control?](https://www.ham-interfaces.com/ham-radio-info-and-guides/what-is-cat-control), retrieved Jul 2026.
5. [CI-V Addressing](https://plicht.de/ci-v/civ-bus-adressing/), retrieved Jul 2026.
6. [Icom IC-905 CI-V Reference Guide](./IC-905_ENG_CI-V_2.pdf), Icom Inc, Jun 2020.
