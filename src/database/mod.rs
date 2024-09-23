use tables::create_listenbrainz_tables;
use tables::create_musicbrainz_tables;
use triggers::create_listenbrainz_triggers;
use welds::connections::sqlite::SqliteClient;

use crate::Error;

mod tables;
mod triggers;
pub mod client;

pub async fn create_database(client: &SqliteClient) -> Result<(), Error> {
    let trans: sqlx::Transaction<'_, sqlx::Sqlite> = client.as_sqlx_pool().begin().await?;

    create_musicbrainz_tables(&client.as_sqlx_pool()).await?;
    create_listenbrainz_tables(&client.as_sqlx_pool()).await?;
    trans.commit().await?;



    create_listenbrainz_triggers(client.as_sqlx_pool()).await?;

    
    Ok(())
}
