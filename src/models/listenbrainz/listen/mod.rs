pub mod deletes;
pub mod selects;
use welds::WeldsModel;

use crate::models::musicbrainz::user::User;

use super::{listen_user_metadata::MessybrainzSubmission, msid_mapping::MsidMapping};

#[derive(Debug, WeldsModel, sqlx::FromRow)]
#[welds(table = "listens")]
#[welds(BelongsTo(user, User, "user"))]
#[welds(BelongsTo(messybrainz_submition, MessybrainzSubmission, "recording_msid"))]
#[welds(HasMany(msib_mapping, MsidMapping, "recording_msid"))]
pub struct Listen {
    #[welds(primary_key)]
    pub id: i64,

    pub listened_at: i64,

    pub user: String,

    pub recording_msid: String,

    pub data: Option<String>,
}
