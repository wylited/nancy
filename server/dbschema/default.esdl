module default {
    type Account {
        required property name         -> str;
        required property balance      -> float32
        
        multi link accounttype -> AccountType;
        multi link transactions -> Transactions;
    }

    type AccountType {
        required property name -> str;
    }

    type Transactions {
        required property cleared  -> bool;
        required property credit   -> float32;
        required property debit    -> float32;
        required property date     -> datetime;
        required property payee    -> str;
        multi link category -> Category;

        property notes -> str
    }

    type Category {
        required property name -> str;
    }
}
