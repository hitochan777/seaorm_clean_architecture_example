use crate::domain::repository::{Op, Repositories, Transaction};
use async_trait::async_trait;
use sea_orm::DbErr;
use std::sync::Arc;
use crate::infra::seaorm_connection::SeaOrmConnection;
use super::entities::person::Entity as Person;
use anyhow::Result;
use crate::domain;
use crate::domain::repository::PersonRepository;

pub struct SeaOrmPersonRepository<'a> {
    pub db: Arc<SeaOrmConnection<'a>>,
}

#[async_trait]
impl<'a> PersonRepository for SeaOrmPersonRepository<'a> {
    async fn fetch_one(&self, id: String) -> Result<Option<domain::person::Person>> {
        let person = Person::find_by_id(id)
            .one(self.db.as_ref())
            .await?;
        Ok(person.and_then(|val| Some(domain::person::Person::from(val))))
    }
    async fn save(&self, person: &domain::person::Person) -> Result<()> {
        let model = super::entities::person::ActiveModel::from(person.into());
        if let Some(_) = self.fetch_one(person.id.to_owned()).await? {
            Person::update(model).exec(self.db.as_ref()).await?;
        }
        else {
            Person::insert(model).exec(self.db.as_ref()).await?;
        }
        Ok(())
    }
}

pub struct SeaOrmTransaction<'a> {
    pub db: Arc<SeaOrmConnection<'a>>,
}

#[async_trait]
impl<'a> Transaction for SeaOrmTransaction<'_> {
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

