use welds::WeldsModel;

use crate::models::musicbrainz::user::User;

#[derive(Debug, WeldsModel)]
#[welds(table = "listens")]
#[welds(BelongsTo(user, User, "id"))]
pub struct Listen {
    #[welds(primary_key)]
    pub id: i32,

    pub listened_at: i64,

    pub user: String,

    pub msid: String,
}
