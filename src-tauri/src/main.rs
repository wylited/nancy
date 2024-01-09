// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_sqlite_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to db"))
}

fn main() {
    let connection = establish_sqlite_connection();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
