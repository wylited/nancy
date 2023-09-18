pub mod accounts;
pub mod transactions;

use accounts::{get_accounts, add_account, get_accounttypes};
use anyhow::Result;
use axum::{routing::get, Router};
use transactions::{get_categories, get_transactions};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Db {
    pub client: edgedb_tokio::Client
}

#[tokio::main]

async fn main() -> Result<()> {
    let db = Db { client: edgedb_tokio::create_client().await? };

    let router = Router::new()
        .route("/api/accounts", get(get_accounts).post(add_account))
        .route("/api/accounttypes", get(get_accounttypes))
        .route("/api/categories", get(get_categories))
        .route("/api/transactions", get(get_transactions))
        .with_state(db);


    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(router.into_make_service())
        .await?;

    Ok(())
}