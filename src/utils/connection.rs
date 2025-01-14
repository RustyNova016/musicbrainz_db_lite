use core::future::ready;
use core::future::Ready;
use std::sync::Arc;

use async_fn_stream::try_fn_stream;
use futures::future::BoxFuture;
use futures::TryStreamExt as _;
use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::Acquire;
use sqlx::Connection;
use sqlx::Executor;
use sqlx::Sqlite;
use sqlx::Transaction;
use tokio::sync::RwLock;

/// A database client holding a transaction
pub struct TransactionClient<'l> {
    connection: Transaction<'l, Sqlite>,
    mb_client: &'l MusicBrainzClient,
}

impl<'l> TransactionClient<'l> {
    pub async fn begin(&'l mut self) -> Result<Self, sqlx::Error> {
        let connection = self.connection.begin().await?;

        Ok(Self {
            connection,
            mb_client: &self.mb_client,
        })
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.connection.commit().await
    }
}

/// A wrapper arround sqlx's Sqlite connection.
///
/// Sqlite can only accept one write transaction at the time.
/// If multiple write transations are requested, they will be run one after another, in a first in, last out, order.
///
/// This struct allow to change this order, by providing a first in, first out, `RwLock`. This also prevent hitting write timeouts.
/// This doesn't cover other application using the database at the same time, but it's better than nothing
#[derive(Debug, Clone)]
pub struct DbConnection(Arc<RwLock<sqlx::SqliteConnection>>);

impl DbConnection {
    pub fn new(connection: sqlx::SqliteConnection) -> Self {
        Self(Arc::new(RwLock::new(connection)))
    }

    pub async fn acquire_guarded(
        &self,
    ) -> tokio::sync::RwLockWriteGuard<'_, sqlx::SqliteConnection> {
        self.0.write().await
    }
}



impl<'c> Executor<'c> for DbConnection {
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
        Box::pin(try_fn_stream(|emitter| async move {
            let mut conn = self.0.write().await;
            let mut s = conn.fetch_many(query);

            while let Some(v) = s.try_next().await? {
                emitter.emit(v);
            }

            Ok(())
        }))
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
        Box::pin(async move { self.0.write().await.fetch_optional(query).await })
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
        Box::pin(async move { self.0.write().await.prepare_with(sql, parameters).await })
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e,
    {
        Box::pin(async move { self.0.write().await.describe(sql).await })
    }
}
