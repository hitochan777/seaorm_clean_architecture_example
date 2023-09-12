use std::sync::Arc;
use sea_orm::*;

pub enum SeaOrmConnection<'a> {
    DbConn(Arc<DbConn>),
    Transaction(Arc<&'a DatabaseTransaction>),
}

#[async_trait]
impl<'a> ConnectionTrait for SeaOrmConnection<'a> {
    fn get_database_backend(&self) -> DbBackend {
        match self {
            SeaOrmConnection::DbConn(dbconn) => dbconn.get_database_backend(),
            SeaOrmConnection::Transaction(tx) => tx.get_database_backend(),
        }
    }

    async fn execute(&self, stmt: Statement) -> std::result::Result<ExecResult, DbErr> {
        match self {
            SeaOrmConnection::DbConn(dbconn) => dbconn.execute(stmt).await,
            SeaOrmConnection::Transaction(tx) => tx.execute(stmt).await,
        }
    }

    async fn query_one(&self, stmt: Statement) -> std::result::Result<Option<QueryResult>, DbErr> {
        match self {
            SeaOrmConnection::DbConn(dbconn) => dbconn.query_one(stmt).await,
            SeaOrmConnection::Transaction(tx) => tx.query_one(stmt).await,
        }
    }

    async fn query_all(&self, stmt: Statement) -> std::result::Result<Vec<QueryResult>, DbErr> {
        match self {
            SeaOrmConnection::DbConn(dbconn) => dbconn.query_all(stmt).await,
            SeaOrmConnection::Transaction(tx) => tx.query_all(stmt).await,
        }
    }
}