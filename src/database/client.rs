use core::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::time::Duration;

use crate::Error;

pub struct DBClient {
    pub connection: Pool<Sqlite>,
}

impl DBClient {
    pub async fn connect(path: &str) -> Result<DBClient, Error> {
        let optconn = SqliteConnectOptions::from_str(&format!("sqlite:{}", path))?
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_millis(60000));

        Ok(Self {
            connection: SqlitePoolOptions::new()
                .acquire_timeout(Duration::from_millis(60000))
                .connect_lazy_with(optconn),
        })
    }

    pub async fn create_database(&self) -> Result<(), Error> {
        musicbrainz_db_lite_schema::create_database(&mut *self.connection.acquire().await?).await?;

        Ok(())
    }

    pub async fn connect_in_memory() -> Result<DBClient, Error> {
        let optconn = SqliteConnectOptions::from_str("sqlite::memory:")?
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_millis(60000));

        Ok(Self {
            connection: SqlitePoolOptions::new()
                .acquire_timeout(Duration::from_millis(60000))
                .connect_lazy_with(optconn),
        })
    }
}
