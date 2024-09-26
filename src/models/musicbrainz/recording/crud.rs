use sqlx::SqliteConnection;

use super::Recording;

impl Recording {
    pub async fn upsert(&self, conn: &mut SqliteConnection) -> Result<Recording, sqlx::Error> {
        sqlx::query_as!(
            Recording,
            r#"
            INSERT INTO
                recordings
            VALUES
                (NULL, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (mbid) DO
            UPDATE
            SET
                title = excluded.title,
                length = excluded.length,
                disambiguation = excluded.disambiguation,
                annotation = excluded.annotation,
                artist_credit = excluded.artist_credit
            RETURNING *;"#,
            self.mbid,
            self.title,
            self.length,
            self.disambiguation,
            self.annotation,
            self.artist_credit
        )
        .fetch_one(conn)
        .await
    }
}