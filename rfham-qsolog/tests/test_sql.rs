use rfham_qsolog::sql::Database;
use std::env;

#[test]
fn test_database_default_path() {
    let tmp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    unsafe {
        env::set_var("RFHAM_QSOLOG_DB_PATH", tmp_dir.path().join("qsolog-test.db"));
    }

    assert_eq!(tmp_dir.path().join("qsolog-test.db"),  Database::default_path());
}

#[test]
fn test_database_not_exists() {
    let tmp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    unsafe {
        env::set_var("RFHAM_QSOLOG_DB_PATH", tmp_dir.path().join("qsolog-test.db"));
    }

    assert_eq!(false,  Database::exists());
}

#[test]
fn test_database_create() {
    let tmp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    unsafe {
        env::set_var("RFHAM_QSOLOG_DB_PATH", tmp_dir.path().join("qsolog-test.db"));
    }

    assert_eq!(false,  Database::exists());
    Database::create().expect("Failed to create database");
    assert_eq!(true,  Database::exists());
}
