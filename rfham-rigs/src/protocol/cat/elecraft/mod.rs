//!
//! Provides commands for Elecraft products, covering the K and KX series transceivers, KPA and KXPA
//! amplifiers, and KP and KXP panadapters.
//!
//! # References
//!
//! 1. [Elecraft K3S/K3/KX3 Programmer's Reference, rev. F2](./K3S&K3&KX3%20Pgmrs%20Ref,%20F2.pdf), Jul 2015.
//! 2. [ElecraftK3S/K3/KX3/KX2 Programmer's Reference, rev. G4](https://ftp.elecraft.com/KX2/Manuals%20Downloads/K3S&K3&KX3&KX2%20Pgmrs%20Ref,%20G4.pdf), November 2018.
//! 3. [ElecraftK3S/K3/KX3/KX2 Programmer's Reference, rev. G5](https://ftp.elecraft.com/K3S/Manuals%20Downloads/K3S&K3&KX3&KX2%20Pgmrs%20Ref,%20G5.pdf), Feb 2019.
//! 4. [K4 Programmer's Reference, rev. C7](https://lutz-electronics.ch/pdf/ELECRAFT/K4_Programmers_Reference_rev.C7.pdf), 2022.
//! 5. [K4 Programmer's Reference, rev. D11](https://ftp.elecraft.com/K4/Manuals%20Downloads/K4%20Programmer's%20Reference,%20rev.%20D12.pdf), May 2026
//! 6. [Elecraft KIO2 Programmer's Reference](https://ftp.elecraft.com/K2/Manuals%20Downloads/KIO2%20Pgmrs%20Ref%20rev%20E.pdf), Feb 2004.
//!    * Complete programmer's command reference for RS-232 computer control of the K2 with the KIO2 or KPA100.
//! 7. [Elecraft KH1 Programmer's Reference, rev. B2](https://ftp.elecraft.com/KH1/Manuals%20Downloads/Elecraft%20KH1%20Programmer's%20Ref,%20rev%20B2.pdf), Jan 2026.
//! 8. [Elecraft P3 Programmer's Reference, rev. A7](https://ftp.elecraft.com/P3/Manuals%20Downloads/P3_Pgmrs_Ref_Rev_A7.pdf), Apr 2016.
//! 9. [Elecraft PX3 Programmer's Reference, rev. A6](https://ftp.elecraft.com/PX3/Manuals%20Downloads/PX3_Pgmrs_Ref_A6.pdf), Feb 2017.
//! 10. [Elecraft KAT500 Automatic Antenna Tuner Command Reference](https://ftp.elecraft.com/KAT500/Manuals%20Downloads/KAT500%20Automatic%20Antenna%20Tuner%20Serial%20Command%20Reference.pdf), Sep 2023.
//! 11. [Elecraft KPA500 Programmer's Reference, rev A2](https://ftp.elecraft.com/KPA/Manuals%20Downloads/KPA500%20Programmers%20Ref.pdf)., Jul 2011
//! 12. [Elecraft KPA1500 Programmer's Reference, rev 3.03](https://ftp.elecraft.com/KPA1500/Manuals%20Downloads/KPA1500ProgrammingReferenceV3.pdf), Jun 2026.
//! 13. [Elecraft KXPA100 Programmer's Reference](https://ftp.elecraft.com/KXPA/Manuals%20Downloads/KXPA100%20Amplifier%20Command%20Reference.pdf), Feb 2014.
//!

#[macro_use]
mod macros;

// ------------------------------------------------------------------------------------------------
// Transceiver Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod k3_kx;
pub mod k4;
pub mod kh1;
pub mod meta;

// ------------------------------------------------------------------------------------------------
// Amplifier Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod kpa1500;
pub mod kpa500;
pub mod kxpa100;

// ------------------------------------------------------------------------------------------------
// Panadapter Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod p3;
pub mod px3;

// ------------------------------------------------------------------------------------------------
// Tuner Sub-Modules
// ------------------------------------------------------------------------------------------------

pub mod kat500;
