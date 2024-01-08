use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: i32,
    pub cleared: bool, // is the transaction cleared or not?

    // how much was credited/debited
    pub credit: f32,
    pub debit: f32,

    // if the payee or payer string starts with a _, that means the following string refers to a account ID
    pub payee: String,
    pub payer: String,

    pub category: String, // categories of the payment

    pub note: String, // optional note
}
