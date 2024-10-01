use macon::Builder;
use sqlx::query_scalar;
use welds::connections::sqlite::SqliteClient;
use welds::prelude::DbState;
use welds::WeldsError;

use crate::models::musicbrainz::user::User;

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
    // pub async fn run(&self, client: &SqliteClient) -> Result<Vec<Listen>, Error> {
    //     if self.fetch_latest_listens {
    //         Listen::fetch_latest_listens_of_user(client, &self.user).await?;
    //     }

    //     // Ok(query_as!(
    //     //     Listen,
    //     //     "SELECT * FROM listens WHERE listens.user = ?",
    //     //     self.user
    //     // )
    //     // .fetch_all(client.as_sqlx_pool())
    //     // .await?)

    //     // let querr = sqlx::query_as::<Sqlite, Listen>(
    //     //     "SELECT
    //     //         *
    //     //     FROM
    //     //         listens
    //     //     WHERE
    //     //         (
    //     //             SELECT
    //     //                 COUNT(msid_mapping.recording_msid)
    //     //             FROM
    //     //                 msid_mapping
    //     //             WHERE
    //     //                 msid_mapping.recording_msid = listens.recording_msid
    //     //                 AND msid_mapping.user = listens.user
    //     //         ) = 0
    //     // "
    //     // );
    // }
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

    pub async fn get_unfetched_recordings_of_user(
        client: &SqliteClient,
        user: &User,
    ) -> Result<Vec<String>, sqlx::Error> {
        /*         Ok(Listen::all().where_col(|c| c.user.equal(user))
        .map_query(|r| r.msib_mapping)
        .where_col(|c| c.user.equal(1))
        .map_query(|r| r.recording_mbid)
        .where_col(|c| c.deleted.equal(0))
        .where_col(|c| c.new_id.equal(None))
        .run(client)
        .await?
        .into_iter().map(|r| r.into_inner().gid).collect()) */

        query_scalar!(r#"
            SELECT DISTINCT
                recordings_gid_redirect."gid"
            FROM
                users
                INNER JOIN listens ON users.name = listens.user
                INNER JOIN messybrainz_submission ON listens.recording_msid = messybrainz_submission.msid
                INNER JOIN msid_mapping ON messybrainz_submission.msid = msid_mapping.recording_msid
                INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
            WHERE
                recordings_gid_redirect.deleted = 0
                AND recordings_gid_redirect.new_id IS NULL
                AND msid_mapping.user = users.id
                AND users.id = ?
                "#,
            user.id
        )
        .fetch_all(client.as_sqlx_pool())
        .await
    }

    pub async fn get_recordings_of_user(
        client: &SqliteClient,
        user: &User,
    ) -> Result<Vec<String>, sqlx::Error> {
        query_scalar!(r#"
            SELECT DISTINCT
                recordings_gid_redirect."gid"
            FROM
                users
                INNER JOIN listens ON users.name = listens.user
                INNER JOIN messybrainz_submission ON listens.recording_msid = messybrainz_submission.msid
                INNER JOIN msid_mapping ON messybrainz_submission.msid = msid_mapping.recording_msid
                INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
            WHERE
                recordings_gid_redirect.deleted = 0
                AND msid_mapping.user = users.id
                AND users.id = ?
                "#,
            user.id
        )
        .fetch_all(client.as_sqlx_pool())
        .await
    }
}
