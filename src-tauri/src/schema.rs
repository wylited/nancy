// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Integer,
        cleared -> Bool,
        credit -> Float,
        debit -> Float,
        payee -> Text,
        payer -> Text,
        category -> Text,
        note -> Nullable<Text>,
        date -> Timestamp,
        cleardate -> Nullable<Timestamp>,
        account_id -> Integer,
        user_id -> Integer,
    }
}
