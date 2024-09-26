use sqlx::SqlitePool;

use super::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_recordings_tables(client: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
        CREATE TABLE IF NOT EXISTS `recordings` (
            `id` INTEGER PRIMARY KEY NOT NULL, 
            `mbid` TEXT UNIQUE NOT NULL, 
            `title` TEXT NOT NULL, 
            `length` INTEGER, 
            `disambiguation` TEXT, 
            `annotation` TEXT,

            -- Foreign keys
            `artist_credit` INTEGER REFERENCES `artist_credits`(`id`)
        ) STRICT;
"#
    )
    .execute(client)
    .await?;

    sqlx::query(&generate_redirect_table("recordings"))
        .execute(client)
        .await?;

    Ok(())
}