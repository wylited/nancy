-- Create the transactions table
CREATE TABLE transactions (
    id INTEGER NOT NULL PRIMARY KEY,
    cleared BOOLEAN NOT NULL DEFAULT FALSE,
    credit REAL NOT NULL,
    debit REAL NOT NULL,
    payee TEXT NOT NULL,
    payer TEXT NOT NULL,
    category TEXT NOT NULL,
    note TEXT,
    date TIMESTAMP NOT NULL,
    cleardate TIMESTAMP,
    account_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);
