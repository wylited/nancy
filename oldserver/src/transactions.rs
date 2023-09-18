use axum::{extract::State, response::IntoResponse, Json};
use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

use crate::Db;

#[derive(Queryable, Deserialize, Serialize)]
pub struct Transactions {
    cleared: bool,
    credit: f32,
    debit: f32,
    // date: String, //should work as long as its iso8601 uh something something.. time.rs work out later...
    payee: String,
    category: Vec<Category>,

    notes: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Category {
    name: String
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

pub async fn get_categories(State(db): State<Db>) -> impl IntoResponse {
    let categories: Vec<Category> = db.client.query(
        "SELECT Category {name}", &()
    ).await.unwrap_or_else( |e| {
        println!("Error: {}", e);
        Vec::new()
    });

    Json(categories)
}