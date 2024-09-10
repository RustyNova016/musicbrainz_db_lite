use welds::{state::DbState, Client, WeldsError, WeldsModel};

use crate::models::musicbrainz::user::User;

use super::listen::Listen;


/// The fingerprint that identify a listened recording. This is the data scrobblers send to LB to tell that the user listened to a recording
#[derive(Debug, WeldsModel)]
#[welds(table = "messybrainz_submission")]
#[welds(BelongsTo(user, User, "id"))]
#[welds(HasMany(listens, Listen, "id"))]
pub struct MessybrainzSubmission {
    #[welds(primary_key)]
    pub id: i32,

    pub msid: String,
}

impl MessybrainzSubmission {
    /// Find an [`MessybrainzSubmission`] by its MSID
    pub async fn find_by_msid(
        client: &dyn Client,
        msid: &str,
    ) -> Result<Option<DbState<Self>>, WeldsError> {
        Ok(Self::all()
            .where_col(|c| c.msid.equal(msid))
            .limit(1)
            .run(client)
            .await?
            .pop())
    }
}
