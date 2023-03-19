pub mod api;
pub mod db;

use anyhow::{Result};
use axum::{routing::get, Router};
use std::net::SocketAddr;

// The struct for our server
#[derive(Debug)]
pub struct Nancy {
    router: Router,
    address: SocketAddr,
    // db: Database,
}

// Base implementations
impl Nancy {
    pub fn init() -> Nancy {
        Nancy {
            router: Router::new().route("/", get(|| async { "Hello, World!" })),
            address: SocketAddr::from(([127, 0, 0, 1], 3000)),
            // db: Database::init(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let nancy = Nancy::init();
    let db = db::Database::init().await?;

    db.get_accounts().await?;

    println!("Nancy is listening on {}", nancy.address);
    axum::Server::bind(&nancy.address)
        .serve(nancy.router.into_make_service())
        .await?;

    Ok(())
}