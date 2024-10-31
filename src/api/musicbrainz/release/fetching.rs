use crate::{
    api::SaveToDatabase,
    models::musicbrainz::release::{Release, Track},
    Error,
};
use musicbrainz_rs_nova::{entity::release::Release as MBRelease, Fetch};
use sqlx::SqliteConnection;

impl Release {
    pub async fn fetch_and_save(
        conn: &mut SqliteConnection,
        mbid: &str,
    ) -> Result<Option<Self>, Error> {
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

impl SaveToDatabase for MBRelease {
    type ReturnedData = Release;

    async fn save(self, conn: &mut SqliteConnection) -> Result<Self::ReturnedData, crate::Error> {
        Release::save_api_response_recursive(conn, self).await
    }
}

impl Track {
    pub async fn refetch(&self, _conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use musicbrainz_db_lite_schema::create_database;

    use crate::database::client::DBClient;
    use crate::models::musicbrainz::release::Release;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_release() {
        let client = DBClient::connect_in_memory().await.unwrap();
        let mut conn  = &mut *client.connection.acquire().await.unwrap();
        create_database(conn).await.unwrap();
        let recording = Release::get_or_fetch(
            &mut conn,
            "daf6e333-b491-490a-9444-8888cb08b141",
        )
        .await
        .unwrap();

        assert!(recording.is_some_and(|r| r.full_update_date.is_some()))
    }
}
