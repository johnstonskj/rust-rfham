//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!

use crate::error::LogError;
use rfham_core::callsigns::CallSign;
use rusqlite::{Connection, OpenFlags, functions::FunctionFlags};
use uuid::{Uuid};
use std::{env, env::current_dir, path::PathBuf};

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
    
    pub fn exists() -> bool {
        Self::default_path().is_file()
    }
   
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
            "generate_random_uuid",
            0,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            |_| {
                Ok(Uuid::now_v7().to_string())
            }
        ).map_err(|e| LogError::SqlDefinition(
            "SCALAR FUNCTION generate_random_uuid".to_string(), e
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
                println!("sqlite function 'regexp' pattern: {pattern}, column_value: {column_value}");
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
                row!(label label not_null),
                row!(callsign owner not_null),
                row!(timestamp created now)
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
                row!(foreign_key in_log => logs : id)
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

        Ok(Self { conn })
    }
 
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
