use core::future::Future;
use core::ops::Deref;
use core::str::FromStr;
use futures::future::BoxFuture;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Acquire, Database, Executor, Pool, Sqlite, SqliteConnection, SqlitePool};
use std::time::Duration;
use tokio::sync::Mutex;
use welds::connections::sqlite::{self, SqliteClient};

use crate::Error;

pub struct DBClient {
    welds_connection: SqliteClient,
    pub connection: Pool<Sqlite>,
}

impl DBClient {
    pub async fn connect(path: &str) -> Result<DBClient, Error> {
        let connection = sqlite::connect(&format!("sqlite:{}", path)).await?;

        let optconn = SqliteConnectOptions::from_str(&format!("sqlite:{}", path))?
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_millis(60000));

        Ok(Self {
            welds_connection: connection,
            connection: SqlitePoolOptions::new().acquire_timeout(Duration::from_millis(60000)).connect_lazy_with(optconn),
        })
    }

    //pub async fn create_database(&self) -> Result<(), Error> {
    //    create_database(self.as_welds_client()).await
    //}

    pub fn as_welds_client(&self) -> &SqliteClient {
        &self.welds_connection
    }
}

