CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE user_accounts (
    user_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (account_id) REFERENCES accounts (id),
    PRIMARY KEY (user_id, account_id)
);
