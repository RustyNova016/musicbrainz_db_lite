use welds::{connections::sqlite::SqliteClient, Client, WeldsError, WeldsModel};

#[derive(Debug, WeldsModel)]
#[welds(table = "recording_gid_redirect")]
pub struct RecordingGidRedirect {
    #[welds(primary_key)]
    pub gid: String,

    pub new_id: Option<String>,
}

impl RecordingGidRedirect {
    pub async fn add_mbid(client: &dyn Client, mbid: &str) -> Result<(), WeldsError> {
        // Check if it's already in
        if Self::where_col(|c| c.gid.equal(mbid))
            .limit(1)
            .run(client)
            .await?
            .first()
            .is_some()
        {
            return Ok(());
        }

        // It's not in. Let's insert it
        let mut data = Self::new();
        data.gid = mbid.to_string();
        data.save(client).await
    }

    pub async fn assign_mbid(
        client: &SqliteClient,
        original_mbid: &str,
        new_id: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        sqlx::query!(
            "INSERT OR REPLACE INTO recording_gid_redirect VALUES (?, ?)",
            original_mbid,
            new_id
        )
        .execute(client.as_sqlx_pool())
        .await
    }
}
