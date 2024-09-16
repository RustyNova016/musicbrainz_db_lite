use macon::Builder;
use sqlx::query_as;
use welds::connections::sqlite::SqliteClient;
use welds::prelude::DbState;
use welds::WeldsError;

use crate::Error;

use super::Listen;

#[derive(Debug, Default, Builder)]
pub struct ListenQuery {
    pub user: String,

    /// Sets wether to filter out mapped listens, unmapped listens, or ignore(default)
    pub unmapped: ListenMappingFilter,

    /// Sets whether it should fetch the user's latest listens or not.
    pub fetch_latest_listens: bool,
}

impl ListenQuery {
    pub async fn run(&self, client: &SqliteClient) -> Result<Vec<Listen>, Error> {
        if self.fetch_latest_listens {
            Listen::fetch_latest_listens_of_user(client, &self.user).await?;
        }

        Ok(query_as!(
            Listen,
            "SELECT * FROM listens WHERE listens.user = ?",
            self.user
        )
        .fetch_all(client.as_sqlx_pool())
        .await?)

        // let querr = sqlx::query_as::<Sqlite, Listen>(
        //     "SELECT
        //         *
        //     FROM
        //         listens
        //     WHERE
        //         (
        //             SELECT
        //                 COUNT(msid_mapping.recording_msid)
        //             FROM
        //                 msid_mapping
        //             WHERE
        //                 msid_mapping.recording_msid = listens.recording_msid
        //                 AND msid_mapping.user = listens.user
        //         ) = 0
        // "
        // );
    }
}

#[derive(Debug, Default)]
pub enum ListenMappingFilter {
    Mapped,
    Unmapped,
    #[default]
    Any,
}

impl Listen {
    pub async fn get_latest_listen_of_user(
        client: &SqliteClient,
        user: &str,
    ) -> Result<Option<DbState<Listen>>, WeldsError> {
        Ok(Listen::where_col(|c| c.user.equal(user))
            .limit(1)
            .run(client)
            .await?
            .pop())
    }
}
