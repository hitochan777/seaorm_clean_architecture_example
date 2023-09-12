use std::sync::Arc;

use crate::domain::person::Person;
use anyhow::Result;
use futures_util::future::BoxFuture;

pub struct Repositories {
    pub person_repository: Arc<dyn PersonRepository + Send + Sync>,
}

#[async_trait]
pub trait PersotRepository {
    async fn fetch_many(&self, ids: Vec<String>) -> Result<Vec<Tweet>>;
    async fn save(&self, tweet: &Tweet) -> Result<()>;
}

pub type Op<'a> = Box<dyn 'a + Send + FnOnce(Repositories) -> BoxFuture<'a, Result<()>>>;
#[async_trait]
pub trait Transaction {
    async fn execute(&self, op: Op<'static>) -> Result<()>;
}
