use std::sync::Arc;

use crate::domain::person::Person;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
#[async_trait]
pub trait PersonRepository {
    async fn fetch_one(&self, id: String) -> Result<Option<Person>>;
    async fn save(&self, tweet: &Person) -> Result<()>;
}

pub struct Repositories {
    pub person_repository: Arc<dyn PersonRepository + Send + Sync>,
}

pub type Op<'a> = Box<dyn 'a + Send + FnOnce(Repositories) -> BoxFuture<'a, Result<()>>>;
#[async_trait]
pub trait Transaction {
    async fn execute(&self, op: Op<'static>) -> Result<()>;
}
