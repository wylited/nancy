# Explanation of the Gin http API for Nancy.

## Index (TBD)

---

## Specification

### Parameters and return values

- Endpoint: `http://127.0.0.1:6806`
- An interface with parameters is required, the parameter is a JSON string, placed in the body, and the header
  Content-Type is `application/json`
- Return value

  ```json
  {
    "code": 0,
    "msg": "",
    "data": {}
  }
  ```

  - `code`: non-zero for exceptions
  - `msg`: an empty string under normal circumstances, an error text will be returned under abnormal conditions
  - `data`: may be `{}`, `[]` or `NULL`, depending on the interface

### Authentication

> to be implemented

---

## Accounts

---

### List accounts

- `/api/account/lsAccounts`
- no parameters
- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "accounts": [
      {
        "_id": { "$oid": "x" },
        "name": "HSBC",
        "type": "checking",
        "balance": { "$numberDecimal": "175.20" }
      },
      {
        "_id": { "$oid": "x" },
        "name": "Credit Mastercard",
        "type": "card",
        "balance": { "$numberDecimal": "584.20" }
      }
    ]
  }
}
```

### Add account

- `/api/account/addAccount`
- parameters

```json
{
  "name": "HSBC",
  "type": "checking",
  "balance": 175.2
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "account": {
      "_id": { "$oid": "x" },
      "name": "HSBC",
      "type": "checking",
      "balance": { "$numberDecimal": "175.20" }
    }
  }
}
```

### Remove account

- `/api/account/rmAccount`
- parameters

```json
{
  "acc_id": "x"
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {}
}
```

### Change account

- `/api/account/changeAccount`
- parameters

```json
{
  "acc_id": "x",
  "name": "HSBC",
  "type": "checking",
  "balance": 175.2
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {}
}
```

### Get account type

- `/api/account/getAccountType`
- parameters

```json
{
  "acc_id": "x"
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "type": "checking"
  }
}
```

### Change account type

- `/api/account/changeAccountType`
- parameters

```json
{
  "acc_id": "x",
  "type": "checking"
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {}
}
```

### Get account balance

- `/api/account/getAccountBalance`
- parameters

```json
{
  "acc_id": "x"
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "balance": { "$numberDecimal": "175.20" }
  }
}
```

### Change account balance

- `/api/account/changeAccountBalance`
- parameters

```json
{
  "acc_id": "x",
  "balance": 175.2
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {}
}
```

### Get account name

- `/api/account/getAccountName`
- parameters

```json
{
  "acc_id": "x"
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "name": "HSBC"
  }
}
```

### Change account name

- `/api/account/changeAccountName`
- parameters

```json
{
  "acc_id": "x",
  "name": "HSBC"
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {}
}
```

---

## Transactions

> To implement, filtering based on price or time.

### List transactions

- `/api/transaction/lsTransactions`
- parameters

```json
{
  "acc_id": "x"
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "transactions": [
      {
        "_id": { "$oid": "x" },
        "date": { "$date": { "$numberLong": "1646755200000" } },
        "payee": "Pizza Hut",
        "credit": { "$numberDecimal": "502.20" },
        "debit": { "$numberDecimal": "0" },
        "notes": "2 pineapple pizzas.",
        "category": "food",
        "cleared": true
      }
    ]
  }
}
```

### New transaction

- `/api/transaction/newTransaction`
- parameters

```json
{
  "acc_id": "x",
  "date": "2022-01-01",
  "payee": "Pizza Hut",
  "credit": 502.2,
  "debit": 0,
  "notes": "2 pineapple pizzas.",
  "category": "food",
  "cleared": true
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "transaction": {
      "_id": { "$oid": "x" },
      "date": { "$date": { "$numberLong": "1646755200000" } },
      "payee": "Pizza Hut",
      "credit": { "$numberDecimal": "502.20" },
      "debit": { "$numberDecimal": "0" },
      "notes": "2 pineapple pizzas.",
      "category": "food",
      "cleared": true
    }
  }
}
```

### Remove transaction

- `/api/transaction/rmTransaction`
- parameters

```json
{
  "trans_id": "x"
}
```

- return value

```json
{
  "code": 0,
  "msg": "",
  "data": {}
}
```

### Update transaction

- `/api/transaction/updateTransaction`
- paramaters

```json
{
  "trans_id": "x"
  "acc_id": "x",
  "date": "2022-01-01",
  "payee": "Pizza Hut",
  "credit": 502.2,
  "debit": 0,
  "notes": "2 pineapple pizzas.",
  "category": "food",
  "cleared": true
}
