# Index

- [Index](#index)
  - [Specification](#specifaction)
    - [Parameters and return values](#parameters-and-return-values)
    - [Authentication](#authentication)
  - [Accounts](#accounts)
    - [List accounts](#List-accounts)
    - [Get account](#Get-account)
    - [Add account](#Add-account)
    - [Update an account](#Update-account)
      - [Update account name](#Update-account-name)
      - [Update account type](#Update-account-type)
      - [Update account balance](#Update-account-balance)
      - [Update account type](#Update-account-type)
    - [Delete account](#Delete-account)
- [Transactions](#Transactions)
    - [List transactions](#List-transactions)
    - [Get transaction](#Get-transaction)
    - [Add transaction](#Add-transaction)
    - [Update transaction](#Update-transaction)
        - [Update transaction date](#Update-transaction-date)
        - [Update transaction amount](#Update-transaction-amount)
        - [Update transaction description](#Update-transaction-description)
        - [Update transaction account](#Update-transaction-account)
        - [Update transaction category](#Update-transaction-category)
        - [Update transaction type](#Update-transaction-type)
    - [Delete transaction](#Delete-transaction)

---

# Specification

## Parameters and return values

- Parameters

  - Request Type
    - GET
    - POST
    - DELETE
    - PATCH

- Return value
  ```json
  {
      "code": 0,
      "msg": "",
      "data"
  }
  ```
  - `code`: non-zero for exceptions.
  - `msg`: an empty string under normal circumstances, an error text will be returned under abnormal conditions.
  - `data`: may be {}, [] or NULL.

## Authentication

- TODO

# Accounts

## List accounts

- `/api/lsAccounts`
- Parameters

  - no parameters
  - request type: GET

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "accounts": [
        {
          "id": { "$oid": "x" },
          "name": "HSBC",
          "type": "checking",
          "balance": { "$numberDecimal": "175.20" }
        },
        {
          "id": { "$oid": "x" },
          "name": "Credit Mastercard",
          "type": "card",
          "balance": { "$numberDecimal": "584.20" }
        }
      ]
    }
  }
  ```

## Get account

- `/api/accounts/{name}`
- Parameters

  - request type: GET

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "account": {
        "id": { "$oid": "x" },
        "name": "HSBC",
        "type": "checking",
        "balance": { "$numberDecimal": "175.20" }
      }
    }
  }
  ```

## Add account

- `/api/accounts`
- Parameters

  ```json
  {
    "name": "HSBC",
    "type": "checking",
    "balance": { "$numberDecimal": "175.20" }
  }
  ```

  - request type: POST

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "account": {
        "id": { "$oid": "x" },
        "name": "HSBC",
        "type": "checking",
        "balance": { "$numberDecimal": "175.20" }
      }
    }
  }
  ```

## Update account

### Update account name

- CURRENTLY UNAVAILABLE

### Update account type

- `/api/accounts/{name}/type`
- Parameters

  ```json
  {
    "type": "checking"
  }
  ```

  - request type: PATCH

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "account": {
        "id": { "$oid": "x" },
        "name": "HSBC",
        "type": "checking",
        "balance": { "$numberDecimal": "175.20" }
      }
    }
  }
  ```

### Update account balance

- `/api/accounts/{name}/balance`
- Parameters

  ```json
  {
    "balance": { "$numberDecimal": "175.20" }
  }
  ```

  - request type: PATCH

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "account": {
        "id": { "$oid": "x" },
        "name": "HSBC",
        "type": "checking",
        "balance": { "$numberDecimal": "175.20" }
      }
    }
  }
  ```

## Delete account

- `/api/accounts/{name}`
- Parameters

  - request type: DELETE

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "account": {
        "id": { "$oid": "x" },
        "name": "HSBC",
        "type": "checking",
        "balance": { "$numberDecimal": "175.20" }
      }
    }
  }
  ```

# Transactions

## List transactions

- `/api/lsTransactions`
- Parameters

  - no parameters
  - request type: GET

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transactions": [
        {
          "_id": { "$oid": "63136a58d301518b82e6c72d" },
          "account": "HSBC",
          "date": { "$date": { "$numberLong": "1646755210000" } },
          "payee": "Pizza Hut",
          "credit": { "$numberDecimal": "420" },
          "debit": { "$numberDecimal": "0" },
          "notes": "2 pineapple pizzas.",
          "category": "food",
          "cleared": true
        },
        {
          "_id": { "$oid": "63136a58d301518b82e6c72e" },
          "account": "HSBC",
          "date": { "$date": { "$numberLong": "1646755220000" } },
          "payee": "Pizza Hut",
          "credit": { "$numberDecimal": "69" },
          "debit": { "$numberDecimal": "0" },
          "notes": "Lemon Tea",
          "category": "food",
          "cleared": true
        },
        {
          "_id": { "$oid": "63136a58d301518b82e6c72f" },
          "account": "HSBC",
          "date": { "$date": { "$numberLong": "1646755230000" } },
          "payee": "Microsoft",
          "credit": { "$numberDecimal": "0" },
          "debit": { "$numberDecimal": "52000" },
          "notes": "Monthly Income",
          "category": "income",
          "cleared": false
        }
      ]
    }
  }
  ```

## Get last X transactions

- `/api/lastTransaction/x`
- Parameters

  - no parameters
  - request type: GET

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transactions": [
        {
          "_id": { "$oid": "63136a58d301518b82e6c72f" },
          "account": "HSBC",
          "date": { "$date": { "$numberLong": "1646755230000" } },
          "payee": "Microsoft",
          "credit": { "$numberDecimal": "0" },
          "debit": { "$numberDecimal": "52000" },
          "notes": "Monthly Income",
          "category": "income",
          "cleared": false
        },
        {
          "_id": { "$oid": "63136a58d301518b82e6c72e" },
          "account": "HSBC",
          "date": { "$date": { "$numberLong": "1646755220000" } },
          "payee": "Pizza Hut",
          "credit": { "$numberDecimal": "69" },
          "debit": { "$numberDecimal": "0" },
          "notes": "Lemon Tea",
          "category": "food",
          "cleared": true
        }
      ]
    }
  }
  ```

## Get transaction

- `/api/transactions/{id}`
- Parameters

  - no parameters
  - request type: GET

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

## Add transaction

- `/api/transactions`
- Parameters

  ```json
  {
    "account": "HSBC",
    "date": { "$date": { "$numberLong": "1646755230000" } },
    "payee": "Microsoft",
    "credit": { "$numberDecimal": "0" },
    "debit": { "$numberDecimal": "52000" },
    "notes": "Monthly Income",
    "category": "income",
    "cleared": false
  }
  ```

  - request type: POST

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

## Update transaction
- `/api/transactions/{id}`
- Parameters

  ```json
  {
    "account": "HSBC",
    "date": { "$date": { "$numberLong": "1646755230000" } },
    "payee": "Microsoft",
    "credit": { "$numberDecimal": "0" },
    "debit": { "$numberDecimal": "52000" },
    "notes": "Monthly Income",
    "category": "income",
    "cleared": false
  }
  ```

  - request type: PUT

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

### Update transaction date
- `/api/transactions/{id}/date`
- Parameters

  ```json
  {
    "date": { "$date": { "$numberLong": "1646755230000" } }
  }
  ```

  - request type: PUT

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

### Update transaction payee
- `/api/transactions/{id}/payee`
- Parameters

  ```json
  {
    "payee": "Microsoft"
  }
  ```

  - request type: PUT

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

### Update transaction credit
- `/api/transactions/{id}/credit`
- Parameters

  ```json
  {
    "credit": { "$numberDecimal": "0" }
  }
  ```

  - request type: PUT

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

### Update transaction debit
- `/api/transactions/{id}/debit`
- Parameters

  ```json
  {
    "debit": { "$numberDecimal": "52000" }
  }
  ```

  - request type: PUT

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

### Update transaction notes
- `/api/transactions/{id}/notes`
- Parameters

  ```json
  {
    "notes": "Monthly Income"
  }
  ```

  - request type: PUT

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

### Update transaction category
- `/api/transactions/{id}/category`
- Parameters

  ```json
  {
    "category": "income"
  }
  ```

  - request type: PUT

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

### Update transaction cleared
- `/api/transactions/{id}/cleared`
- Parameters

  ```json
  {
    "cleared": false
  }
  ```

  - request type: PUT

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```

### Delete transaction
- `/api/transactions/{id}`
- Parameters

  - request type: DELETE

- Return value
  ```json
  {
    "code": 0,
    "msg": "",
    "data": {
      "transaction": {
        "_id": { "$oid": "63136a58d301518b82e6c72f" },
        "account": "HSBC",
        "date": { "$date": { "$numberLong": "1646755230000" } },
        "payee": "Microsoft",
        "credit": { "$numberDecimal": "0" },
        "debit": { "$numberDecimal": "52000" },
        "notes": "Monthly Income",
        "category": "income",
        "cleared": false
      }
    }
  }
  ```
  
