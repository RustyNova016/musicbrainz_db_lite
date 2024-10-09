
use sqlx::SqliteConnection;

use super::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_recordings_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"
        CREATE TABLE IF NOT EXISTS `recordings` (
            `id` INTEGER PRIMARY KEY NOT NULL, 
            `mbid` TEXT UNIQUE NOT NULL, 
            `title` TEXT NOT NULL, 
            `video` INTEGER CHECK(`video` = 0 OR `video` = 1),
            `length` INTEGER, 
            `disambiguation` TEXT,  
            `annotation` TEXT,
            `first_release_date`,

            `full_update_date` INTEGER CHECK(`full_update_date` > 0),

            -- Foreign keys
            `artist_credit` INTEGER REFERENCES `artist_credits`(`id`)
        ) STRICT;
"#
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("recordings"))
        .execute(conn)
        .await?;

    Ok(())
}
