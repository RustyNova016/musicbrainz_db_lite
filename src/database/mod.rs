use tables::create_listenbrainz_tables;
use tables::create_musicbrainz_tables;
use triggers::create_listenbrainz_triggers;
use welds::connections::sqlite::SqliteClient;
use welds::TransactStart;
use welds::WeldsError;

mod triggers;
mod tables;

pub async fn create_database(client: &SqliteClient) -> Result<(), WeldsError> {
    let trans = client.begin().await?;

    create_musicbrainz_tables(&trans).await?;
    create_listenbrainz_tables(&trans).await?;

    create_listenbrainz_triggers(&trans).await?;

    trans.commit().await?;
    Ok(())
}