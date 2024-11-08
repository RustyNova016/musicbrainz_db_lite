pub mod deletes;
pub mod querry_builder;
pub mod relations;
pub mod selects;
use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use serde::Serialize;
use welds::WeldsModel;

use crate::models::musicbrainz::user::User;
use crate::models::shared_traits::RowId;

use super::{listen_user_metadata::MessybrainzSubmission, msid_mapping::MsidMapping};

#[derive(Debug, WeldsModel, PartialEq, Eq, Clone, sqlx::FromRow, Deserialize, Serialize)]
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

impl Listen {
    pub fn listened_at_as_datetime(&self) -> DateTime<Utc> {
        // unwrap() is best combined with time zone types where the mapping can never fail like Utc.
        Utc.timestamp_opt(self.listened_at, 0).unwrap()
    }
}

impl RowId for Listen {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}
