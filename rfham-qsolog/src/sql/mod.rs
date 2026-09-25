//!
//! Provides the SQL schema and database access layer for the RF Ham QSO Log application.
//!
//! More detailed description
//!
//! # Schema
//!
//! ## Table `version_info`
//! 
//! | Column           | Type | Unique | Null | Default             | Generated | Constraints                         |
//! |------------------|------|--------|------|---------------------|-----------|-------------------------------------|
//! | `created`        | TEXT |        | No   | `CURRENT_TIMESTAMP` |           | `length() >= 19 AND length() <= 30` |
//! | `schema_version` | TEXT |        | No   |                     |           | `REGEXP '...'`                      |
//! | `extensions`     | TEXT |        | Yes  |                     |           |                                     |
//!
//! The regular expression used for validating the `schema_version` as a semantic version ID column is:
//! 
//! ```text
//! ^[0-9]+(\.[0-9]+(\.[0-9]+)?)?(-[a-zA-Z][a-zA-Z0-9_+/]*)?$
//! ```
//! 
//! ## Table `logs`
//! 
//! | Column                        | Type    | Unique  | Null | Default             | Generated                  | Constraints                                       |
//! |-------------------------------|---------|---------|------|---------------------|----------------------------|---------------------------------------------------|
//! | `id`                          | INTEGER | **Yes** | No   |                     |                            | **`PRIMARY KEY`**                                 |
//! | `external_id`                 | TEXT    | Yes     | No   |                     | `gen_random_uuid() STORED` | `length() = 32`                                   |
//! | `created`                     | TEXT    |         | No   | `CURRENT_TIMESTAMP` |                            | `length() >= 19 AND length() <= 30`               |
//! | `label`                       | TEXT    |         | No   |                     |                            | `length() <= 40`                                  |
//! | `default_station_id`          | INTEGER |         | No   |                     |                            | `FOREIGN KEY () REFERENCES stations(id)`          |
//! | `default_station_location_id` | INTEGER |         | No   |                     |                            | `FOREIGN KEY () REFERENCES station_locations(id)` |
//! | `notes`                       | TEXT    |         | Yes  |                     |                            |                                                   |
//! 
//! | Index Name                | Unique | `id` | `external_id` | `label` | `owner` | `created` |
//! |---------------------------|--------|------|---------------|---------|---------|---------|
//! | `idx_logs_external_key`   | Yes    |      | Yes           |         |         |         |
//! 
//! ## Table `entries`
//! 
//! | Column                | Type    | Unique  | Null | Default             | Generated                  | Constraints                                       |
//! |-----------------------|---------|---------|------|---------------------|----------------------------|---------------------------------------------------|
//! | `id`                  | INTEGER | **Yes** | No   |                     |                            | **`PRIMARY KEY`**                                 |
//! | `in_log`              | INTEGER |         | No   |                     |                            | `FOREIGN KEY () REFERENCES logs(id)`              |
//! | `external_id`         | TEXT    | Yes     | No   |                     | `gen_random_uuid() STORED` | `length() = 32`                                   |
//! | `created`             | TEXT    |         | No   | `CURRENT_TIMESTAMP` |                            | `length() >= 19 AND length() <= 30`               |
//! | `started`             | TEXT    |         | Yes  |                     |                            | `length() >= 19 AND length() <= 30`               |
//! | `ended`               | TEXT    |         | Yes  |                     |                            | `length() >= 19 AND length() <= 30`               |
//! | `station_id`          | INTEGER |         | No   |                     |                            | `FOREIGN KEY () REFERENCES stations(id)`          |
//! | `station_location_id` | INTEGER |         | No   |                     |                            | `FOREIGN KEY () REFERENCES station_locations(id)` |
//! | `club_callsign`       | TEXT    |         | Yes  |                     |                            | `length() >= 2 AND length() <= 10`                |
//! | `frequency`           | INTEGER |         | No   |                     |                            |                                                   |
//! | `band`                | INTEGER |         | No   |                     |                            |                                                   |
//! | `mode`                | TEXT    |         | No   |                     |                            | `length() <= 10`                                  |
//! | `recv_signal_report`  | TEXT    |         | No   |                     |                            | `length() <= 8`                                   |
//! | `txmt_signal_report`  | TEXT    |         | No   |                     |                            | `length() <= 8`                                   |
//! | `notes`               | TEXT    |         | Yes  |                     |                            |                                                   |
//! 
//! | Index Name                    | Unique | `id` | `in_log` | `external_id` | `created` | `started` | `ended` | `station_id` | `station_location_id` | `club_callsign` | `frequency` | `band` | `mode` | `recv_signal_report` | `txmt_signal_report` | `notes` |
//! |-------------------------------|--------|------|----------|---------------|-----------|-----------|---------|--------------|-----------------------|-----------------|-------------|--------|--------|----------------------|----------------------|---------|
//! | `idx_entries_in_log`          | Yes    |      | Yes      |               |           |           |         |              |                       |                 |             |        |        |                      |                      |         |
//! | `idx_entries_external_key`    | Yes    |      |          | Yes           |           |           |         |              |                       |                 |             |        |        |                      |                      |         |
//! | `idx_entries_station`         | No     |      |          |               |           |           |         | Yes          |                       |                 |             |        |        |                      |                      |         |
//! | `idx_entries_location`        | No     |      |          |               |           |           |         |              | Yes                   |                 |             |        |        |                      |                      |         |
//! | `idx_entries_band`            | No     |      |          |               |           |           |         |              |                       |                 |             | Yes    |        |                      |                      |         |
//! | `idx_entries_mode`            | No     |      |          |               |           |           |         |              |                       |                 |             |        | Yes    |                      |                      |         |
//! 
//! ## Table `stations`
//! 
//! | Column                | Type    | Unique  | Null | Default             | Generated                  | Constraints                                       |
//! |-----------------------|---------|---------|------|---------------------|----------------------------|---------------------------------------------------|
//! | `id`                  | INTEGER | **Yes** | No   |                     |                            | **`PRIMARY KEY`**                                 |
//! | `created`             | TEXT    |         | No   | `CURRENT_TIMESTAMP` |                            | `length() >= 19 AND length() <= 30`               |
//! 
//! ## Table `station_locations`
//! 
//! | Column                | Type    | Unique  | Null | Default             | Generated                  | Constraints                                          |
//! |-----------------------|---------|---------|------|---------------------|----------------------------|------------------------------------------------------|
//! | `id`                  | INTEGER | **Yes** | No   |                     |                            | **`PRIMARY KEY`**                                    |
//! | `station_id`          | INTEGER |         | No   |                     |                            | `FOREIGN KEY () REFERENCES stations(id)`             |
//! | `created`             | TEXT    |         | No   | `CURRENT_TIMESTAMP` |                            | `length() >= 19 AND length() <= 30`                  |
//! | `kind`                | TEXT    |         | No   |                     |                            |                                                      |
//! | `locator_grid`        | TEXT    |         | No   |                     |                            | `REGEXP '...'  AND length() >= 2 AND length() <= 10` |
//! | `elevation`           | INTEGER |         | Yes  |                     |                            |                                                      |
//! | `street`              | TEXT    |         | Yes  |                     |                            |                                                      |
//! | `street_line_2`       | TEXT    |         | Yes  |                     |                            |                                                      |
//! | `city`                | TEXT    |         | Yes  |                     |                            |                                                      |
//! | `county_or_district`  | TEXT    |         | Yes  |                     |                            |                                                      | 
//! | `state_or_province`   | TEXT    |         | Yes  |                     |                            |                                                      |
//! | `postal_code`         | TEXT    |         | Yes  |                     |                            |                                                      |
//! | `country`             | TEXT    |         | Yes  |                     |                            |                                                      |
//! | `tz_offset`           | INTEGER |         | Yes  |                     |                            |                                                      |
//! | `notes`               | TEXT    |         | Yes  |                     |                            |                                                      |
//!
//! The regular expression used for validating the `locator_grid` as a Maidenhead locator column is:
//! 
//! ```text
//! ^[a-rA-R]{2}([0-9]{2}([a-xA-X]{2}([0-9]{2})?)?([a-xA-X]{2}([0-9]{2})?([a-xA-X]{2}([0-9]{2})?)?)?)?$
//! ```
//! 
//! | Index Name                        | Unique | `id` | `station_id` | `created` | `kind` | `locator_grid` | `elevation` | `street` | `street_line_2` | `city` | `county_or_district` | `state_or_province` | `postal_code` | `country` | `tz_offset` | `notes` |
//! |-----------------------------------|--------|------|--------------|-----------|--------|----------------|-------------|----------|-----------------|--------|----------------------|---------------------|---------------|-----------|-------------|---------|
//! | `idx_stations_grid`               | No     |      |              |           |        | Yes            |             |          |                 |        |                      |                     |               |           |             |         |
//! | `idx_stations_state_or_province`  | No     |      |              |           |        |                |             |          |                 |        |                      | Yes                 |               |           |             |         |
//! | `idx_stations_country`            | No     |      |              |           |        |                |             |          |                 |        |                      |                     |               | Yes       |             |         |
//! 

use crate::error::LogError;
use rfham_core::callsigns::CallSign;
use rusqlite::{Connection, OpenFlags, functions::FunctionFlags};
use uuid::{Uuid};
use std::{env, env::current_dir, path::PathBuf};
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Schema
// ------------------------------------------------------------------------------------------------

const DATABASE_DIR_NAME: &str = "RF Ham";
const DATABASE_FILE_NAME: &str = "rfham-qsolog.db";

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Log {
    id: i64,
    public_id: Uuid,
    label: String,
    owner: CallSign,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogEntry {
    id: Uuid,
    in_log: Uuid,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Database {
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// 
    /// Return the default path used to locate the database file.
    /// 
    /// Location priority:
    /// 
    /// 1. The path specified by the `RFHAM_QSOLOG_DB_PATH` environment variable.
    /// 2. The `XDG_DOCUMENTS_DIR` environment variable, if set, followed by `DATABASE_DIR_NAME`
    ///    and `DATABASE_FILE_NAME`.
    /// 3. The `HOME` environment variable, if set, followed by `Documents`, `DATABASE_DIR_NAME`
    ///    and `DATABASE_FILE_NAME`.
    /// 4. The current working directory, followed by `Documents`, `DATABASE_DIR_NAME` and
    ///    `DATABASE_FILE_NAME`.
    /// 
    pub fn default_path() -> PathBuf {
        env::var("RFHAM_QSOLOG_DB_PATH")
            .map(|s|PathBuf::from(s))
            .unwrap_or_else(|_| {
                env::var("XDG_DOCUMENTS_DIR")
                    .map(|s|PathBuf::from(s))
                    .unwrap_or_else(|_| {
                        env::var("HOME")
                            .map(|s|PathBuf::from(s))
                            .unwrap_or_else(|_| 
                                current_dir().expect("No current directory?")
                            )
                            .join("Documents")
                    })
                    .join(DATABASE_DIR_NAME)
                    .join(DATABASE_FILE_NAME)
            })
    }
    
    /// 
    /// Returns `true` if the database file exists.
    /// 
    pub fn exists() -> bool {
        Self::default_path().is_file()
    }
   
    /// 
    /// Creates the database file if it does not exist and returns a `Database` instance.
    /// 
    /// # Errors
    /// 
    /// Returns a `LogError` if the database file cannot be created or if there is an error
    /// defining the necessary SQLite functions or executing DDL statements.
    /// 
    pub fn create() -> Result<Self, LogError> {
        let conn = Connection::open_with_flags(
            Self::default_path(),
            OpenFlags::SQLITE_OPEN_READ_WRITE | 
            OpenFlags::SQLITE_OPEN_CREATE | 
            OpenFlags::SQLITE_OPEN_URI | 
            OpenFlags::SQLITE_OPEN_NO_MUTEX
        ).map_err(|e| LogError::SqlConnection(
            Self::default_path().clone(),
            e
        ))?;

        //
        // Creates the SQLite scalar function `generate_random_uuid` which returns a new UUID v7 as
        // a string.
        //
        conn.create_scalar_function(
            "gen_random_uuid",
            0,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            |_| {
                Ok(Uuid::now_v7().to_string())
            }
        ).map_err(|e| LogError::SqlDefinition(
            "SCALAR FUNCTION gen_random_uuid".to_string(), e
        ))?;

        // 
        // Creates the SQLite scalar function `regexp` which allows using regular expressions in
        // SQL queries and more importantly in check constraints.
        //
        // ```sql
        // CREATE TABLE example (
        //     version TEXT CHECK (version REGEXP '^[0-9]+(\\.[0-9]+(\\.[0-9]+)?)?$')
        // )
        // ```
        //
        conn.create_scalar_function(
            "regexp",
            2,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            |ctx| {
                let pattern: String = ctx.get::<String>(0)?;
                let column_value: String = ctx.get::<String>(1)?;
                trace!("sqlite function 'regexp' pattern: {pattern}, column_value: {column_value}");
                let re = regex::Regex::new(&pattern).map_err(|e| rusqlite::Error::UserFunctionError(Box::new(e)))?;
                Ok(re.is_match(&column_value))
            }
        ).map_err(|e| LogError::SqlDefinition(
            "SCALAR FUNCTION regexp".to_string(), e
        ))?;

        ddl!(
            conn, 
            create_table!(version_info (
                row!(timestamp created now),
                row!(semver schema_version not_null),
                row!(text extensions)
            ))
        );

        insert_into!(
            conn,
            version_info ( schema_version => "1.0" )
        );

        ddl!(
            conn, 
            create_table!(logs (
                row!(primary_key),
                row!(external_key),
                row!(timestamp created now),
                row!(text purpose),
                row!(label label not_null),
                row!(integer default_station_id not_null),
                row!(integer default_station_location_id not_null),
                row!(text notes),
                row!(foreign_key default_station_id => stations : id),
                row!(foreign_key default_station_location_id => station_locations : id)
            ))
        );

        ddl!(
            conn, 
            create_index!(unique idx_logs_external_key on logs (
                external_id
            ))
        );

        ddl!(
            conn, 
            create_table!(entries (
                row!(primary_key),
                row!(integer in_log not_null),
                row!(external_key),
                row!(timestamp created now),
                row!(timestamp started),
                row!(timestamp ended),
                row!(integer station_id not_null),
                row!(integer station_location_id not_null),
                row!(callsign club_callsign),
                row!(integer frequency not_null),
                row!(text mode not_null),
                row!(text recv_signal_report not_null),
                row!(text txmt_signal_report not_null),
                row!(text notes),
                row!(foreign_key in_log => logs : id),
                row!(foreign_key station_id => stations : id),
                row!(foreign_key station_location_id => station_locations : id)
            ))
        );

        ddl!(
            conn, 
            create_index!(unique idx_entries_in_log on entries (
                id, in_log
            ))
        );
        ddl!(
            conn, 
            create_index!(unique idx_entries_external_key on entries (
                external_id
            ))
        );
        ddl!(
            conn, 
            create_index!(unique idx_entries_station_id on entries (
                station_id
            ))
        );
        ddl!(
            conn, 
            create_index!(unique idx_entries_station_location_id on entries (
                station_location_id
            ))
        );
        ddl!(
            conn, 
            create_index!(unique idx_entries_band on entries (
                band
            ))
        );
        ddl!(
            conn, 
            create_index!(unique idx_entries_mode on entries (
                mode
            ))
        );

        ddl!(
            conn, 
            create_table!(stations (
                row!(primary_key),
                row!(timestamp created now),
                row!(callsign callsign not_null),
                row!(text name not_null),
                row!(text notes)
            ))
        );

        ddl!(
            conn, 
            create_index!(unique idx_stations_callsign on stations (
                callsign
            ))
        );

        ddl!(
            conn, 
            create_table!(station_locations (
                row!(primary_key),
                row!(integer station_id not_null),
                row!(timestamp created now),
                row!(text kind not_null),
                row!(grid locator_grid),
                row!(integer elevation),
                row!(text street),
                row!(text street_line_2),
                row!(text city),
                row!(text county_or_district),
                row!(text state_or_province),
                row!(text postal_code),
                row!(text country),
                row!(integer tz_offset),
                row!(text notes),
                row!(foreign_key station_id => stations : id)
            ))
        );

        ddl!(
            conn, 
            create_index!(idx_stations_grid on station_locations (
                locator_grid
            ))
        );
        ddl!(
            conn, 
            create_index!(idx_stations_state_or_province on station_locations (
                state_or_province
            ))
        );
        ddl!(
            conn, 
            create_index!(idx_stations_country on station_locations (
                country
            ))
        );

        Ok(Self { conn })
    }
 
    /// 
    /// Opens an existing database file and returns a `Database` instance.
    /// 
    /// # Errors
    /// 
    /// Returns a `LogError` if the database file cannot be opened.
    /// 
    pub fn open() -> Result<Self, LogError> {
        Ok(Self {
            conn: Connection::open_with_flags(
                Self::default_path(),
                OpenFlags::SQLITE_OPEN_READ_WRITE | 
                OpenFlags::SQLITE_OPEN_URI | 
                OpenFlags::SQLITE_OPEN_NO_MUTEX
            ).map_err(|e| LogError::SqlConnection(
                Self::default_path().clone(),
                e
            ))?
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------
