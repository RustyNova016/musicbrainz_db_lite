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

#[cfg(test)]
mod tests {
    use musicbrainz_db_lite_schema::create_database;

    use crate::database::client::DBClient;
    use crate::models::musicbrainz::recording::Recording;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_recording() {
        let client = DBClient::connect_in_memory().await.unwrap();
        let mut conn  = &mut *client.connection.acquire().await.unwrap();
        create_database(conn).await.unwrap();

        let recording = Recording::get_or_fetch(conn,"5fed738b-1e5c-4a1b-9f66-b3fd15dbc8ef")
        .await
        .unwrap();

        assert!(recording.is_some_and(|r| r.full_update_date.is_some()))
    }
}
