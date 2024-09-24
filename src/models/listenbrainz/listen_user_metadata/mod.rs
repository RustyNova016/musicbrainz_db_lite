use macon::Builder;
use sqlx::{Executor, Sqlite};
use welds::{state::DbState, Client, WeldsError, WeldsModel};

use super::{listen::Listen, msid_mapping::MsidMapping};

/// The fingerprint that identify a listened recording. This is the data scrobblers send to LB to tell that the user listened to a recording
#[derive(Debug, WeldsModel, Builder)]
#[builder(Default=!)]
#[derive()]
#[welds(table = "messybrainz_submission")]
#[welds(HasMany(listen, Listen, "recording_msid"))]
#[welds(HasMany(mapping, MsidMapping, "recording_msid"))]
pub struct MessybrainzSubmission {
    #[welds(primary_key)]
    pub id: i32,

    #[builder(Default=!)]
    pub msid: String,

    #[builder(Default=!)]
    pub recording: String,

    #[builder(Default=!)]
    pub artist_credit: String,

    #[builder(Default=!)]
    pub release: Option<String>,

    #[builder(Default=!)]
    pub track_number: Option<String>,

    #[builder(Default=!)]
    pub duration: Option<i32>,
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

    ///
    pub async fn insert_or_ignore(
        &self,
        client: impl Executor<'_, Database = Sqlite>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT OR IGNORE INTO `messybrainz_submission` VALUES (NULL, ?, ?, ?, ?, ?, ?)",
            self.msid,
            self.recording,
            self.artist_credit,
            self.release,
            self.track_number,
            self.duration
        )
        .execute(client)
        .await?;
        Ok(())
    }
}
