use core::fmt::Debug;

use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::Executor;
use sqlx::Sqlite;

pub struct ClientConnection<'l> {
    pub connection: &'l mut sqlx::SqliteConnection,
    pub mb_client: &'l MusicBrainzClient,
}

impl<'l> ClientConnection<'l> {
    pub fn get_mb_client(&self) -> &MusicBrainzClient {
        self.mb_client
    }

    pub fn as_sqlx_connection(&'l mut self) -> &'l mut sqlx::SqliteConnection {
        self.connection
    }
}

//TODO: https://github.com/RustyNova016/musicbrainz_rs_nova/issues/73 + Change this
impl Debug for ClientConnection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConnection")
            .field("connection", &self.connection)
            .finish()
    }
}

impl<'l> Executor<'l> for &'l mut ClientConnection<'l> {
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
        'l: 'e,
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
        'l: 'e,
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
        'l: 'e,
    {
        self.connection.prepare_with(sql, parameters)
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'l: 'e,
    {
        self.connection.describe(sql)
    }
}
