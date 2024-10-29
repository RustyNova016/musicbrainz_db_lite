use musicbrainz_rs_nova::{entity::label::Label as MBLabel, Fetch};

use crate::{api::SaveToDatabase, models::musicbrainz::label::Label};

impl Label {
    pub async fn fetch_and_save(conn: &mut sqlx::SqliteConnection, mbid: &str) -> Result<Option<Self>, crate::Error> {
        let data = MBLabel::fetch()
            .id(mbid)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_genres()
            .with_label_relations()
            .with_medias()
            .with_ratings()
            .with_recording_relations()
            .with_release_relations()
            .with_releases()
            .with_tags()
            .with_url_relations()
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

impl SaveToDatabase for MBLabel {
    type ReturnedData = Label;

    async fn save(self, conn: &mut sqlx::SqliteConnection) -> Result<Self::ReturnedData, crate::Error> {
        Label::save_api_response_recursive(conn, self).await
    }
}