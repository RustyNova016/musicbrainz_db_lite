use sqlx::SqliteConnection;

use crate::models::{
    listenbrainz::msid_mapping::MsidMapping,
    musicbrainz::{recording::Recording, user::User},
};

use super::Listen;

impl Listen {
    pub async fn get_recording_or_fetch(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Option<Recording>, crate::Error> {
        let user = User::find_by_name(conn, &self.user)
            .await?
            .expect("User should be in due to foreign keys");

        let recording_mbid =
            MsidMapping::find_by_user_msid2(conn, user.id, &self.recording_msid).await?;

        match recording_mbid {
            None => Ok(None),
            Some(mapping) => Recording::get_or_fetch(conn, &mapping.recording_mbid).await,
        }
    }
}
