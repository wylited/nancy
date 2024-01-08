// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        admin -> Bool,
        password -> Text,
    }
}

diesel::table! {
    accounts (id) {
        id -> Integer,
        name -> Text,
        category -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        cleared -> Bool,
        credit -> Float,
        debit -> Float,
        payee -> Text,
        payer -> Text,
        category -> Text,
        note -> Text,
    }
}
