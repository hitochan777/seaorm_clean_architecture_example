mod domain;
mod infra;

use anyhow::Result;
use domain::repository::Transaction;
use std::sync::Arc;

use futures::executor::block_on;
use infra::{seaorm_connection::SeaOrmConnection, repository::{SeaOrmPersonRepository, SeaOrmTransaction}};
use sea_orm::Database;

const DATABASE_URL: &str = "sqlite://sample.db?mode=rwc";

async fn save(transaction: Arc<SeaOrmTransaction>, first_name: String, last_name: String) -> Result<()> {
    transaction.execute(|repositories| {
        let person = repositories.person_repository.save(crate::domain::person::Person {
          id: 12,
          first_name,
          last_name,
        });
    }).await?;
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Arc::new(Database::connect(DATABASE_URL).await?);
    let conn = Arc::new(SeaOrmConnection::DbConn(db.clone()));
    let repository = Arc::new(SeaOrmPersonRepository {
        db: conn.clone(),
    });
    let transaction = Arc::new(SeaOrmTransaction {
        db: conn.clone(),
    });
    save(transaction.clone(), "hoge".to_owned(), "hhi".to_owned()).await?;
    Ok(())
}
