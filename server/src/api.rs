use axum::{response::IntoResponse, routing::get, Json};
use serde::Serialize;
use std::collections::HashMap;

use crate::Nancy;

#[derive(Serialize, Debug)]
pub struct Database {
    accounts: Vec<Account>,
    categories: Categories,
}

impl Database {
    pub fn init() -> Database {
        Database {
            accounts: Vec::new(),
            categories: Categories::init(),
        }
    }
}

#[derive(Serialize, Debug)]

pub struct Account {
    id: u64,
    name: String,
    accounttype: AccountType,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Debug)]

pub enum AccountType {
    Checking,
    Savings,
    Credit,
    Cash,
}

#[derive(Serialize, Debug)]

pub struct Transaction {
    id: u64,
    account: u64,
    date: String,
    payee: String,
    notes: String,
    category: u64,
    credit: f64,
    debit: f64,
    cleared: bool,
}

#[derive(Serialize, Debug)]

pub struct Categories {
    categories: HashMap<u64, String>,
}

impl Categories {
    pub fn init() -> Categories {
        Categories {
            categories: HashMap::from([
                (1, "Foodstuffs".to_string()),
                (2, "Travel".to_string()),
                (3, "Entertainment".to_string()),
                (4, "Bills".to_string()),
                (5, "Other".to_string()),
            ]),
        }
    }

    // Yippee
    pub fn add_category(&mut self, id: u64, name: String) {
        self.categories.insert(id, name);
    }

    // aww, bye
    pub fn remove_category(&mut self, id: u64) {
        self.categories.remove(&id);
    }

    // fasty fast fast
    pub fn get_category(&self, id: u64) -> Option<&String> {
        self.categories.get(&id)
    }

    // slow, try not to use this
    pub fn get_category_by_name(&self, name: &str) -> Option<&u64> {
        self.categories
            .iter()
            .find(|(_, v)| v == &name)
            .map(|(k, _)| k)
    }

    // return current categories hashmap
    pub fn get_categories(&self) -> HashMap<u64, String> {
        self.categories.clone()
    }
}

impl Nancy {
    pub async fn addroutes(&mut self) {
        let newrouter = self
            .router
            .clone()
            .route("/api/", get(Self::apiroot))
            .route("/api/categories", get(categories));

        // replace it and dont use the old one.. idk how to do this normally.
        let _x = std::mem::replace(&mut self.router, newrouter);
    }

    pub async fn apiroot() -> impl IntoResponse {
        Json("the cool nancy api")
    }
}

pub async fn categories() -> impl IntoResponse {
    Json(Categories::init())
}