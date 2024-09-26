use crate::{
    api::{SaveToDatabase, SaveToDatabaseOld},
    models::musicbrainz::recording::{redirect::RecordingGidRedirect, Recording},
    Error,
};
use musicbrainz_rs_nova::{entity::recording::Recording as MSRecording, Fetch, FetchQuery};
use sqlx::SqliteConnection;
use welds::{connections::sqlite::SqliteClient, state::DbState};

impl Recording {
    /// Create a fetch querry to fetch a recording by ID
    pub fn fetch_querry_by_id(mbid: &str) -> FetchQuery<MSRecording> {
        let mut querry = MSRecording::fetch();
        querry.id(mbid);
        querry
    }

    /// Fetch a recording with all relationships. Then save to the db
    pub async fn fetch_all_and_save(
        conn: &mut SqliteConnection,
        mbid: &str,
    ) -> Result<Recording, Error> {
        let data = Self::fetch_querry_by_id(mbid)
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
            .execute()
            .await?
            .save(conn)
            .await?;

        RecordingGidRedirect::assign_mbid(conn, mbid, data.id).await?;

        Ok(data)
    }
}
