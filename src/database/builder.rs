use core::str::FromStr as _;
use core::time::Duration;
use std::fs::File;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

use musicbrainz_db_lite_schema::create_and_migrate;
use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;

use crate::database::client::DBClient;

#[derive(Default)]
pub struct DBClientBuilder {
    pub database_client: Option<sqlx::SqlitePool>,
    pub musicbrainz_client: Option<MusicBrainzClient>,
}

impl DBClientBuilder {
    pub fn set_musicbrainz_client(&mut self, client: MusicBrainzClient) {
        self.musicbrainz_client = Some(client)
    }

    pub fn create_database_if_missing(&self, path: &Path) -> Result<(), crate::Error> {
        if path.exists() {
            return Ok(());
        }

        match File::create_new(path) {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.kind() == io::ErrorKind::AlreadyExists {
                    return Ok(());
                }

                Err(err.into())
            }
        }
    }

    pub fn read_database(&mut self, database_path: &str) -> Result<(), sqlx::Error> {
        let optconn = SqliteConnectOptions::from_str(database_path)?
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_millis(60000));

        let pool = SqlitePoolOptions::new()
            .acquire_timeout(Duration::from_millis(60000))
            .connect_lazy_with(optconn);
        self.database_client = Some(pool);

        Ok(())
    }

    pub async fn migrate_database(&self) -> Result<(), crate::Error> {
        Ok(create_and_migrate(
            self.database_client
                .as_ref()
                .expect("No database connection was established"),
        )
        .await?)
    }

    pub fn build(self) -> Result<DBClient, crate::Error> {
        Ok(DBClient {
            connection: self
                .database_client
                .ok_or(crate::Error::ClientBuildingError("connection".to_string()))?,
            musicbrainz_rs: Arc::new(RwLock::new(self.musicbrainz_client.ok_or(
                crate::Error::ClientBuildingError("musicbrainz_client".to_string()),
            )?)),
        })
    }
}
