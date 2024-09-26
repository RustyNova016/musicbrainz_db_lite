pub mod releases;
pub mod recordings;
use artists::create_artist_tables;
use recordings::create_recordings_tables;
use releases::create_release_tables;
use sqlx::SqliteConnection;

pub mod artists;
pub mod gid_redirect_tables;

pub(super) async fn generate_musicbrainz_database(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    create_artist_tables(conn).await?;
    create_recordings_tables(conn).await?;
    create_release_tables(conn).await?;

    Ok(())
}
