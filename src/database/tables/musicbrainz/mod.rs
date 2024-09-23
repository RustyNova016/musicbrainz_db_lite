use artists::create_artist_tables;
use sqlx::SqlitePool;

pub mod gid_redirect_tables;
pub mod artists;

pub(super) async fn generate_musicbrainz_database(client: &SqlitePool) -> Result<(), sqlx::Error> {
    create_artist_tables(client).await?;

    Ok(())
}