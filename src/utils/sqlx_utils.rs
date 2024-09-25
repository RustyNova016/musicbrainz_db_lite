use sqlx::{Acquire, Pool, Sqlite, SqliteConnection, SqliteExecutor, Transaction};

pub trait SqliteAquire<'c>: Acquire<'c, Database = Sqlite> {}

impl<'c> SqliteAquire<'c> for &Pool<Sqlite> {}
impl<'c> SqliteAquire<'c> for &'c mut Transaction<'c, Sqlite> {}
impl<'c> SqliteAquire<'c> for &'c mut SqliteConnection {}

pub trait SqliteAquireRef<'c>: SqliteAquire<'c> + Copy {}
impl<'c, T: SqliteAquire<'c> + Copy> SqliteAquireRef<'c> for T {}

/* pub trait GenericExecutor {
    fn as_executor<'c>(&self) -> impl SqliteExecutor<'c>;
}

impl GenericExecutor for SqlitePool {
    fn as_executor<'c>(&self) -> impl SqliteExecutor<'c> {
        self
    }
}

impl<'a> GenericExecutor for Transaction<'a, Sqlite> {
    fn as_executor<'c>(&mut self) -> impl SqliteExecutor<'c> {
        self
    }
}
 */
