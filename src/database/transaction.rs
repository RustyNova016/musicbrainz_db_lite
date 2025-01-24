use std::sync::Arc;

use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::Acquire as _;
use sqlx::Sqlite;
use sqlx::Transaction as SqlxTransaction;
use tokio::sync::MutexGuard;
use tokio::sync::OwnedMutexGuard;

use crate::database::client::DBClient;
use crate::database::client_connection::ClientConnection;
use crate::database::client_like::ClientLike;
use crate::utils::sqlx_utils::db_connection::DbConnection;
use crate::utils::sqlx_utils::get_exec::GetExecutor;

// pub struct Transaction<'c, 'g> {
//     //guard: MutexGuard<'g, &'c mut sqlx::SqliteConnection>,
//     pub connection: SqlxTransaction<'g, Sqlite>,
//     pub musicbrainz_rs: &'c MusicBrainzClient,
// }

// impl<'c, 'g> Transaction<'c, 'g> {
//     pub async fn new(conn: &'g ClientConnection<'c>) -> Result<Self, crate::Error> {
//         let mut guard = conn.connection.acquire_owned().await;
//         Ok(Self {

//             connection: guard.begin().await?,
//             musicbrainz_rs: conn.musicbrainz_rs,
//         })
//     }
// }

// impl<'c> ClientLike<'c> for &'c mut Transaction<'c> {
//     fn get_mb_client(self) -> &'c MusicBrainzClient {
//         &self.musicbrainz_rs
//     }
// }

// impl<'c> GetExecutor<'c> for &'c mut Transaction<'c> {
//     async fn get_executor(self) -> Result<impl sqlx::SqliteExecutor<'c>, crate::Error> {
//         Ok(DbConnection::new(self.connection.acquire().await?))
//     }
// }

// pub trait StartTransaction<'c> {
//     fn begin(
//         self,
//     ) -> impl std::future::Future<Output = Result<Transaction<'c>, crate::Error>> + Send;
// }

// impl<'c> StartTransaction<'c> for &'c mut DBClient {
//     async fn begin(self) -> Result<Transaction<'c>, crate::Error> {
//         Ok(Transaction {
//             connection: self.connection.begin().await?,
//             musicbrainz_rs: self.musicbrainz_rs.clone(),
//         })
//     }
// }

// impl<'c> StartTransaction<'c> for &'c mut Transaction<'c> {
//     async fn begin(self) -> Result<Transaction<'c>, crate::Error> {
//         Ok(Transaction {
//             connection: self.connection.begin().await?,
//             musicbrainz_rs: self.musicbrainz_rs.clone(),
//         })
//     }
// }

// impl<'c> StartTransaction<'c> for &'c mut ClientConnection<'c> {
//     async fn begin(self) -> Result<Transaction<'c>, crate::Error> {
//         Ok(Transaction {
//             connection: self.connection.begin().await?,
//             musicbrainz_rs: self.musicbrainz_rs.clone(),
//         })
//     }
// }
