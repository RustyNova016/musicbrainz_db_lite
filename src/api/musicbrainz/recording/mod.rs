pub mod fetching;
use musicbrainz_rs_nova::entity::recording::Recording as MSRecording;
use welds::state::DbState;

use crate::{api::SaveToDatabase, models::musicbrainz::recording::Recording};

impl SaveToDatabase for MSRecording {
    type ReturnedData = DbState<Recording>;

    async fn save(
        &self,
        client: &dyn welds::Client,
    ) -> Result<Self::ReturnedData, welds::WeldsError> {
        // Save the recording
        let recording = Recording::find_by_mbid(client, &self.id)
            .await?
            .unwrap_or_else(Recording::new);
        let mut recording = Recording::replace(recording, Recording::from(self));

        recording.save(client).await?;
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
        }
    }
}
