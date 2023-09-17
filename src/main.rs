mod domain;
mod infra;

use anyhow::Result;
use domain::repository::{Repositories, Transaction};
use futures_util::FutureExt;
use std::sync::Arc;

use infra::{
    repository::{SeaOrmPersonRepository, SeaOrmTransaction},
    seaorm_connection::SeaOrmConnection,
};
use sea_orm::Database;

const DATABASE_URL: &str = "sqlite://sample.db?mode=rwc";

async fn save(
    transaction: Arc<dyn Transaction>,
    first_name: String,
    last_name: String,
) -> Result<()> {
    transaction.execute(Box::new(|repositories| {
        async move {
            let person = repositories
                .person_repository
                .save(crate::domain::person::Person {
                    id: 12,
                    first_name,
                    last_name,
                });
            println!("saved");
            Ok(())
        }.boxed()
    })).await?;
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Arc::new(Database::connect(DATABASE_URL).await?);
    let conn = Arc::new(SeaOrmConnection::DbConn(db.clone()));
    let repository = Arc::new(SeaOrmPersonRepository { db: conn.clone() });
    let transaction = Arc::new(SeaOrmTransaction { db: db.clone() });
    save(transaction.clone(), "foo".to_owned(), "bar".to_owned()).await?;
    Ok(())
}
