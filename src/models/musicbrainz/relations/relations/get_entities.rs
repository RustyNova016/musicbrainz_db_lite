use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;

use crate::models::musicbrainz::relations::traits::HasRelation;
use crate::models::musicbrainz::relations::Relation;
use crate::models::shared_traits::has_table::HasTable;

impl<T, U> Relation<T, U>
where
    T: for<'a> FromRow<'a, SqliteRow> + Send + Unpin + HasRelation<U>,
    U: for<'a> FromRow<'a, SqliteRow> + HasTable + Send + Unpin + Clone,
{
    async fn get_entity_inner<V>(
        &self,
        conn: &mut sqlx::SqliteConnection,
        entity_num: &str,
    ) -> Result<Vec<V>, crate::Error>
    where
        V: for<'a> FromRow<'a, SqliteRow> + Send + Unpin,
    {
        Ok(sqlx::query_as(&format!(
            r#"SELECT
                        right.*
                    FROM
                        {right_table} as right
                        INNER JOIN {left_table} as left ON right.id = left.entity{entity_num}
                    WHERE
                        left.id = ?"#,
            left_table = T::RELATION_TABLE,
            right_table = U::TABLE_NAME
        ))
        .bind(self.id)
        .fetch_all(conn)
        .await?)
    }

    pub async fn get_entity_0_as_left(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<T>, crate::Error> {
        self.get_entity_inner(conn, "0").await
    }

    pub async fn get_entity_0_as_right(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<U>, crate::Error> {
        self.get_entity_inner(conn, "0").await
    }

    pub async fn get_entity_1_as_left(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<T>, crate::Error> {
        self.get_entity_inner(conn, "1").await
    }

    pub async fn get_entity_1_as_right(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<U>, crate::Error> {
        self.get_entity_inner(conn, "1").await
    }
}
