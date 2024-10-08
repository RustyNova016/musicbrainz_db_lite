use musicbrainz_rs_nova::entity::release::Track as MBTrack;
use sqlx::{sqlite::SqliteQueryResult, SqliteConnection};

use crate::models::musicbrainz::{artist_credit::ArtistCredits, recording::Recording, release::Track};

impl Track {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: Vec<MBTrack>,
        media_id: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        // Now convert the medias and save
        let mut converteds = Vec::new();
        for track in value {
            
            let new_track = Track {
                id: Default::default(),
                position: track.position as i64,
                title: track.title,
                gid: track.id,
                number: track.number,
                media: media_id,
                recording: None,
                length: track.length.map(|v| v as i64),
                artist_credit: None
            };

            let mut new_track = new_track.upsert(&mut *conn).await?;

            if let Some(recording) = track.recording {
                Recording::add_redirect_mbid(conn, &recording.id).await?;
                new_track.set_recording_id(conn, &recording.id).await?;
                Recording::save_api_response(conn, recording).await?;
            }

            if let Some(artist_credits) = track.artist_credit.clone() {
                let credits = ArtistCredits::save_api_response(conn, artist_credits).await?;
                new_track.set_artist_credits(conn, credits.0).await?;
            }

            converteds.push(new_track);
        }

        Ok(converteds)
    }

    pub async fn set_recording_id(&self, conn: &mut SqliteConnection, id: &str) -> Result<SqliteQueryResult, sqlx::Error>{
        sqlx::query!("UPDATE `tracks` SET recording = ? WHERE id = ?", id, self.id).execute(conn).await
    }

    /// Associate a track gid to a recording gid
    pub async fn set_recording_id_from_gid(conn: &mut SqliteConnection, recording_id: &str, track_id: &str) -> Result<SqliteQueryResult, sqlx::Error>{
        sqlx::query!("UPDATE `tracks` SET recording = ? WHERE gid = ?", recording_id, track_id).execute(conn).await
    }
}
