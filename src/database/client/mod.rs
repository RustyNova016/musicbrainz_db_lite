use core::str::FromStr;

use std::fs::File;
use std::sync::Arc;

use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::Connection as _;
use sqlx::SqliteConnection;
use tokio::sync::RwLock;
use std::time::Duration;

use crate::ClientConnection;
use crate::Error;

pub mod client_connection;

pub struct DBClient {
    pub connection: Arc<RwLock<SqliteConnection>>,

    pub musicbrainz_client: MusicBrainzClient,
}

impl DBClient {
    /// Connect to a database file. It will also create/migrate the schema on load
    pub async fn connect(path: &str) -> Result<DBClient, Error> {
        let optconn = SqliteConnectOptions::from_str(&format!("sqlite:{}", path))?
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_millis(60000));

        let mut connection = SqliteConnection::connect_with(&optconn).await?;

        musicbrainz_db_lite_schema::create_and_migrate(&mut connection).await?;

        Ok(Self {
            connection: Arc::new(RwLock::new(connection)),
            musicbrainz_client: Default::default(),
        })
    }

    pub async fn a(&self) {
        self.connection.write().await.begin().await.unwrap()
    }

    /// Create the database file and the database
    pub async fn create_database_file(path: &str) -> Result<Self, Error> {
        File::create_new(path).unwrap();
        let new = Self::connect(path).await?;
        new.create_database().await?;

        Ok(new)
    }

    pub async fn create_database(&self) -> Result<(), Error> {
        musicbrainz_db_lite_schema::create_and_migrate(&mut self.connection)
            .await?;

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
            musicbrainz_client: Default::default(),
        })
    }

    pub async fn connect_in_memory_and_create() -> Result<DBClient, Error> {
        let client = Self::connect_in_memory().await?;
        client.create_database().await?;
        Ok(client)
    }
}

pub trait Client {
    async fn acquire<'l>(&mut  self) -> Result<&mut sqlx::SqliteConnection, crate::Error>;

    fn get_mb_client(&self) -> &MusicBrainzClient;
}

impl Client for DBClient {
    async fn acquire<'l>(& mut self) -> Result<&mut sqlx::SqliteConnection, crate::Error> {
        Ok(self.connection.write().await.acquire().await?)
    }

    fn get_mb_client(&self) -> &MusicBrainzClient {
        &self.musicbrainz_client
    }
}
mod tests {
    use chrono::Utc;

    use super::DBClient;

    impl DBClient {
        pub async fn create_test_file_database() -> Result<Self, crate::Error> {
            Self::create_database_file(&format!(
                "./tests/results/data_{}.db",
                Utc::now().timestamp()
            ))
            .await
        }
    }
}
