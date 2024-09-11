use super::listen_user_metadata::MessybrainzSubmission;
use crate::models::musicbrainz::user::User;
use welds::{state::DbState, Client, WeldsError, WeldsModel};

#[derive(Debug, WeldsModel)]
#[welds(table = "msid_mapping")]
//#[welds(BelongsTo(recording_mbid, User, "id"))]
#[welds(BelongsTo(recording_msid, MessybrainzSubmission, "msid"))]
#[welds(BelongsTo(user, User, "id"))]
pub struct MsidMapping {
    pub recording_mbid: String,

    pub recording_msid: String,

    pub user: i32,
}

impl MsidMapping {
    /// Finds a mapping by its user's ID, and msid
    pub async fn find_by_user_msid(
        client: &dyn Client,
        user_id: i32,
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

    /// Set the MBID mapping for an msid for user
    pub async fn set_user_mapping(
        client: &dyn Client,
        user_id: i32,
        msid: String,
        mbid: String,
    ) -> Result<(), WeldsError> {
        if let Some(mut mapping) = Self::find_by_user_msid(client, user_id, &msid).await? {
            if mapping.recording_mbid != mbid {
                mapping.recording_mbid = mbid;
                return mapping.save(client).await;
            }
            return Ok(());
        }

        let mut mapping = MsidMapping::new();
        mapping.user = user_id;
        mapping.recording_msid = msid;
        mapping.recording_mbid = mbid;
        mapping.save(client).await
    }
}
