pub mod recordings;
use artists::create_artist_tables;
use recordings::create_recordings_tables;
use sqlx::SqlitePool;

pub mod artists;
pub mod gid_redirect_tables;

pub(super) async fn generate_musicbrainz_database(client: &SqlitePool) -> Result<(), sqlx::Error> {
    create_artist_tables(client).await?;
    create_recordings_tables(client).await?;

    Ok(())
}
