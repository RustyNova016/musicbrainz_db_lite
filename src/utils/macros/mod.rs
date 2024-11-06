pub mod artist_credits;
pub mod get_and_fetch;
macro_rules! impl_redirections {
    ($row_struct: ty, $entity_table_name: expr) => {
        impl $row_struct {
            /// Add an mbid in the redirect pool if it isn't in yet.
            pub async fn add_redirect_mbid(
                conn: &mut sqlx::SqliteConnection,
                mbid: &str,
            ) -> Result<(), sqlx::Error> {
                sqlx::query(concat!(
                    "INSERT OR IGNORE INTO `",
                    $entity_table_name,
                    "_gid_redirect` VALUES (?, NULL, 0)"
                ))
                .bind(mbid)
                .execute(conn)
                .await?;
                Ok(())
            }

            /// Link an mbid to the actual entity
            pub async fn set_redirection(
                conn: &mut sqlx::SqliteConnection,
                mbid: &str,
                id: i64,
            ) -> Result<(), sqlx::Error> {
                sqlx::query(concat!(
                    "INSERT OR IGNORE INTO `",
                    $entity_table_name,
                    "_gid_redirect` VALUES (?, ?, 0) ON CONFLICT DO UPDATE SET `new_id` = ?"
                ))
                .bind(mbid)
                .bind(id)
                .bind(id)
                .execute(conn)
                .await?;
                Ok(())
            }

            pub async fn find_by_mbid(
                conn: &mut sqlx::SqliteConnection,
                mbid: &str,
            ) -> Result<Option<$row_struct>, sqlx::Error> {
                sqlx::query_as(&format!(
                    r#"
                    SELECT
                        {}.*
                    FROM
                        {}
                        INNER JOIN {}_gid_redirect ON {}.id = {}_gid_redirect.new_id
                    WHERE
                        {}_gid_redirect.gid = ?
                        AND deleted = 0
                    LIMIT
                        1
                "#,
                    $entity_table_name,
                    $entity_table_name,
                    $entity_table_name,
                    $entity_table_name,
                    $entity_table_name,
                    $entity_table_name
                ))
                .bind(mbid)
                .fetch_optional(conn)
                .await
            }

            pub async fn get_mbids_of_entity(
                conn: &mut sqlx::SqliteConnection,
                id: i64,
            ) -> Result<Vec<String>, sqlx::Error> {
                sqlx::query_scalar(&format!(
                    "SELECT gid FROM `{}_gid_redirect` WHERE new_id = ?",
                    $entity_table_name
                ))
                .bind(id)
                .fetch_all(conn)
                .await
            }
        }
    };
}

pub(crate) use impl_redirections;
