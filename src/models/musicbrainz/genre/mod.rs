pub mod genre_tag;
use sqlx::prelude::FromRow;

use crate::RowId;

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
pub struct Genre {
    pub id: i64,
    pub mbid: String,
    pub name: String,
    pub disambiguation: Option<String>,
}

impl Genre {
    pub async fn upsert(&mut self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let returned: Genre = sqlx::query_as(
            "
        INSERT INTO
            `genres` (
                `mbid`,
                `name`,
                `disambiguation`
            )
        VALUES
            (?, ?, ?)
        ON CONFLICT DO
        UPDATE
        SET
            `disambiguation` = excluded.`disambiguation`
        RETURNING *;",
        )
        .bind(&self.mbid)
        .bind(&self.name)
        .bind(&self.disambiguation)
        .fetch_one(conn)
        .await?;

        *self = returned;

        Ok(())
    }
}

impl RowId for Genre {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}
