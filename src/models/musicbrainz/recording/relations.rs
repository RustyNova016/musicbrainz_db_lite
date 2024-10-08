use sqlx::SqliteConnection;

use crate::{models::musicbrainz::release::Release, Error};

use super::Recording;

impl Recording {
    /// Get the releases of the recording, and fetch them if necessary.
     pub async fn get_releases_or_fetch(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Release>, crate::Error> {
        // First, make sure all the releases of the recording are in the database
        let id = self.get_or_fetch_as_complete(conn).await?.id;

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
                
                 ).bind(id)
                 .fetch_all(conn)
                 .await?)
    } 


}
