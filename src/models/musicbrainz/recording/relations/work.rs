use std::collections::HashMap;

use itertools::Itertools;
use sqlx::SqliteConnection;

use crate::models::musicbrainz::work::Work;
use crate::utils::sqlx_utils::entity_relations::{JoinCollection, JoinRelation};

use super::Recording;

impl Recording {
    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_works_or_fetch(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Work>, crate::Error> {
        // First, make sure all the work of the recording are in the database
        self.fetch_if_incomplete(conn).await?;

        // Next, get all the works
        Ok(sqlx::query_as(
            r#"SELECT
                    works.*
                FROM
                    works
                    INNER JOIN l_releases_works as rel ON works.id = rel.entity1
                    INNER JOIN recordings ON rel.entity0 = recordings.id
                WHERE
                    recordings.id = ?"#,
        )
        .bind(self.id)
        .fetch_all(conn)
        .await?)
    }

    /// Get a all the releases of a list of recordings.
    ///
    /// ⚠️ The recordings must all be fetched before. A `debug_assert` will block in case of, but won't trigger in production
    pub async fn get_works_as_batch<'r>(
        conn: &mut sqlx::SqliteConnection,
        recordings: &'r [&'r Recording],
    ) -> Result<HashMap<i64, (&'r &'r Recording, Vec<Work>)>, crate::Error> {
        #[cfg(debug_assertions)]
        //Self::assert_recordings_fetched(recordings); TODO: Fix borow types
        let ids = recordings.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, Work>> = sqlx::query_as(
            "
            SELECT
                recordings.id as original_id,
                works.*
            FROM
                works
                INNER JOIN l_recordings_works as rel ON works.id = rel.entity1
                INNER JOIN recordings ON rel.entity0 = recordings.id
            WHERE
                recordings.id IN (
                    SELECT
                        value
                    FROM
                        JSON_EACH(?)
                )
        ",
        )
        .bind(id_string)
        .fetch_all(conn)
        .await?;

        Ok(JoinCollection::from(joins).into_hashmap(recordings, |id, value| &value.id == id))
    }
}
