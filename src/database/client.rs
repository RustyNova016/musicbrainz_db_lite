use std::sync::Arc;
use std::sync::RwLock;

use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::{Pool, Sqlite};

pub struct DBClient {
    pub connection: Pool<Sqlite>,
    pub musicbrainz_rs: Arc<RwLock<MusicBrainzClient>>,
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
            client.read_database(&path)?;
            client.migrate_database().await?;

            client.build()
        }

        pub async fn connect_in_memory_and_create() -> Result<Self, crate::Error> {
            let mut client = DBClientBuilder::default();
            client.set_musicbrainz_client(Default::default());
            let path = ":memory:".to_string();
            client.create_database_if_missing(Path::new(&path))?;
            client.read_database(&path)?;
            client.migrate_database().await?;

            client.build()
        }
    }
}
