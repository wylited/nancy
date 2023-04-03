use anyhow::{Result};
use axum::{routing::get, Router, response::IntoResponse, Json, extract::{State, Query}};
use edgedb_derive::Queryable;
use serde::{Serialize, Deserialize};
use std::{net::SocketAddr, collections::HashMap};

#[derive(Clone)]
pub struct Db {
    pub client: edgedb_tokio::Client
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Account {
    balance: f32,
    name: String,
    accounttype: Vec<AccountType>,
    // transactions: Vec<Transactions>,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct AccountType {
    name: String
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Transactions {
    cleared: bool,
    credit: f32,
    debit: f32,
    // date: String, //should work as long as its iso8601
    payee: String,
    category: Vec<Category>,

    notes: Option<String>,
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

pub async fn root() -> impl IntoResponse {
    Json("the cool nancy api")
}

pub async fn get_accounts(State(db): State<Db>) -> impl IntoResponse {
    let accounts: Vec<Account> = db.client.query(
        "SELECT Account {balance, name, accounttype:{name}};",
        &(),
    ).await.unwrap_or_else(|e| {
        println!("Error: {}", e);
        Vec::new()
    });
    
    Json(accounts)
}

pub async fn get_accounttypes(State(db): State<Db>) -> impl IntoResponse {
    let account_types: Vec<AccountType> = db.client.query(
        "SELECT AccountType.name", &()
    ).await.unwrap_or_else( |e| {
        println!("Error: {}", e);
        Vec::new()
    });

    Json(account_types)
}

pub async fn get_categories(State(db): State<Db>) -> impl IntoResponse {
    let categories: Vec<Category> = db.client.query(
        "SELECT Category {name}", &()
    ).await.unwrap_or_else( |e| {
        println!("Error: {}", e);
        Vec::new()
    });

    Json(categories)
}

pub async fn get_transactions(State(db): State<Db>) -> impl IntoResponse {
    let transactions: Vec<Transactions> = db.client.query(
        "SELECT Transactions {cleared, credit, debit, payee, category:{name}, notes}", &()
    ).await.unwrap_or_else( |e| {
        println!("Error: {}", e);
        Vec::new()
    });

    Json(transactions)
}

pub async fn add_account(State(db): State<Db>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let name = params.get("name").unwrap_or(&String::from("")).to_string();
    let balance: f32 = params.get("balance").unwrap_or(&String::from("")).to_string().parse::<f32>().unwrap_or(0.0);
    println!("name: {}, balance: {}", name, balance);

    let result = db.client.query_json(
        "INSERT Account { balance := <std::float32>$0, name := <std::str>$1 };",
        &(balance, name),
    ).await;

    if let Err(e) = result {
        println!("Error: {}", e);
        return Json("Error")
    }

    Json("Success")
}