//! Mirrors `rfham_rigs::protocol::cat::elecraft` — see `tests/protocol_cat.rs`.
//!
//! Each module below is gated by the same Cargo feature as its corresponding source module in
//! `src/protocol/cat/elecraft/`, so tests only compile (and only need to compile) when that
//! feature is enabled.

#[cfg(feature = "meta")]
mod test_meta;

#[cfg(feature = "k3-kx")]
mod test_k3_kx;

#[cfg(feature = "k4")]
mod test_k4;

#[cfg(feature = "kh1")]
mod test_kh1;

#[cfg(feature = "kat500")]
mod test_kat500;

#[cfg(feature = "kpa500")]
mod test_kpa500;

#[cfg(feature = "kpa1500")]
mod test_kpa1500;

#[cfg(feature = "kxpa100")]
mod test_kxpa100;

#[cfg(feature = "p3")]
mod test_p3;

#[cfg(feature = "px3")]
mod test_px3;
