use sqlx::SqliteConnection;

use super::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_artist_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
    `artists` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        `mbid` TEXT UNIQUE NOT NULL ,
        `name` TEXT NOT NULL,
        `sort_name` TEXT NOT NULL,
        `disambiguation` TEXT NOT NULL,
        `country` TEXT,
        `annotation` TEXT,
        
        `full_update_date` INTEGER
    ) STRICT;
     
    CREATE TABLE IF NOT EXISTS `artist_credits_item` (
        `artist_credit` INTEGER REFERENCES `artist_credits` (`id`) ON DELETE CASCADE,
        `position` INTEGER NOT NULL,
        `name` TEXT NOT NULL,
        `artist_gid` TEXT NOT NULL REFERENCES `artists_gid_redirect` (`gid`),
        `join_phrase` TEXT NOT NULL,

        PRIMARY KEY (`artist_credit`, `position`)
    ) STRICT;

    CREATE TABLE IF NOT EXISTS `artist_credits` (`id` INTEGER PRIMARY KEY AUTOINCREMENT) STRICT;
"#,
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("artists"))
        .execute(conn)
        .await?;

    Ok(())
}
