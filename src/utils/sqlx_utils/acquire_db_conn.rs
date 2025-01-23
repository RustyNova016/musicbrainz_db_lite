use sqlx::Executor;
use sqlx::Sqlite;

use crate::database::client::DBClient;
use crate::utils::sqlx_utils::db_connection::DbConnection;

pub trait AcquireDbConnection<'c> {
    async fn acquire(self) -> Result<DbConnection<'c>, crate::Error>;
}

// impl<'c> AcquireDbConnection<'c> for DBClient {
//     async fn acquire(self) -> Result<DbConnection<'c>, crate::Error> {
//         Ok(DbConnection::new(&mut *self.connection.acquire().await?))
//     }
// }

// impl<'c> Executor<'c> for DBClient {
//     type Database = Sqlite;

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
//         'c: 'e,
//         E: 'q + sqlx::Execute<'q, Self::Database>,
//     {
//         self.acquire().fetch_many(query)
//     }

//     fn fetch_optional<'e, 'q: 'e, E>(
//         self,
//         query: E,
//     ) -> futures::future::BoxFuture<
//         'e,
//         Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
//     >
//     where
//         'c: 'e,
//         E: 'q + sqlx::Execute<'q, Self::Database>,
//     {
//         self.acquire().fetch_optional(query)
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
//         'c: 'e,
//     {
//         self.acquire().prepare_with(sql, parameters)
//     }

//     fn describe<'e, 'q: 'e>(
//         self,
//         sql: &'q str,
//     ) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
//     where
//         'c: 'e,
//     {
//         self.acquire().describe(sql)
//     }
// }
