use crate::api::SaveToDatabase;
use extend::ext;
use listenbrainz::raw::response::UserListensListen;
use listenbrainz::raw::response::UserListensPayload;
use welds::Client;

impl SaveToDatabase for Vec<UserListensListen> {
    async fn save_in_transaction(&self, client: &dyn Client) -> Result<(), welds::WeldsError> {
        for listen in self {
            listen.save_in_transaction(client).await?; //TODO: Multithread it
        }

        Ok(())
    }
}

#[ext(name = SaveListenPayload)]
pub impl UserListensPayload {
    /// Save the listens received from the api. Handles deleting the listens, and overlapping ends.
    ///
    /// ⚠️ May not insert all the listens if the recieved count is equal to the asked count ⚠️
    ///
    /// Return the timestamp for the next fetch in sequence
    async fn save_listen_payload_in_transaction(
        &self,
        client: &dyn Client,
        max_ts: i64,
        count: u64,
    ) -> Result<Option<i64>, welds::WeldsError> {
        // If the  count retrived is the count we asked, then there's an high change that it is a partial fetch.
        let delete_range = if count == self.listens.len() as u64 {
            get_deletion_range_for_part(self, max_ts)
        } else {
            get_deletion_range_for_limit(self, max_ts)
        };

        // Trim the listens we aren't inserting
        let listens = self
            .listens
            .iter()
            .filter(|l| {
                let var_name = l.listened_at < delete_range.0;
                let var_namet = l.listened_at > delete_range.1;
                var_name && var_namet
            })
            .cloned()
            .collect::<Vec<_>>();

        listens.save_in_transaction(client).await?;

        if count == self.listens.len() as u64 {
            Ok(Some(delete_range.1))
        } else {
            Ok(None)
        }
    }
}

/// Gives the range of timestamps to delete (inclusive) if we fetched up to the first listen of the user.
/// Returns a tuple of `(higher bound, lower bound)`
fn get_deletion_range_for_limit(res: &UserListensPayload, max_ts: i64) -> (i64, i64) {
    (
        max_ts - 1,
        res.listens.iter().map(|l| l.listened_at).min().unwrap_or(0),
    )
}

/// Gives the range of timestamps to delete (inclusive) if we only fetched a part of the listens
/// Returns a tuple of `(higher bound, lower bound)`
fn get_deletion_range_for_part(res: &UserListensPayload, max_ts: i64) -> (i64, i64) {
    (
        max_ts - 1,
        res.listens.iter().map(|l| l.listened_at).min().unwrap_or(0) + 1,
    )
}
