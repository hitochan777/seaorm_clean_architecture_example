mod domain;
mod infra;

use std::sync::Arc;

use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use infra::{entities::{prelude::*, *}, seaorm_connection::SeaOrmConnection, person_repository::SeaOrmPersonRepository, transaction::SeaOrmTransaction};

const DATABASE_URL: &str = "sqlite://sample.db?mode=rwc";

async fn save(transaction: Arc<SeaOrmTransaction>, first_name: String, last_name: String) -> Result<()> {
    transaction.execute(|repositories| {
        let person = repositories.person_repository.save()
    }).await?;
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
    if let Err(err) = block_on(run()) {
      panic!("{}", err);
    }
    Ok(())
}
