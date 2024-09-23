pub mod fetching;
use crate::models::listenbrainz::msid_mapping::MsidMapping;
use crate::models::musicbrainz::recording::redirect::RecordingGidRedirect;
use crate::Error;
use listenbrainz::raw::response::UserListensListen;
use sqlx::{Sqlite, Transaction};
use welds::prelude::DbState;

use crate::models::listenbrainz::listen_user_metadata::MessybrainzSubmission;
use crate::models::{listenbrainz::listen::Listen, musicbrainz::user::User};

impl Listen {
    pub async fn insert_api_listen(
        client: &mut Transaction<'_, Sqlite>,
        listen: &UserListensListen,
    ) -> Result<DbState<Listen>, Error> {

        // First, get the user
        User::insert_or_ignore(&mut **client, &listen.user_name).await?;

        // Then upsert the MSID.
        MessybrainzSubmission::from(listen)
            .insert_or_ignore(&mut **client)
            .await?;

        // Set the mapping if available
        if let Some(mapping) = &listen.track_metadata.mbid_mapping {
            // First insert the mbid
            RecordingGidRedirect::add_mbid(&mut **client, &mapping.recording_mbid).await?;

            let user = User::find_by_name(&mut **client, &listen.user_name)
                .await?
                .expect("The user shall be inserted");

            MsidMapping::set_user_mapping(
                &mut **client,
                user.id,
                listen.recording_msid.clone(),
                mapping.recording_mbid.clone(),
            )
            .await?;
        }

        let data = serde_json::to_string(&listen.track_metadata.additional_info)
            .expect("Crashing from serializing a serde::Value isn't possible");

        let listen_db = sqlx::query_as!(
            Listen,
            "INSERT INTO listens VALUES (NULL, ?, ?, ?, ?) RETURNING *",
            listen.listened_at,
            listen.user_name,
            listen.recording_msid,
            data
        )
        .fetch_one(&mut **client)
        .await?;

        Ok(DbState::new_uncreated(listen_db))
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
