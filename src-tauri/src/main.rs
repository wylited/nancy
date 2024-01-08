// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable)]
struct User {
    id: i32,
    name: String,
    admin: bool,
    password: String,
}

fn main() {
    let connection = SqliteConnection::establish("nancy.db")
        .unwrap_or_else(|_| panic!("Error connecting to db"));

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
