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
