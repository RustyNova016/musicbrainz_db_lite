mod fetching;
use crate::models::listenbrainz::msid_mapping::MsidMapping;
use crate::models::musicbrainz::recording::redirect::RecordingGidRedirect;
use crate::Client;
use listenbrainz::raw::response::UserListensListen;
use welds::prelude::DbState;
use welds::WeldsError;

use crate::models::listenbrainz::listen_user_metadata::MessybrainzSubmission;
use crate::{
    api::SaveToDatabase,
    models::{listenbrainz::listen::Listen, musicbrainz::user::User},
};

impl SaveToDatabase for UserListensListen {
    async fn save_in_transaction(&self, client: &dyn Client) -> Result<(), WeldsError> {
        // First, get the user
        let user = User::get_or_create_user(client, &self.user_name).await?;

        // Then upsert the MSID.
        MessybrainzSubmission::upsert_listen_messybrainz_data(client, self).await?;

        // Set the mapping if available
        if let Some(mapping) = &self.track_metadata.mbid_mapping {
            // First insert the mbid
            RecordingGidRedirect::add_mbid(client, &mapping.recording_mbid).await?;

            MsidMapping::set_user_mapping(
                client,
                user.id,
                self.recording_msid.clone(),
                mapping.recording_mbid.clone(),
            )
            .await?;
        }

        let mut data = DbState::new_uncreated(Listen::from(self));
        data.save(client).await?;
        Ok(())
    }
}

impl From<&UserListensListen> for Listen {
    fn from(value: &UserListensListen) -> Self {
        Self {
            id: Default::default(),
            listened_at: value.listened_at,
            user: value.user_name.clone(),
            recording_msid: value.recording_msid.clone(),
            data: Some(
                serde_json::to_string(&value.track_metadata.additional_info)
                    .expect("Crashing from serializing a serde::Value isn't possible"),
            ),
        }
    }
}

impl MessybrainzSubmission {
    /// Create or update a messybrainz submition from a listen
    pub async fn upsert_listen_messybrainz_data(
        client: &dyn Client,
        listen: &UserListensListen,
    ) -> Result<DbState<MessybrainzSubmission>, WeldsError> {
        if let Some(msid_in_db) =
            MessybrainzSubmission::find_by_msid(client, &listen.recording_msid).await?
        {
            // Messybrainz data is static. So skip the update!
            println!("{:#?}", msid_in_db);
            return Ok(msid_in_db);
        }

        let mut data = DbState::new_uncreated(MessybrainzSubmission::from(listen));
        data.save(client).await?;
        println!("{:#?}", data);
        Ok(data)
    }
}

impl From<&UserListensListen> for MessybrainzSubmission {
    fn from(value: &UserListensListen) -> Self {
        Self {
            id: Default::default(),
            msid: value.recording_msid.clone(),
            recording: value.track_metadata.track_name.clone(),
            artist_credit: value.track_metadata.artist_name.clone(),
            release: value.track_metadata.release_name.clone(),
            track_number: None, // TODO: Find where is it stored in the json... If it even is stored...
            duration: None, //TODO: Get the duration from additiona info or ditch it from the schema?
        }
    }
}
