use crate::domain::repository::{Op, Repositories, Transaction};
use crate::Result;
use sea_orm::{DatabaseConnection, TransactionTrait, DbErr};
use std::sync::Arc;
use crate::infra::seaorm_connection::SeaOrmConnection;

use super::person_repository::SeaOrmPersonRepository;

pub struct SeaOrmTransaction {
    pub db: Arc<DatabaseConnection>,
}

#[async_trait]
impl Transaction for SeaOrmTransaction {
    async fn execute(&self, op: Op<'static>) -> Result<()> {
        self.db.transaction::<_, (), DbErr>(|tx| {
            let c = async move {
                let db = Arc::new(SeaOrmConnection::Transaction(Arc::new(tx)));
                let person_repository = Arc::new(SeaOrmPersonRepository {
                    db: db.clone(),
                });
                let res = op(Repositories {
                    person_repository,
                }).await;
                if let Err(_) = res {
                    return Err(DbErr::Custom(String::from("error")));
                }
                Ok(())
            };
            Box::pin(c)
        })
        .await?;
        Ok(())
    }
}

