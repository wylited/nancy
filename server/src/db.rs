use edgedb_derive::Queryable;
use edgedb_tokio::Client;

#[derive(Debug)]
pub struct Database {
    pub client: Client
}

#[derive(Queryable, Debug)]
pub struct Account {
    name: String,
}

impl Database {
    pub async fn init() -> Result<Database, edgedb_tokio::Error> {
        let conn = edgedb_tokio::create_client().await?;
        Ok(
            Database {
                client: conn
            }
        )
    }

    pub async fn get_accounts(&self) -> Result<Vec<Account>, edgedb_tokio::Error>{
        let accounts = self.client.query::<Account, _>(
            "SELECT Account {name}",
            &(),
        ).await?;

        Ok(accounts)
    }
}