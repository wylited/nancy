use crate::App;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Transactions {
    cleared: bool, // is the transaction cleared or not?

    // how much was credited/debited
    credit: f32,
    debit: f32,

    // if the payee or payer string starts with a _, that means the following string refers to a account ID
    payee: String,
    payer: String,

    category: Vec<String>, // category of payments

    cleardate: NaiveDate, // date transaction was cleared
    date: NaiveDate,      // date transaction was made

    note: Option<String>, // optional note
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Account {
    id: String,       // account id starting with a _ followed by the hash of the name
    name: String,     // name of the account
    category: String, // category of the account
}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    name: String,
    admin: bool,
    accounts: Vec<String>,
    password: String, // password hash
}

#[derive(Clone)]
pub struct DB {
    users: Vec<User>,
    accounts: Vec<Account>,
    transactions: Vec<Transactions>,
}

impl DB {
    pub fn new() -> Self {
        Self {
            users: vec![],
            accounts: vec![],
            transactions: vec![],
        }
    }

    pub fn init(app: App) -> Self {
        let mut db = Self::new();

        db
    }
}
