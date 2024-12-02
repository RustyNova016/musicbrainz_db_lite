use sqlx::FromRow;

use crate::models::musicbrainz::relations::traits::HasRelation;


    pub async fn get_works_or_fetch<'a,  T, U: FromRow<'a>>(
        conn: &mut sqlx::SqliteConnection,
        left: T,
        left_table: &str,
        right_table: &str
    ) -> Result<Vec<U>, crate::Error> {
        // First, make sure all the work of the recording are in the database
        left.fetch_if_incomplete(conn).await?;

        // Next, get all the works
        Ok(sqlx::query_as(format!(
            r#"SELECT
                    {right_table}.*
                FROM
                    works
                    INNER JOIN l_{left_table}_{right_table} as rel ON {right_table}.id = rel.entity1
                    INNER JOIN {left_table} ON rel.entity0 = {left_table}.id
                WHERE
                    {left_table}.id = ?"#,
        ))
        .bind(self.id)
        .fetch_all(conn)
        .await?)
    }