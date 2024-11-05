use crate::models::listenbrainz::listen::Listen;
use crate::Error;
use extend::ext;
use listenbrainz::raw::response::UserListensListen;
use listenbrainz::raw::response::UserListensPayload;
use sqlx::SqliteConnection;
use welds::connections::sqlite::SqliteClient;
use welds::state::DbState;

#[ext(name = SaveListenPayload)]
pub impl UserListensPayload {
    /// Save the listens received from the api. Handles deleting the listens, and overlapping ends.
    ///
    /// ⚠️ May not insert all the listens if the recieved count is equal to the asked count ⚠️
    ///
    /// Return the timestamp for the next fetch in sequence
    ///
    async fn save_listen_payload_in_transaction(
        &self,
        client: &SqliteClient,
        max_ts: i64,
        count: u64,
    ) -> Result<Option<i64>, Error> {
        // Check: We must have at least 1 listen
        if self.listens.len() == 0 {
            return Ok(None);
        }

        // If the count retrived is the count we asked, then there's an high change that it is a partial fetch.
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

        let mut trans = client.as_sqlx_pool().begin().await?;

        // Delete the old listens. we want to remove all the old data to not miss any removed listens
        Listen::delete_listen_range(
            &mut *trans,
            delete_range.1,
            delete_range.0,
            self.get_username()
                .expect("There should have at least one listen"),
        )
        .await?;

        Self::save_listens(&mut trans, listens).await?;

        trans.commit().await?;

        if count == self.listens.len() as u64 {
            Ok(Some(delete_range.1))
        } else {
            Ok(None)
        }
    }

    async fn save_listens(
        client: &mut SqliteConnection,
        listens: Vec<UserListensListen>,
    ) -> Result<Vec<DbState<Listen>>, Error> {
        let mut result = Vec::with_capacity(1000);

        for listen in listens {
            result.push(Listen::insert_api_listen(&mut *client, &listen).await?);
        }

        Ok(result)
    }

    fn get_username(&self) -> Option<&String> {
        self.listens.first().map(|listen| &listen.user_name)
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
