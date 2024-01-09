// @generated automatically by Diesel CLI.

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
        note -> Nullable<Text>,
        date -> Timestamp,
        cleardate -> Nullable<Timestamp>,
        account_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    user_accounts (user_id, account_id) {
        user_id -> Integer,
        account_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> users (user_id));
diesel::joinable!(user_accounts -> accounts (account_id));
diesel::joinable!(user_accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
    user_accounts,
    users,
);
