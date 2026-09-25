use rfham_qsolog::sql::Database;
use colored::Colorize;

fn main() {
    if Database::exists() {
        println!("Database already exists at {}.", Database::default_path().display());
        let db = Database::open().expect("Failed to open database");

        let mut all_logs = db.connection()
            .prepare("SELECT * FROM logs")
            .expect("Failed to prepare query");
        let mut rows = all_logs
            .query([])
            .expect("Failed to execute query");
        
        while let Some(row) = rows.next().expect("Failed to fetch row") {
            let external_id: String = row.get("external_id").expect("Failed to get external_id");
            let created: String = row.get("created").expect("Failed to get created");
            let label: String = row.get("label").expect("Failed to get label");
            let default_station_id: i64 = row.get("default_station_id").unwrap_or_default();
            let default_station_location_id: i64 = row.get("default_station_location_id").unwrap_or_default();
            println!();
            println!(
                "Log: {}{}", 
                if let Some(purpose)= row.get::<&str, String>("purpose").ok() {
                    format!("{}:", purpose.bold())
                } else {
                    String::default()
                },
                label
            );
            println!("{}", format!("Created: {created} ({external_id})").dimmed());
            println!();
        
            let mut station_query = db.connection()
                .prepare("SELECT * FROM stations WHERE id = ?1")
                .expect("Failed to prepare query");
            let row = station_query
                .query_one(
                    [default_station_id],
                    |row| {
                        println!("Station callsign: {}", row.get::<&str, String>("callsign").unwrap_or_default());
                        println!("Operator Name: {}", row.get::<&str, String>("name").unwrap_or_default());
                        if let Some(notes) = row.get::<&str, String>("notes").ok() {
                            println!("Notes: {}", notes);
                        }
                        Ok(())
                    }
                )
                .expect("Failed to execute query"); 
        
            let mut location_query = db.connection()
                .prepare("SELECT * FROM station_locations WHERE id = ?1")
                .expect("Failed to prepare query");
            let row = location_query
                .query_one(
                    [default_station_location_id],
                    |row| {
                        println!("Location Kind: {}", row.get::<&str, String>("kind").unwrap_or_default());
                        if let Some(locator_grid) = row.get::<&str, String>("locator_grid").ok() {
                            println!("Locator Grid: {}", locator_grid);
                        }
                        if let Some(street) = row.get::<&str, String>("street").ok() {
                            println!("Street: {}", street);
                        }
                        if let Some(street_line_2) = row.get::<&str, String>("street_line_2").ok() {
                            println!("Street Line 2: {}", street_line_2);
                        }
                        if let Some(city) = row.get::<&str, String>("city").ok() {
                            println!("City: {}", city);
                        }
                        if let Some(county_or_district) = row.get::<&str, String>("county_or_district").ok() {
                            println!("County/District: {}", county_or_district);
                        }
                        if let Some(state_or_province) = row.get::<&str, String>("state_or_province").ok() {
                            println!("State/Province: {}", state_or_province);
                        }
                        if let Some(postal_code) = row.get::<&str, String>("postal_code").ok() {
                            println!("Postal Code: {}", postal_code);
                        }
                        if let Some(country) = row.get::<&str, String>("country").ok() {
                            println!("Country: {}", country);
                        }
                        if let Some(tz_offset) = row.get::<&str, i64>("tz_offset").ok() {
                            println!("TZ Offset: {} minutes", tz_offset);
                        }
                        if let Some(notes) = row.get::<&str, String>("notes").ok() {
                            println!("Notes: {}", notes);
                        }
                        println!();
                        Ok(())
                    }
                )
                .expect("Failed to execute query"); 

            let mut entries = db.connection()
                .prepare("SELECT * FROM entries WHERE in_log = ?1")
                .expect("Failed to prepare query");
            let mut entry_rows = entries
                .query(
                    [default_station_id])
                .expect("Failed to execute query"); 

            while let Some(entry_row) = entry_rows.next().expect("Failed to fetch entry row") {
                let entry_external_id: String = entry_row.get("external_id").expect("Failed to get entry external_id");
                let created: String = entry_row.get("created").expect("Failed to get created");
                let started: Option<String> = entry_row.get("started").expect("Failed to get started");
                let ended: Option<String> = entry_row.get("ended").expect("Failed to get ended");
                let frequency: Option<i64> = entry_row.get("frequency").expect("Failed to get frequency");
                let mode: Option<String> = entry_row.get("mode").expect("Failed to get mode");
                let rst_received: Option<String> = entry_row.get("recv_signal_report").expect("Failed to get recv_signal_report");
                let rst_sent: Option<String> = entry_row.get("txmt_signal_report").expect("Failed to get txmt_signal_report");
                println!(
                    "| {:<19} | {:<19} | {:<19} | {:<10} | {:<10} | {:<8} | {:<8} | {:<36} |", 
                    "Created".bold(), "Started".bold(), "Ended".bold(), "Frequency".bold(), "Mode".bold(), "▼RST".bold(), "▲RST".bold(), "ID".bold());
                println!(
                    "|-{:-<19}-|-{:-<19}-|-{:-<19}-|-{:-<10}-|-{:-<10}-|-{:-<8}-|-{:-<8}-|-{:-<36}-|", 
                    "", "", "", "", "", "", "", "");
                println!(
                    "| {:<19} | {:<19} | {:<19} | {:<10} | {:<10} | {:<8} | {:<8} | {:<36} |", 
                    created, started.unwrap_or_default(), ended.unwrap_or_default(), frequency.unwrap_or_default(), mode.unwrap_or_default(), rst_received.unwrap_or_default(), rst_sent.unwrap_or_default(), entry_external_id.dimmed());
            }
        }

    } else {
        println!("Creating database at {}...", Database::default_path().display());
        let db = Database::create().expect("Failed to create database");
               
        db.connection()
            .execute(
                "INSERT INTO stations (callsign, name) VALUES (?1, ?2)", 
                ( "K7SKJ", "Simon" )
            )
            .expect("Failed to insert default station");

        let station_id = db.connection().last_insert_rowid();
               
        db.connection()
            .execute(
                "INSERT INTO station_locations (station_id, kind, locator_grid, city, state_or_province, country, tz_offset) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
                ( station_id, "Home", "CN87", "Bellevue", "Washington", "USA", -480 )
            )
            .expect("Failed to insert default station");

        let location_id = db.connection().last_insert_rowid();
        println!("Inserted your default station location with ID {}", location_id);

        db.connection()
            .execute(
                "INSERT INTO logs (label, default_station_id, default_station_location_id) VALUES (?1, ?2, ?3)", 
                ( "My First Log", station_id, location_id )
            )
            .expect("Failed to insert first log");

        let log_id = db.connection().last_insert_rowid();
        println!("Inserted your first log with ID {}", log_id);

        db.connection()
            .execute(
                "INSERT INTO entries (in_log, station_id, station_location_id, frequency, mode, recv_signal_report, txmt_signal_report) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
                ( log_id, station_id, location_id, 52525, "FM", "59", "59" )
            )
            .expect("Failed to insert first log");
        println!("Inserted your first log entry!");
    }
}