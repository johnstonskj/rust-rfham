# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**rust-rfham** is a Cargo workspace of ham radio libraries and tools for Rust. The project uses [mise](https://mise.jdx.dev/) for tool version management and [bacon](https://dystroy.org/bacon/) for watch-based development.

## Commands

### Build & Check

```sh
cargo build --workspace
cargo check --all-targets         # what bacon runs by default
cargo clippy --all-targets
```

### Testing

```sh
cargo test --workspace            # all tests
cargo nextest run --workspace     # preferred runner (installed via mise)
cargo test -p rfham-core          # single crate
cargo nextest run -p rfham-core   # single crate via nextest
```

### Coverage

```sh
cargo tarpaulin                   # default config (HTML + XML output)
cargo tarpaulin --config no_std   # no_std coverage profile
cargo tarpaulin --config all_features
```

Coverage configs are defined in `.tarpaulin.toml`.

### Documentation

```sh
cargo doc --no-deps --open
mdbook build rust-rfham.github.io
mdbook serve rust-rfham.github.io  # live preview at localhost:3000
```

### Bacon (interactive watch loop)

```sh
bacon           # runs check-all by default; keybindings: c=clippy, t=test, d=doc, v=coverage
```

## Workspace Architecture

### Crate Dependency Hierarchy

```text
rfham-core          ← foundational types (no workspace deps)
rfham-markdown      ← markdown formatting helpers (no workspace deps)
rfham-iri           ← IRI/URI newtype (no workspace deps)
rfham-geo           ← geographic coordinates and grid locators
rfham-maidenhead    ← Maidenhead locator parsing (wraps rfham-geo)
rfham-itu           ← ITU prefix, zone, and band allocations
rfham-bands         ← country-specific band plans (US FCC, UK RSGB)
rfham-config        ← TOML configuration (connections, equipment, stations)
rfham-rigs          ← rig control layer (depends on rfham-config + rfham-core + rfham-iri)
rfham-services      ← external service integrations (space weather, etc.)
rfham-antennas      ← antenna models and calculations
rfham              ← prelude re-exporting the whole ecosystem
rfham-cli          ← binary CLI (depends on the whole stack)
```

### Core Type Traits (`rfham-core`)

Two foundational traits used throughout the workspace:

- **`StringLike`** – for newtype string wrappers (callsigns, names, country codes). Requires `Clone + Display + PartialEq + Eq + Ord + Hash + FromStr + AsRef<str> + Into<String>` and a `MAX_LENGTH` const.
- **`Measure`** – for physical unit wrappers (frequency, power, wavelength). Requires `Display + FromStr + TryFrom<f64> + Into<f64>`. `BidirectionalMeasure` extends this for signed values (e.g., power that can be negative for reverse flow).

### Rig Control Layer (`rfham-rigs`)

The rig control crate is structured in layers:

- `protocol/` — wire-level protocol implementations
  - `cat/` — Computer-Aided Transceiver protocol (Kenwood dialect base, Elecraft and Yaesu extensions)
  - `civ/` — Icom CI-V protocol
- `transport/` — I/O abstraction over serial/IP connections (with a `log` transport for testing)
- `rigs/` — per-manufacturer rig definitions (Elecraft, Icom, Lab599)
- `actors/` — actor-model wrappers for async rig control
- `asyncs/` — async utilities
- `script/` — scripted command sequences
- `entities/`, `features/`, `replies/` — shared rig data model (re-exported from crate root)

The `init<R, C, S>()` function is the entry point for opening a rig connection; it is currently stubbed.

### Configuration (`rfham-config`)

Config files are TOML. Connections support two variants serialized with `#[serde(tag = "type")]`:

- `serial` — `SerialConnection` with path, baud_rate, and optional serial parameters
- `ip` — `IpConnection` with host/address and port

Connection strings can be parsed from `"<path>:<baud>"` (serial) or `"<host>:<port>(;timeout=<secs>)?"` (IP).

## Lint Configuration

The workspace enforces strict lints in `Cargo.toml`:

- `unsafe_code = "forbid"` — no unsafe code permitted anywhere
- `exported_private_dependencies = "deny"` — all public API deps must be public deps
- `rustdoc::all = "warn"` and `missing_crate_level_docs = "warn"` — all public items need docs

## `no_std` Support

`rfham-core` supports `no_std` via a `std` feature flag (enabled by default). The `alloc` crate is pulled in when `std` is disabled. Other crates in the workspace that need `no_std` compatibility should follow the same pattern:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc as std;
```

## Static Data

Several crates embed data files that are read at compile time or runtime:

- `rfham-core/data/` — country code CSVs
- `rfham-itu/data/` — ITU prefix CSV, zone mapping JSON, zone names CSV
