use std::collections::HashMap;
use axum::{Json, extract::{State, Query}, response::IntoResponse};
use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;
use serde::{Deserialize, Serialize};

use crate::Db;

type Values = HashMap<String, String>;

#[derive(Queryable, Deserialize, Serialize)]
pub struct Account {
    balance: f32,
    name: String,
    accounttype: Vec<AccountType>,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct AccountType {
    name: String
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

pub async fn add_account(State(db): State<Db>, Query(params): Query<Values>) -> impl IntoResponse {
    let name = params.get("name").unwrap_or(&String::from("")).to_string();
    let balance = params.get("balance").unwrap_or(&String::from("")).to_string().parse::<f32>().unwrap_or(0.0);
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

pub async fn update_account(State(db): State<Db>, Query(params): Query<Values>) -> impl IntoResponse {
    let name = params.get("name").unwrap_or(&String::from("")).to_string();
    let balance: f32 = params.get("balance").unwrap_or(&String::from("")).to_string().parse::<f32>().unwrap_or(0.0);
    println!("name: {}, balance: {}", name, balance);

    let result = db.client.query_json(
        "UPDATE Account FILTER .name = <std::str>$0 SET { balance := <std::float32>$1 };",
        &(name, balance),
    ).await;

    if let Err(e) = result {
        println!("Error: {}", e);
        return Json("Error")
    }

    Json("Success")
}

pub async fn get_account_id(State(db): State<Db>, Query(params): Query<Values>) -> impl IntoResponse {
    let name = params.get("name").unwrap_or(&String::from("")).to_string();

    let account_id: Vec<Uuid> = db.client.query( 
        "SELECT Account.id FILTER Account.name = <std::str>'HSBC';",
        &()
    ).await.unwrap_or_else(|e| {
        println!("Error: {}", e);
        Vec::new()
    });

    Json(account_id.iter().map(|x| x.to_string()).collect::<Vec<String>>()) // Return the uuid as a string.
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