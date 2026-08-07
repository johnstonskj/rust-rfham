use rfham_config::connections::{Connection, SerialConnection};
use rfham_rigs::{
    protocol::cat::{
        CatWrapper, Vfo,
        common::{GetOperatingFrequency, GetTransceiverId},
        elecraft::{
            GetInstalledOptions, GetK2CommandMode, GetK3CommandMode, GetK3IconsAndStatus,
            GetOperatingMode, InstalledOptions,
        },
    },
    rigs::elecraft::kx3,
    transport::ActiveConnectionKind,
};
use std::{io::Error as IoError, process::ExitCode, str::FromStr};

fn main() -> Result<ExitCode, IoError> {
    let conn: Connection =
        SerialConnection::from_str("/dev/cu.usbserial-A10KMJZB:38400;stop-bits=Two")
            .unwrap()
            .into();
    let port = ActiveConnectionKind::try_from(&conn).unwrap();
    println!("Connection active ({conn:?})");

    let mut cat = CatWrapper::new(port, kx3::model_urn());

    match cat.send_and_receive(GetTransceiverId) {
        Ok(Some(id)) => println!("Transceiver ID: {id}"),
        Ok(None) => println!("GetTransceiverId command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match cat.send_and_receive(GetK2CommandMode) {
        Ok(Some(mode)) => println!("{mode}"),
        Ok(None) => println!("GetK2CommandMode command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match cat.send_and_receive(GetK3CommandMode) {
        Ok(Some(mode)) => println!("{mode}"),
        Ok(None) => println!("GetK3CommandMode command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match cat.send_and_receive(GetInstalledOptions) {
        Ok(Some(InstalledOptions::KX(options))) => {
            if options.has_kxat_atu {
                println!(
                    "Has optional {} ATU installed.",
                    if options.is_kx3 { "KXAT3" } else { "KXAT2" }
                );
            }
            if options.has_kxfl3_roofing_filter {
                println!("Has optional KXFL3 roofing filter installed.");
            }
            if options.has_kxbc3_realtime_clock {
                println!("Has optional KXBC3 NiMH battery-charger and real-time clock installed.");
            }
            if options.has_kx3m_transverter {
                println!("Has optional KX3-2M or KX3-4M transverter installed.");
            }
            if options.has_kx3m_transverter {
                println!("Has optional KXIO2 RTC I/O  installed.");
            }
            if options.has_kxio2_rtc_io {
                println!("Is connected to external KXPA100 power amplifier.");
            }
            if options.has_kxat100_external_atu {
                println!("Connected amplifier has optional KXAT100 ATU installed.")
            }
        }
        Ok(Some(other)) => eprintln!("Error: unexpected radio response: {other:?}"),
        Ok(None) => println!("GetInstalledOptions command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match cat.send_and_receive(GetK3IconsAndStatus) {
        Ok(Some(status)) => println!("{status:?}"),
        Ok(None) => println!("GetK3IconsAndStatus command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match cat.send_and_receive(GetOperatingFrequency { vfo: Vfo::A }) {
        Ok(Some(frequency)) => println!("VFO A: {frequency:#} Hz"),
        Ok(None) => println!("GetOperatingFrequency command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match cat.send_and_receive(GetOperatingMode { vfo: Vfo::A }) {
        Ok(Some(mode)) => println!("VFO A: {mode}"),
        Ok(None) => println!("GetOperatingMode command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match cat.send_and_receive(GetOperatingFrequency { vfo: Vfo::B }) {
        Ok(Some(frequency)) => println!("VFO B: {frequency:#} Hz"),
        Ok(None) => println!("GetOperatingFrequency command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match cat.send_and_receive(GetOperatingMode { vfo: Vfo::B }) {
        Ok(Some(mode)) => println!("VFO B: {mode}"),
        Ok(None) => println!("GetOperatingMode command timed out"),
        Err(e) => eprintln!("Error: {e}"),
    }

    println!("Completed.");
    Ok(ExitCode::SUCCESS)
}
