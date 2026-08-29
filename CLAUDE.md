# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

**rust-rfham** is a Cargo workspace of ham radio libraries and tools for Rust.
The project uses [mise](https://mise.jdx.dev/) for tool version management and
[bacon](https://dystroy.org/bacon/) for watch-based development.

## Commands

### Build & Check

```sh
cargo build --workspace
cargo check --all-targets
cargo clippy --all-targets
```

The workspace enforces strict lints in `Cargo.toml`:

* `unsafe_code = "forbid"`; no unsafe code permitted anywhere
* `exported_private_dependencies = "deny"`; all public API deps must be public
  deps
* `rustdoc::all = "warn"` and `missing_crate_level_docs = "warn"`; all public
  items need docs

### Testing

```sh
cargo nextest run --workspace
cargo nextest run -p rfham-core
cargo test --all-features
cargo test --no-default-features
```

### Coverage

```sh
cargo tarpaulin
cargo tarpaulin --config no_std
cargo tarpaulin --config all_features
```

Coverage configs are defined in `.tarpaulin.toml`.

### Rust Documentation

```sh
cargo doc --no-deps --open
mdbook build rust-rfham.github.io
mdbook serve rust-rfham.github.io
```

### Markdown Documentation

Any markdown documentation must pass the markdownlint tool, which is invoked
as follows.

```sh
markdownlint-cli2 "./*.md"
```

The only exceptions are wide tables which must be column aligned and so tend to
be longer than the 80 column limit and so may be wrapped in disable/enable
markers as follows. Additionally, any verbatim text which should retain it's
format but overflows the 80 column limit may disable the warning.

```md
<!-- markdownlint-disable MD013 -->
<!-- markdownlint-enable MD013 -->
```

Note also, bulleted lists must use asterisks, not hyphens as bullets, and should
use semi-colons as term/definition separators where appropriate.

```md
* one; thing one
* two; thing two
```

### Bacon (interactive watch loop)

```sh
bacon
```

## Workspace Architecture

### Crate Dependencies

<!-- markdownlint-disable MD013 -->
|            | antennas | bands | cli | config | core | geo | iri | itu | maidenhead | markdown | rfham | rigs | services |
|------------|:--------:|:-----:|:---:|:------:|:----:|:---:|:---:|:---:|:----------:|:--------:|:-----:|:----:|:--------:|
| antennas   | X        |       |     |        |      |     |     |     |            |          |       |      |          |
| bands      |          | X     |     |        | Y    |     |     | Y   |            | Y        |       |      |          |
| cli        | Y        | Y     | X   | Y      | Y    | Y   |     | Y   | Y          | Y        |       | Y    | Y        |
| config     |          | Y     |     | X      | Y    | Y   |     | Y   | Y          | Y        |       |      |          |
| core       |          |       |     |        | X    |     |     |     |            |          |       |      |          |
| geo        |          |       |     |        | Y    | X   |     |     |            |          |       |      |          |
| iri        |          |       |     |        |      |     | X   |     |            |          |       |      |          |
| itu        |          |       |     |        | Y    |     |     | X   |            | Y        |       |      |          |
| maidenhead |          |       |     |        | Y    | Y   |     |     | X          |          |       |      |          |
| markdown   |          |       |     |        |      |     |     |     |            | X        |       |      |          |
| rfham      | Y        | Y     |     | Y      | Y    | Y   |     | Y   | Y          | Y        | X     |      |          |
| rigs       |          |       |     | Y      | Y    |     | Y   | Y   |            |          |       | X    |          |
| services   |          |       |     | Y      | Y    | Y   |     |     | Y          |          |       |      | X        |
<!-- markdownlint-enable MD013 -->

### Core Type Traits (`rfham-core`)

Two foundational traits used throughout the workspace:

* **`StringLike`**; for newtype string wrappers, i.e. callsigns, names, country
  codes.
  * Requires `Clone + Display + PartialEq + Eq + Ord + Hash + FromStr +
    AsRef<str> + Into<String>` and a `MAX_LENGTH` const.
* **`Measure`**; for physical unit wrappers (frequency, power, wavelength).
  * Requires `Display + FromStr + TryFrom<f64> + Into<f64>`.
* **`BidirectionalMeasure`**; extends **`Measure`** for signed values, e.g.
  power that can be negative for reverse flow.

### Rig Control Layer (`rfham-rigs`)

The rig control crate is structured in layers:

* `protocol/`; wire-level protocol implementations
  * `cat/`; Computer-Aided Transceiver protocol (Kenwood dialect base, Elecraft
    and Yaesu extensions)
  * `civ/`; Icom CI-V protocol
* `transport/`; I/O abstraction over serial/IP connections with a `log`
  transport for testing
* `rigs/`; per-manufacturer rig definitions (Elecraft, Icom, Lab599)
* `actors/`; actor-model wrappers for async rig control
* `asyncs/`; async utilities
* `script/`; scripted command sequences
* `entities/`, `features/`, `replies/`; shared rig data model re-exported from
  crate root

The `init<R, C, S>()` function is the entry point for opening a rig connection;
it is currently stubbed.

### Configuration (`rfham-config`)

Config files are TOML. Connections support two variants serialized with
`#[serde(tag = "type")]`:

* `serial`; `SerialConnection` with path, baud_rate, and optional serial
  parameters
* `ip`; `IpConnection` with host/address and port

Connection strings can be parsed from `"<path>:<baud>"` (serial) or
`"<host>:<port>(;timeout=<secs>)?"` (IP).

## `no_std` Support

`rfham-core` supports `no_std` via a `std` feature flag (enabled by default).
The `alloc` crate is pulled in when `std` is disabled. Other crates in the
workspace that need `no_std` compatibility should follow the same pattern:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc as std;
```

## Static Data

Several crates embed data files that are read at compile time or runtime:

* `rfham-core/data/`; country code CSVs
* `rfham-itu/data/`; ITU prefix CSV, zone mapping JSON, zone names CSV
