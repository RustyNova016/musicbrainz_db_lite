use std::collections::HashMap;

use itertools::Itertools;
use sqlx::SqliteConnection;

use crate::models::musicbrainz::release::Release;
use crate::utils::sqlx_utils::entity_relations::{JoinCollection, JoinRelation};

use super::Recording;

impl Recording {
    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_releases_or_fetch(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Release>, crate::Error> {
        // First, make sure all the releases of the recording are in the database
        self.fetch_if_incomplete(conn).await?;

        // Next, get all the releases
        Ok(
            sqlx::query_as(
                r#"SELECT
                    releases.*
                FROM
                    releases
                    INNER JOIN medias ON medias.`release` = releases.id
                    INNER JOIN tracks ON tracks.media = medias.id
                    INNER JOIN recordings_gid_redirect ON recordings_gid_redirect.gid = tracks.recording
                    INNER JOIN recordings ON recordings.id = recordings_gid_redirect.new_id
                WHERE
                    recordings.id = ?"#,
                
                 ).bind(self.id)
                 .fetch_all(conn)
                 .await?)
    } 

    /// Get a all the releases of a list of recordings. 
    /// 
    /// ⚠️ The recordings must all be fetched before. A `debug_assert` will block in case of, but won't trigger in production
    pub async fn get_releases_as_batch<'r>(conn: &mut sqlx::SqliteConnection, recordings: &'r[&'r Recording]) -> Result<HashMap<i64, (&'r &'r Recording, Vec<Release>)>, crate::Error> {
        #[cfg(debug_assertions)]
        //Self::assert_recordings_fetched(recordings); TODO: Fix borow types

        let ids = recordings.iter().map(|r| r.id).collect_vec();
        let id_string = serde_json::to_string(&ids)?;

        let joins: Vec<JoinRelation<i64, Release>> = sqlx::query_as("
            SELECT
                recordings.id as original_id,
                releases.*
            FROM
                recordings
                INNER JOIN recordings_gid_redirect ON recordings.id = recordings_gid_redirect.new_id
                INNER JOIN tracks ON recordings_gid_redirect.gid = tracks.recording
                INNER JOIN medias ON tracks.media = medias.id
                INNER JOIN releases ON medias.`release` = releases.id
            WHERE
                recordings.id IN (
                    SELECT
                        value
                    FROM
                        JSON_EACH(?)
                )
        ").bind(id_string).fetch_all(conn).await?;

        Ok(JoinCollection::from(joins).into_hashmap(recordings, |id, value| &value.id == id))
    }


}

