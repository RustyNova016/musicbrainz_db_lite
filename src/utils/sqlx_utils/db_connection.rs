use std::sync::Arc;

use async_fn_stream::try_fn_stream;
use futures::TryStreamExt as _;
use sqlx::Acquire;
use sqlx::Executor;
use sqlx::Sqlite;
use sqlx::Transaction;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

/// A wrapper arround sqlx's Sqlite connection.
///
/// Sqlite can only accept one write transaction at the time.
/// If multiple write transations are requested, they will be run one after another, in a first in, last out, order.
///
/// This struct allow to change this order, by providing a first in, first out `RwLock`. This also prevent hitting write timeouts.
/// This doesn't cover other application using the database at the same time, but it's better than nothing
#[derive(Debug, Clone)]
pub struct DbConnection<'c>(Arc<Mutex<&'c mut sqlx::SqliteConnection>>);

impl<'c> DbConnection<'c> {
    pub fn new(connection: &'c mut sqlx::SqliteConnection) -> Self {
        Self(Arc::new(Mutex::new(connection)))
    }

    pub async fn acquire_guarded(
        &self,
    ) -> tokio::sync::MutexGuard<'_, &'c mut sqlx::SqliteConnection> {
        self.0.lock().await
    }

    pub async fn from_sqlx_transaction(
        trans: &'c mut Transaction<'c, Sqlite>,
    ) -> Result<Self, crate::Error> {
        let conn = trans.acquire().await?;
        Ok(Self::new(conn))
    }
}

impl<'c> Executor<'c> for DbConnection<'c> {
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
            let mut conn = self.0.lock().await;
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
        Box::pin(async move { self.0.lock().await.fetch_optional(query).await })
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
        Box::pin(async move { self.0.lock().await.prepare_with(sql, parameters).await })
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e,
    {
        Box::pin(async move { self.0.lock().await.describe(sql).await })
    }
}
