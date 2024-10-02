use crate::{api::SaveToDatabase, models::musicbrainz::release::Release, Error};
use musicbrainz_rs_nova::{entity::release::Release as MBRelease, Fetch};
use sqlx::SqliteConnection;

impl Release {
    pub async fn fetch_and_save(conn: &mut SqliteConnection, mbid: &str) -> Result<Self, Error> {
        let data = MBRelease::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artist_credits()
            .with_artist_relations()
            .with_artists()
            .with_genres()
            .with_labels()
            .with_ratings()
            .with_recording_level_relations()
            .with_recordings()
            .with_release_groups()
            .with_tags()
            .with_url_relations()
            .with_work_level_relations()
            .with_work_relations()
            .execute()
            .await?
            .save(conn)
            .await?;

        Self::reset_full_update_date(conn, data.id).await?;

        Self::set_redirection(conn, mbid, data.id).await?;

        Ok(data)
    }
}

impl SaveToDatabase for MBRelease {
    type ReturnedData = Release;

    async fn save(self, conn: &mut SqliteConnection) -> Result<Self::ReturnedData, sqlx::Error> {
        Release::save_api_response_recursive(conn, self).await
    }
}
