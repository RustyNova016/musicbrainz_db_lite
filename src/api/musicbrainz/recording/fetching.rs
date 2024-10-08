use crate::{api::SaveToDatabase, models::musicbrainz::recording::Recording, Error};
use musicbrainz_rs_nova::{entity::recording::Recording as MSRecording, Fetch};
use sqlx::SqliteConnection;

impl Recording {
    /// Fetch a recording with all relationships. Then save to the db
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        mbid: &str,
    ) -> Result<Option<Recording>, Error> {
        let data = MSRecording::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artists()
            .with_genres()
            .with_isrcs()
            .with_ratings()
            .with_releases()
            .with_tags()
            .with_url_relations()
            .with_work_level_relations()
            .with_work_relations()
            .with_medias()
            .execute()
            .await;

        match data {
            Ok(data) => {
                let data = data.save(conn).await?;
                Self::reset_full_update_date(conn, data.id).await?;

                Self::set_redirection(conn, mbid, data.id).await?;

                Ok(Some(data))
            }
            Err(musicbrainz_rs_nova::Error::NotFound(_)) => {
                // TODO: Set deleted
                Ok(None)
            }
            Err(err) => Err(err.into()),
        }
    }
}
