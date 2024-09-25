use sqlx::SqliteExecutor;

use super::Artist;

impl Artist {
    pub async fn upsert(&self, conn: impl SqliteExecutor<'_>) -> Result<Artist, sqlx::Error> {
        sqlx::query_as!(
            Artist,
            r#"
        INSERT INTO
            artists
        VALUES
            (NULL, ?, ?, ?, ?, ?, ?)
        ON CONFLICT (mbid) DO
        UPDATE
        SET
            name = excluded.name,
            sort_name = excluded.sort_name,
            disambiguation = excluded.disambiguation,
            country = excluded.country,
            annotation = excluded.annotation
        RETURNING *;"#,
            self.mbid,
            self.name,
            self.sort_name,
            self.disambiguation,
            self.country,
            self.annotation
        )
        .fetch_one(conn)
        .await
    }
}
