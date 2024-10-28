pub mod relations;
use super::listen_user_metadata::MessybrainzSubmission;
use crate::models::musicbrainz::{recording::redirect::RecordingGidRedirect, user::User};
use sqlx::{Executor, Sqlite, SqliteConnection};
use welds::{state::DbState, Client, WeldsError, WeldsModel};

#[derive(Debug, WeldsModel, Clone)]
#[welds(table = "msid_mapping")]
#[welds(BelongsTo(recording_mbid, RecordingGidRedirect, "recording_mbid"))]
#[welds(BelongsTo(messybrainz_submission, MessybrainzSubmission, "recording_msid"))]
#[welds(BelongsTo(user, User, "user"))]
pub struct MsidMapping {
    #[welds(primary_key)]
    pub id: i64,

    pub recording_mbid: String,

    pub recording_msid: String,

    pub release_mbid: Option<String>,

    pub user: i64,
}

impl MsidMapping {
    /// Finds a mapping by its user's ID, and msid
    pub async fn find_by_user_msid(
        client: &dyn Client,
        user_id: i64,
        msid: &str,
    ) -> Result<Option<DbState<Self>>, WeldsError> {
        Ok(MsidMapping::all()
            .where_col(|c| c.user.equal(user_id))
            .where_col(|c| c.recording_msid.equal(msid))
            .limit(1)
            .run(client)
            .await?
            .pop())
    }

    pub async fn find_by_user_msid2(
        conn: &mut SqliteConnection,
        user_id: i64,
        msid: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM msid_mapping WHERE recording_msid = ? AND user = ?;",
            msid,
            user_id
        )
        .fetch_optional(conn)
        .await
    }

    /// Set the MBID mapping for an msid for user
    pub async fn set_user_mapping(
        client: impl Executor<'_, Database = Sqlite>,
        user_id: i64,
        msid: String,
        mbid: String,
    ) -> Result<(), sqlx::Error> {
        //println!("mapping {} to {}", msid, mbid);

        

        sqlx::query!("INSERT INTO `msid_mapping` VALUES (NULL, ?, ?, ?, NULL) ON CONFLICT DO UPDATE SET `recording_mbid` = ?", msid, mbid, user_id, mbid).execute(client).await?;
        Ok(())
    }
}
