use async_fn_stream::try_fn_stream;
use futures::future::BoxFuture;
use sqlx::Executor;
use sqlx::SqliteExecutor;

pub mod join_map;
//pub mod query_builder;
pub mod entity_relations;

pub trait ExecutorRef<'e> where &'e Self: SqliteExecutor<'e> + 'e {}

pub trait 

// impl<'l, 'c, T> Executor<'l> for T
// where
//     T: AcquireExec<'c>,
// {
//     type Database = sqlx::Sqlite;

//     fn fetch_many<'e, 'q: 'e, E>(
//         self,
//         query: E,
//     ) -> futures::stream::BoxStream<
//         'e,
//         Result<
//             sqlx::Either<
//                 <Self::Database as sqlx::Database>::QueryResult,
//                 <Self::Database as sqlx::Database>::Row,
//             >,
//             sqlx::Error,
//         >,
//     >
//     where
//         'l: 'e,
//         E: 'q + sqlx::Execute<'q, Self::Database>,
//     {
//         Box::pin(async_fn_stream::try_fn_stream(|emitter| async move {
//             let mut conn = self.acquire_exec().await?;
//             let mut s = conn.fetch_many(query);

//             while let Some(v) = futures::TryStreamExt::try_next(&mut s).await? {
//                 emitter.emit(v);
//             }

//             Ok(())
//         }))
//     }

//     fn fetch_optional<'e, 'q: 'e, E>(
//         self,
//         query: E,
//     ) -> futures::future::BoxFuture<
//         'e,
//         Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
//     >
//     where
//         'l: 'e,
//         E: 'q + sqlx::Execute<'q, Self::Database>,
//     {
//         Box::pin(async {
//             match self.acquire_exec().await {
//                 Err(err) => Err(err),
//                 Ok(conn) => conn.fetch_optional(query).await,
//             }
//         })
//     }

//     fn prepare_with<'e, 'q: 'e>(
//         self,
//         sql: &'q str,
//         parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
//     ) -> futures::future::BoxFuture<
//         'e,
//         Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>,
//     >
//     where
//         'l: 'e,
//     {
//         Box::pin(async {
//             match self.acquire_exec().await {
//                 Err(err) => Err(err),
//                 Ok(conn) => conn.prepare_with(sql, parameters).await,
//             }
//         })
//     }

//     fn describe<'e, 'q: 'e>(
//         self,
//         sql: &'q str,
//     ) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
//     where
//         'l: 'e,
//     {
//         Box::pin(async {
//             match self.acquire_exec().await {
//                 Err(err) => Err(err),
//                 Ok(conn) => conn.describe(sql).await,
//             }
//         })
//     }
// }

// impl<'c> AcquireExec<'c> for &DbConnection {
//     fn acquire_exec<'e>(self) -> BoxFuture<'static, Result<impl Executor<'e>, sqlx::Error>> {
//         Box::pin(ready(Ok(self)))
//     }
// }
