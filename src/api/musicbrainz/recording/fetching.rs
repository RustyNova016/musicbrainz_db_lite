use crate::{
    api::{SaveToDatabase, SaveToDatabaseOld},
    models::musicbrainz::recording::{redirect::RecordingGidRedirect, Recording},
    Error,
};
use musicbrainz_rs_nova::{entity::recording::Recording as MSRecording, Fetch, FetchQuery};
use sqlx::SqliteConnection;

impl Recording {
    /// Fetch a recording with all relationships. Then save to the db
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        mbid: &str,
    ) -> Result<Recording, Error> {
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
            .await?
            .save(conn)
            .await?;

            Self::set_redirection(conn, mbid, data.id).await?;

        Ok(data)
    }
}
