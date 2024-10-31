pub mod join_map;
//pub mod query_builder;
pub mod entity_relations;
use core::ops::{Deref, DerefMut};


use sqlx::{Acquire, Pool, Sqlite, SqliteConnection, Transaction};



// pub trait SqliteAquire<'c>: Acquire<'c, Database = Sqlite> {}

// impl<'c> SqliteAquire<'c> for &Pool<Sqlite> {}
// impl<'c> SqliteAquire<'c> for &'c mut Transaction<'c, Sqlite> {}
// impl<'c> SqliteAquire<'c> for &'c mut SqliteConnection {}

// pub trait SqliteAquireRef<'c>: SqliteAquire<'c> + Copy {}
// impl<'c, T: SqliteAquire<'c> + Copy> SqliteAquireRef<'c> for T {}

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

pub struct ConnectionWithPool<'conn> {
    conn: ConnectionType<'conn>,
    read_pool: Pool<Sqlite>,
}

impl<'conn> ConnectionWithPool<'conn> {
    pub fn new(conn: ConnectionType<'conn>, read_pool: Pool<Sqlite>) -> Self {
        Self { conn, read_pool }
    }

    pub async fn begin<'s: 'conn>(&'s mut self) -> Result<ConnectionWithPool<'s>, sqlx::Error> {
        Ok(Self {
            conn: self.conn.begin().await?,
            read_pool: self.read_pool.clone(),
        })
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.conn.commit().await
    }

    pub fn write(&mut self) -> &mut SqliteConnection {
        self.deref_mut()
    }

    pub async fn read(&self) -> Result<sqlx::pool::PoolConnection<sqlx::Sqlite>, sqlx::Error> {
        self.read_pool.acquire().await
    }
}

impl<'conn> Deref for ConnectionWithPool<'conn> {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl<'conn> DerefMut for ConnectionWithPool<'conn> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

pub enum ConnectionType<'c> {
    Raw(SqliteConnection),
    Transaction(Transaction<'c, Sqlite>),
}

impl<'c> ConnectionType<'c> {
    pub async fn begin<'s: 'c>(&'s mut self) -> Result<ConnectionType<'s>, sqlx::Error> {
        let trans = match self {
            Self::Raw(v) => v.begin().await,
            Self::Transaction(v) => v.begin().await,
        }?;

        Ok(Self::Transaction(trans))
    }

    pub async fn commit(self) -> Result<(), sqlx::Error> {
        match self {
            Self::Transaction(v) => v.commit().await,
            _ => Ok(()),
        }
    }
}

impl<'c> From<SqliteConnection> for ConnectionType<'c> {
    fn from(value: SqliteConnection) -> Self {
        Self::Raw(value)
    }
}

impl<'c> Deref for ConnectionType<'c> {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Raw(v) => v,
            Self::Transaction(v) => v,
        }
    }
}

impl<'c> DerefMut for ConnectionType<'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Raw(v) => v,
            Self::Transaction(v) => &mut *v,
        }
    }
}
