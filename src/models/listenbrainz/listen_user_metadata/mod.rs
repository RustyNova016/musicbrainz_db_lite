use macon::Builder;
use sqlx::{Executor, Sqlite};

/// The fingerprint that identify a listened recording. This is the data scrobblers send to LB to tell that the user listened to a recording
#[derive(Debug, Builder)]
#[builder(Default=!)]
pub struct MessybrainzSubmission {
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
