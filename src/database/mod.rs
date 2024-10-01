use tables::create_listenbrainz_tables;
use tables::create_musicbrainz_tables;

use welds::connections::sqlite::SqliteClient;

use crate::Error;

pub mod client;
mod tables;
mod triggers;

pub async fn create_database(client: &SqliteClient) -> Result<(), Error> {
    let mut trans: sqlx::Transaction<'_, sqlx::Sqlite> = client.as_sqlx_pool().begin().await?;

    create_musicbrainz_tables(&mut *trans).await?;
    create_listenbrainz_tables(&mut *trans).await?;
    trans.commit().await?;

    Ok(())
}
