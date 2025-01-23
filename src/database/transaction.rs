use std::sync::Arc;
use std::sync::RwLock;

use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::Acquire as _;
use sqlx::Sqlite;
use sqlx::Transaction as SqlxTransaction;

use crate::database::client::DBClient;

pub struct Transaction<'c> {
    pub connection: SqlxTransaction<'c, Sqlite>,
    pub musicbrainz_rs: Arc<RwLock<MusicBrainzClient>>,
}

pub trait StartTransaction<'c> {
    fn begin(self) -> impl std::future::Future<Output = Result<Transaction<'c>, crate::Error>> + Send;
}

impl<'c> StartTransaction<'c> for &DBClient {
    async fn begin(self) -> Result<Transaction<'c>, crate::Error> {
        Ok(Transaction {
            connection: self.connection.begin().await?,
            musicbrainz_rs: self.musicbrainz_rs.clone(),
        })
    }
}

impl<'c> StartTransaction<'c> for &'c mut Transaction<'c> {
    async fn begin(self) -> Result<Transaction<'c>, crate::Error> {
        Ok(Transaction {
            connection: self.connection.begin().await?,
            musicbrainz_rs: self.musicbrainz_rs.clone(),
        })
    }
}
