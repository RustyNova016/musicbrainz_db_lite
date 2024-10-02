pub mod fetching;
use crate::{
    api::SaveToDatabase,
    models::musicbrainz::{
        artist_credit::ArtistCredits,
        recording::Recording,
        release::{Release, Track},
    },
};
use musicbrainz_rs_nova::entity::recording::Recording as MBRecording;
use musicbrainz_rs_nova::entity::release::Release as MBRelease;
use sqlx::SqliteConnection;

impl Recording {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBRecording,
    ) -> Result<Self, sqlx::Error> {
        Recording::add_redirect_mbid(conn, &value.id).await?;
        Recording::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Recording::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    pub fn merge_api_data(self, new: MBRecording) -> Self {
        Self {
            id: self.id,
            annotation: new.annotation.or(self.annotation),
            mbid: new.id,
            artist_credit: self.artist_credit,
            disambiguation: new.disambiguation.or(self.disambiguation),
            length: new.length.map(|n| n as i64).or(self.length),
            title: new.title,
            full_update_date: self.full_update_date
        }
    }

    /// Save a recording from the api data. It also save the relationships.
    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBRecording,
    ) -> Result<Self, sqlx::Error> {
        // Save the recording
        let mut recording = Recording::save_api_response(&mut *conn, value.clone()).await?;

        // Save relations
        if let Some(artist_credits) = value.artist_credit.clone() {
            let credits = ArtistCredits::save_api_response(conn, artist_credits).await?;
            recording.set_artist_credits(conn, credits.0).await?;
        }

        if let Some(values) = value.releases.clone() {
            for value in values {
                let gids = get_track_gids_from_release(value.clone());
                Release::save_api_response_recursive(conn, value).await?;

                for gid in gids {
                    Track::set_recording_id_from_gid(conn, &recording.mbid, &gid).await?;
                }
            }
        }

        Ok(recording)
    }
}

impl SaveToDatabase for MBRecording {
    type ReturnedData = Recording;

    async fn save(self, conn: &mut SqliteConnection) -> Result<Self::ReturnedData, sqlx::Error> {
        Recording::save_api_response_recursive(conn, self).await
    }
}

fn get_track_gids_from_release(release: MBRelease) -> Vec<String> {
    let mut gids = Vec::new();

    for media in release.media.unwrap_or_else(Vec::new) {
        for track in media.tracks.unwrap_or_else(Vec::new) {
            gids.push(track.id);
        }
    }

    gids
}
