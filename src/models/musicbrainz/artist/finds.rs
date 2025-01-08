use sqlx::Executor;
use sqlx::SqliteExecutor;

use crate::models::shared_traits::find_by_mbid::FindByMBID;
use crate::models::shared_traits::find_by_rowid::FindByRowID;

use super::Artist;

impl FindByRowID for Artist {
    async fn find_by_rowid(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(Self::find_by_id_column(conn, id).await?)
    }
}

impl FindByMBID for Artist {
    async fn find_by_mbid<E>(
        conn: E,
        id: &str,
    ) -> Result<Option<Self>, crate::Error> where E: for<'e> SqliteExecutor<'e> {
        Ok(Self::find_by_mbid(conn, id).await?)
    }
}
