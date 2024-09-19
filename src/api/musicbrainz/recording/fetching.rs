use std::sync::Arc;

use crate::{
    api::SaveToDatabase,
    models::musicbrainz::recording::{redirect::RecordingGidRedirect, Recording},
    Error,
};
use async_stream::try_stream;
use futures::{Stream, TryStream, TryStreamExt};
use musicbrainz_rs_nova::{entity::recording::Recording as MSRecording, Fetch, FetchQuery};
use welds::{connections::sqlite::SqliteClient, state::DbState, Client};

impl Recording {
    /// Create a fetch querry to fetch a recording by ID
    pub fn fetch_querry_by_id(mbid: &str) -> FetchQuery<MSRecording> {
        let mut querry = MSRecording::fetch();
        querry.id(mbid);
        querry
    }

    /// Fetch a recording with all relationships. Then save to the db
    pub async fn fetch_all_and_save(
        client: &SqliteClient,
        mbid: &str,
    ) -> Result<DbState<Recording>, Error> {
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
            .save(client)
            .await?;

        RecordingGidRedirect::assign_mbid(client, mbid, data.id).await?;

        Ok(data)
    }
}
