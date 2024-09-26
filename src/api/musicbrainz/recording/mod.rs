pub mod fetching;
use musicbrainz_rs_nova::entity::recording::Recording as MSRecording;
use sqlx::SqliteConnection;

use crate::{
    api::SaveToDatabase,
    models::musicbrainz::{artist_credit::ArtistCredits, recording::Recording},
};

impl SaveToDatabase for MSRecording {
    type ReturnedData = Recording;

    async fn save(&self, conn: &mut SqliteConnection) -> Result<Self::ReturnedData, sqlx::Error> {
        // Save the recording
        let mut recording = Recording::from(self).upsert(conn).await?;

        // Save relations
        if let Some(artist_credits) = &self.artist_credit {
            let credits = ArtistCredits::save_api_response(conn, artist_credits).await?;
            recording.set_artist_credits(conn, credits.0).await?;
        }

        Ok(recording)
    }
}

impl From<&MSRecording> for Recording {
    fn from(value: &MSRecording) -> Self {
        Self {
            id: Default::default(),
            mbid: value.id.clone(),
            annotation: value.annotation.clone(),
            disambiguation: value.disambiguation.clone(),
            length: value.length.clone().map(|val| val as i64),
            title: value.title.clone(),
            artist_credit: None,
        }
    }
}
