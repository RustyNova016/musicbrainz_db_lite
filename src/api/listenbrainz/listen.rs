use crate::Client;
use listenbrainz::raw::response::UserListensListen;
use welds::connections::sqlite::SqliteClient;
use welds::connections::Transaction;
use welds::prelude::DbState;
use welds::TransactStart;
use welds::WeldsError;

use crate::models::listenbrainz::listen_user_metadata::MessybrainzSubmission;
use crate::models::listenbrainz::listen_user_metadata::MessybrainzSubmissionRelation;
use crate::{
    api::SaveToDatabase,
    models::{listenbrainz::listen::Listen, musicbrainz::user::User},
};

impl SaveToDatabase for UserListensListen {
    async fn save_in_transaction<'t>(&self, client: &Transaction<'t>) -> Result<(), WeldsError> {
        // First, get the user
        let user = User::get_or_create_user(client, &self.user_name).await?;

        // Then upsert the MSID.
        let messy = MessybrainzSubmission::upsert_listen_messybrainz_data(client, self).await?;

        let mut data = Listen::new();
        data.listened_at = self.listened_at;
        data.msid = messy.msid.clone();
        data.user = user.name.clone();

        data.save(client).await?;
        Ok(())
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
            return Ok(msid_in_db);
        }

        let mut mess = MessybrainzSubmission::new();
        mess.msid = listen.recording_msid.clone();
        mess.save(client).await?;
        Ok(mess)
    }
}

