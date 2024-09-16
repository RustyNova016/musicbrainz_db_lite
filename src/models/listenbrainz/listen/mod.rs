pub mod selects;
use welds::WeldsModel;

use crate::models::musicbrainz::user::User;

use super::listen_user_metadata::MessybrainzSubmission;

#[derive(Debug, WeldsModel, sqlx::FromRow)]
#[welds(table = "listens")]
#[welds(BelongsTo(user, User, "id"))]
#[welds(BelongsTo(recording_msid, MessybrainzSubmission, "gid"))]
pub struct Listen {
    #[welds(primary_key)]
    pub id: i64,

    pub listened_at: i64,

    pub user: String,

    pub recording_msid: String,

    pub data: Option<String>,
}
