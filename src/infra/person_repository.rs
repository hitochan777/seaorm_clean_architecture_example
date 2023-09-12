use anyhow::Result;
use std::sync::Arc;

use super::entities::person::Entity as Person;
use super::seaorm_connection::SeaOrmConnection;
use crate::domain;

pub struct SeaOrmPersonRepository<'a> {
    pub db: Arc<SeaOrmConnection<'a>>,
}

#[async_trait]
impl<'a> PersonRepository for SeaOrmPersonRepository<'a> {
    async fn fetch_one(&self, id: String) -> Result<Option<domain::person::Person>> {
        let person = Person::find_by_id(id)
            .one(self.db.as_ref())
            .await?;
        Ok(person.and_then(|val| Some(domain::person::Person::from(&val))))
    }
    async fn save(&self, person: &Person) -> Result<()> {
        let model = super::entities::person::ActiveModel::from(person);
        if let Some(_) = self.fetch_one(person.id.to_owned()).await? {
            Person::update(model).exec(self.db.as_ref()).await?;
        }
        else {
            Person::insert(model).exec(self.db.as_ref()).await?;
        }
        Ok(())
    }
}

pub struct SeaOrmPersonRepository<'a> {
    pub db: Arc<SeaOrmConnection<'a>>,
}
