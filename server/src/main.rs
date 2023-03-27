use anyhow::{Result};
use axum::{routing::get, Router, response::IntoResponse, Json, extract::State};
use edgedb_derive::Queryable;
use edgedb_protocol::{model::{Uuid, Datetime}};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Db {
    pub client: edgedb_tokio::Client
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Account {
    balance: f32,
    name: String,

    // id: Uuid,
    // account_types: Vec<AccountType>,
    // transactions: Vec<Transaction>,
}

#[derive(Queryable)]
// how do i make serde deserialize and serialze work on this?
pub struct Transaction {
    id: Uuid,
    credit: f32,
    debit: f32,
    notes: String,
    payee: String,
    cleared: bool,
    date: Datetime,
    description: String,
    // categorys: Vec<Category>,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct AccountType {
    name: String
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Category {
    name: String
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
        "SELECT Account {name, balance}",
        &(),
    ).await.unwrap_or_else(|e| {
        println!("Error: {}", e);
        Vec::new()
    });
    
    Json(accounts)
}