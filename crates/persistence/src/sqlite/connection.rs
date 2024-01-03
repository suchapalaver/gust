use std::{env, ops::Deref};

use diesel::{r2d2::ConnectionManager, SqliteConnection};
use r2d2::Pool;

use crate::store::StoreError;

pub struct DbUri(String);

impl Default for DbUri {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for DbUri {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Deref for DbUri {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DbUri {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set")
            .as_str()
            .into()
    }

    pub fn inmem() -> Self {
        Self::from(":memory:")
    }
}

pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

pub(crate) trait Connection {
    async fn try_connect(&self) -> Result<ConnectionPool, StoreError>;
}

pub(crate) struct DatabaseConnector {
    db_uri: DbUri,
}

impl DatabaseConnector {
    pub(crate) fn new(db_uri: DbUri) -> Self {
        Self { db_uri }
    }
}

impl Connection for DatabaseConnector {
    async fn try_connect(&self) -> Result<ConnectionPool, StoreError> {
        use diesel::Connection;
        SqliteConnection::establish(&self.db_uri)?;
        Ok(
            Pool::builder().build(ConnectionManager::<SqliteConnection>::new(
                self.db_uri.deref(),
            ))?,
        )
    }
}
