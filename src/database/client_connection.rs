use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::Acquire;
use sqlx::Executor;
use sqlx::Sqlite;
use tokio::sync::MutexGuard;
use tokio::sync::OwnedMutexGuard;

// use crate::database::transaction::Transaction;
use crate::utils::sqlx_utils::db_connection::DbConnection;

#[derive(Debug)]
pub struct ClientConnection<'c> {
    pub(crate) connection: DbConnection<'c>,
    pub(crate) musicbrainz_rs: &'c MusicBrainzClient,
}

impl<'c> ClientConnection<'c> {
    //     pub async fn begin<'d>(&self) -> Transaction<'d> {
    //         let trans = self.connection.acquire_owned().await;
            

    //     Transaction {
    //         musicbrainz_rs: self.musicbrainz_rs,
    //         connection: trans.begin().await?
    //     }
    // }
}

impl<'c> Executor<'c> for &'c ClientConnection<'c> {
    type Database = Sqlite;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> futures::stream::BoxStream<
        'e,
        Result<
            sqlx::Either<
                <Self::Database as sqlx::Database>::QueryResult,
                <Self::Database as sqlx::Database>::Row,
            >,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        self.connection.fetch_many(query)
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> futures::future::BoxFuture<
        'e,
        Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
    >
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        self.connection.fetch_optional(query)
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> futures::future::BoxFuture<
        'e,
        Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>,
    >
    where
        'c: 'e,
    {
        self.connection.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e,
    {
        self.connection.describe(sql)
    }
}
