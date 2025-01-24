use std::sync::Arc;
use std::sync::RwLock;

use musicbrainz_rs_nova::client::MusicBrainzClient;

use crate::database::client_connection::ClientConnection;
use crate::database::client_like::ClientLike;
use crate::utils::sqlx_utils::db_connection::DbConnection;
use crate::utils::sqlx_utils::get_exec::GetExecutor;

pub struct DBClient {
    pub connection: sqlx::SqliteConnection,
    pub musicbrainz_rs: Arc<MusicBrainzClient>,
}

impl DBClient {
    pub fn acquire<'c>(&'c mut self) -> ClientConnection<'c> {
        ClientConnection {
            musicbrainz_rs: self.musicbrainz_rs.as_ref(),
            connection: DbConnection::new(&mut self.connection),
        }
    }
}

mod tests {
    use std::path::Path;

    use chrono::Utc;

    use crate::database::builder::DBClientBuilder;

    use super::DBClient;

    impl DBClient {
        pub async fn create_test_file_database() -> Result<Self, crate::Error> {
            let mut client = DBClientBuilder::default();
            client.set_musicbrainz_client(Default::default());
            let path = format!("./tests/results/data_{}.db", Utc::now().timestamp());
            client.create_database_if_missing(Path::new(&path))?;
            client.read_database(&path).await?;
            client.migrate_database().await?;

            client.build()
        }

        pub async fn connect_in_memory_and_create() -> Result<Self, crate::Error> {
            let mut client = DBClientBuilder::default();
            client.set_musicbrainz_client(Default::default());
            let path = ":memory:".to_string();
            client.create_database_if_missing(Path::new(&path))?;
            client.read_database(&path).await?;
            client.migrate_database().await?;

            client.build()
        }
    }
}
