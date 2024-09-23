use sqlx::SqlitePool;
use welds::{
    connections::sqlite::{self, SqliteClient},
    WeldsError,
};

use crate::Error;

pub struct DBClient {
    connection: SqliteClient,
}

impl DBClient {
    pub async fn connect(path: &str) -> Result<DBClient, Error> {
        let connection = sqlite::connect(&format!("sqlite:{}", path)).await?;
        sqlx::query("PRAGMA journal_mode=WAL; PRAGMA synchronous = NORMAL;")
            .execute(connection.as_sqlx_pool())
            .await
            .unwrap();
        sqlx::query("PRAGMA busy_timeout=60000")
            .execute(connection.as_sqlx_pool())
            .await
            .unwrap();

        // Commit wal mode
        drop(connection);

        let connection = sqlite::connect(&format!("sqlite:{}", path)).await?;
        Ok(Self { connection })
    }

    pub fn as_welds_client(&self) -> &SqliteClient {
        &self.connection
    }

    pub fn as_sqlx_pool(&self) -> &SqlitePool {
        self.connection.as_sqlx_pool()
    }
}
