use musicbrainz_rs_nova::entity::artist::Artist as MBArtist;
use musicbrainz_rs_nova::Fetch;
use sqlx::SqliteConnection;
use tracing::debug;

use crate::api::SaveToDatabase;
use crate::models::musicbrainz::artist::Artist;
use crate::Error;

impl Artist {
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
        debug!(mbid = mbid);

        let data = MBArtist::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_event_relations()
            .with_genres()
            .with_rating()
            .with_recording_relations()
            .with_recordings()
            .with_release_groups()
            .with_release_relations()
            .with_releases()
            .with_releases_and_discids()
            .with_series_relations()
            .with_tags()
            .with_url_relations()
            .with_work_relations()
            .with_works()
            .with_medias()
            .execute()
            .await;

        match data {
            Ok(data) => {
                let mut data = data.save(conn).await?;
                data.reset_full_update_date(conn).await?;

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
