use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub cleared: bool,

    pub credit: f32,
    pub debit: f32,

    pub payee: String,
    pub payer: String,

    pub date: NaiveDateTime,
    pub cleardate: Option<NaiveDateTime>,

    pub category: String,

    pub note: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub category: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Associations)]
#[belongs_to(User)]
#[belongs_to(Account)]
#[diesel(table_name = crate::schema::user_accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserAccount {
    pub user_id: i32,
    pub account_id: i32,
}
