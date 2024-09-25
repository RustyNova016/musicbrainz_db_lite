use sqlx::{Acquire, Sqlite, SqliteConnection, SqliteExecutor, Transaction};

use crate::utils::sqlx_utils::SqliteAquire;

use super::Artist;

impl Artist {
    /// Link an mbid to the actual entity
    pub async fn add_redirection(
        conn: &mut SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO artists_gid_redirect VALUES (?, ?, 0) ON CONFLICT DO UPDATE SET `new_id` = ?",
            mbid,
            id,
            id
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    /// Add an mbid in the redirect pool if it isn't in yet.
    pub async fn add_mbid(conn: &mut SqliteConnection, mbid: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT OR IGNORE INTO `artists_gid_redirect` VALUES (?, NULL, 0)",
            mbid
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn find_by_mbid(
        conn: &mut SqliteConnection,
        mbid: &str,
    ) -> Result<Option<Artist>, sqlx::Error> {
        sqlx::query_as!(
            Artist,
            r#"
            SELECT
                artists.*
            FROM
                artists
                INNER JOIN artists_gid_redirect ON artists.id = artists_gid_redirect.new_id
            WHERE
                artists_gid_redirect.gid = ?
            LIMIT
                1
        "#,
            mbid
        )
        .fetch_optional(conn)
        .await
    }
}
