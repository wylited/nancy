pub mod db;

use anyhow::{Result};
use axum::{routing::get, Router, response::IntoResponse, Json, extract::State};
use edgedb_derive::Queryable;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Db {
    pub client: edgedb_tokio::Client
}

#[derive(Queryable)]
pub struct Account {
    name: String,
}

#[tokio::main]

async fn main() -> Result<()> {
    let db = Db { client: edgedb_tokio::create_client().await? };

    let router = Router::new()
        .route("/api", get(root))
        .route("/api/accounts", get(get_accounts))
        .with_state(db);


    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

pub async fn root() -> impl IntoResponse {
    Json("the cool nancy api")
}

pub async fn get_accounts(State(db): State<Db>) -> impl IntoResponse {
    let accounts = db.client.query::<Account, _>(
        "SELECT Account {name}",
        &(),
    ).await.unwrap_or_else(|e| {
        println!("Error: {}", e);
        Vec::new()
    });
    
    Json(accounts.iter().map(|a| a.name.clone()).collect::<Vec<String>>())
}