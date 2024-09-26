use futures::executor::block_on;
use sqlx::{SqliteConnection, SqliteExecutor};

use super::Artist;

impl Artist {
    pub async fn upsert(&self, conn: &mut SqliteConnection) -> Result<Artist, sqlx::Error> {
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

    pub async fn upsert_partial(&self, conn: &mut SqliteConnection) -> Result<Artist, sqlx::Error> {
        let old_data = Self::find_by_mbid(conn, &self.mbid).await?;

        match old_data {
            Some(old) => Self::merge_data(old, self.clone()).upsert(conn).await,
            None => self.upsert(conn).await
        }
    }

    pub fn merge_data(base: Self, new: Self) -> Self {
        Self {
            annotation: new.annotation.or(base.annotation),
            id: base.id,
            country: new.country.or(base.country),
            disambiguation: new.disambiguation,
            mbid: new.mbid,
            name: new.name,
            sort_name: new.sort_name,
        }
    }
}
