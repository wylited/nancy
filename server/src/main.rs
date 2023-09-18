pub mod db;

use axum::{routing::get, Router};
use db::DB;
use std::fs;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub struct App {
    pub path: String,
}

impl App {
    pub fn init(path: Option<String>) -> App {
        let directory_path = match path {
            Some(p) => p,
            None => String::from("$HOME/nancy/"), // Provide a default path if None is passed
        };

        // Create the directory if it doesn't exist
        if let Err(err) = fs::create_dir_all(&directory_path) {
            println!("Error creating directory: {}", err);
        } else {
            println!("Directory created successfully!");
        }

        App {
            path: directory_path,
        }
    }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let db = DB::new();

    let router = Router::new()
        .route("/api", get(hello_world))
        .route("/api/accounts", get(hello_world))
        .route("/api/transactions", get(hello_world))
        .with_state(db);

    Ok(router.into())
}
