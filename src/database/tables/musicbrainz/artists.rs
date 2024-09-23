use sqlx::{query as query_mac, SqlitePool};

use super::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_artist_tables(client: &SqlitePool) -> Result<(), sqlx::Error> {
    query_mac!(r#"CREATE TABLE IF NOT EXISTS
    `artists` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        `mbid` TEXT NOT NULL,
        `name` TEXT NOT NULL,
        `sort_name` TEXT NOT NULL,
        `disambiguation` TEXT NOT NULL,
        `rating` TEXT,
        `country` TEXT,
        `annotation` TEXT
    ) STRICT;
"#).execute(client).await?;

    sqlx::query(&generate_redirect_table("artists")).execute(client).await?;

    Ok(())
}