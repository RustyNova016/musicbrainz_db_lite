use std::fmt::Debug;

use builder::ClientBuilder;
use macon::Builder;
use musicbrainz_rs_nova::client::MusicBrainzClient;

use crate::utils::connection::DbConnection;

pub mod builder;
pub mod client_connection;
pub mod database_connection;

#[derive(Builder)]
pub struct DBClient {
    pub connection: DbConnection,

    pub musicbrainz_client: MusicBrainzClient,
}

impl DBClient {
    pub fn new() -> ClientBuilder<(), (), ()> {
        ClientBuilder::default()
    }

    #[cfg(test)]
    /// Create an in memory database with the default MB client
    pub async fn test_client() -> Result<Self, crate::Error> {
        Ok(Self::new().in_memory().set_default_mb_client().connect_and_migrate().await?.build())
    }
}

// pub trait Client {
//     async fn acquire<'l>(&mut self) -> Result<&mut sqlx::SqliteConnection, crate::Error>;

//     fn get_mb_client(&self) -> &MusicBrainzClient;
// }

// impl Client for DBClient {
//     async fn acquire<'l>(&mut self) -> Result<&mut sqlx::SqliteConnection, crate::Error> {
//         Ok(self.connection.aquire().await)
//     }

//     fn get_mb_client(&self) -> &MusicBrainzClient {
//         &self.musicbrainz_client
//     }
// }

//TODO: https://github.com/RustyNova016/musicbrainz_rs_nova/issues/73 + Change this
impl Debug for DBClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DBClient")
            .field("connection", &self.connection)
            .finish()
    }
}
