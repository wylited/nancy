use edgedb_derive::Queryable;
use edgedb_tokio::Client;

pub struct Database {
    db: Client
}

#[derive(Queryable, Debug)]
struct Account {
    name: String,
    age: i32,
}

impl Database {
    pub async fn init() -> Result<Database, edgedb_tokio::Error> {
        let conn = edgedb_tokio::create_client().await?;
        Ok(
            Database {
                db: conn
            }
        )
    }

    pub async fn get_accounts(&self) -> Result<(), edgedb_tokio::Error>{
        let accounts = self.db.query::<Account, _>(
            "SELECT Account {name}",
            &(),
        ).await?;

        println!("Accounts: {:#?}", accounts);

        Ok(())
    }
}