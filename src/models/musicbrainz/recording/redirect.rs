use sqlx::{Executor, Sqlite, SqlitePool};
use welds::{connections::sqlite::SqliteClient, Client, WeldsError, WeldsModel};

use crate::models::listenbrainz::msid_mapping::MsidMapping;

use super::Recording;

#[derive(Debug, WeldsModel)]
#[welds(table = "recording_gid_redirect")]
#[welds(BelongsTo(recording, Recording, "new_id"))]
#[welds(HasMany(messybrainz_mapping, MsidMapping, "recording_mbid"))]
pub struct RecordingGidRedirect {
    #[welds(primary_key)]
    pub gid: String,

    pub new_id: Option<String>,

    /// 1 means that the MBID is linked to a deleted entry, and shouldn't be persued.
    pub deleted: i64,
}

impl RecordingGidRedirect {
    /// Add an mbid in the redirect pool if it isn't in yet.
    pub async fn add_mbid(client: impl Executor<'_, Database = Sqlite>, mbid: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT OR IGNORE INTO `recording_gid_redirect` VALUES (?, NULL, 0)",
            mbid
        )
        .execute(client)
        .await?;
        Ok(())
    }

    /// Assign an mbid to a Recording's ID.
    pub async fn assign_mbid(
        client: &SqliteClient,
        original_mbid: &str,
        new_id: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO recording_gid_redirect VALUES (?, ?, 0) ON CONFLICT DO UPDATE SET `new_id` = ?",
            original_mbid,
            new_id,
            new_id
        )
        .execute(client.as_sqlx_pool())
        .await
    }

    pub async fn get_unfetched_recordings_mbids(
        client: &SqliteClient,
    ) -> Result<Vec<String>, WeldsError> {
        Ok(RecordingGidRedirect::all()
            .where_col(|c| c.new_id.equal(None))
            .where_col(|c| c.deleted.equal(0))
            .run(client)
            .await?
            .into_iter()
            .map(|r| r.into_inner().gid)
            .collect())
    }
}
