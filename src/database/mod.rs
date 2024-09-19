use tables::create_listenbrainz_tables;
use tables::create_musicbrainz_tables;
use triggers::create_listenbrainz_triggers;
use welds::connections::sqlite::SqliteClient;
use welds::TransactStart;
use welds::WeldsError;

use crate::Error;

mod tables;
mod triggers;
pub mod client;

pub async fn create_database(client: &SqliteClient) -> Result<(), Error> {
    let trans = client.begin().await?;

    create_musicbrainz_tables(&trans).await?;
    create_listenbrainz_tables(&trans).await?;
    trans.commit().await?;



    create_listenbrainz_triggers(client.as_sqlx_pool()).await?;

    
    Ok(())
}
