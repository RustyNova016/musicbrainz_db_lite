use musicbrainz_rs_nova::entity::artist::Artist as MBArtist;
use musicbrainz_rs_nova::Fetch;
use sqlx::SqliteConnection;

use crate::api::SaveToDatabase;
use crate::models::musicbrainz::artist::Artist;
use crate::Error;

impl Artist {
    pub async fn fetch_and_save(conn: &mut SqliteConnection, mbid: &str) -> Result<Artist, Error> {
        let artist = MBArtist::fetch()
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
            .await?
            .save(conn)
            .await?;

        Artist::set_redirection(conn, mbid, artist.id).await?;

        Ok(artist)
    }
}

