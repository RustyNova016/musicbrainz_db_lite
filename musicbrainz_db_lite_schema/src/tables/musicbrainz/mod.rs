use sqlx::SqliteConnection;

use artists::create_artist_tables;
use label::create_label_tables;
use recordings::create_recordings_tables;
use relations::create_relation_tables;
use releases::create_release_tables;
use work::create_work_tables;

pub mod artists;
pub mod gid_redirect_tables;
pub mod label;
pub mod recordings;
pub mod relations;
pub mod releases;
pub mod work;

pub(super) async fn generate_musicbrainz_database(
    conn: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    create_artist_tables(conn).await?;
    create_recordings_tables(conn).await?;
    create_release_tables(conn).await?;
    create_label_tables(conn).await?;
    create_work_tables(conn).await?;

    create_relation_tables(conn).await?;

    Ok(())
}
