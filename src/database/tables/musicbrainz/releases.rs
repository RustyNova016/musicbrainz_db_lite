use sqlx::SqliteConnection;

use super::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_release_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS
            `releases` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `mbid` TEXT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `date` INTEGER,
                `country` TEXT,
                `quality` TEXT,
                `status` TEXT,
                `barcode` TEXT,
                `disambiguation` TEXT,
                `packaging` TEXT,
                `annotation` TEXT,

                -- Foreign Keys
                `artist_credit` INTEGER REFERENCES `artist_credits` (`id`)
            ) STRICT;

        CREATE TABLE IF NOT EXISTS
            `medias` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `title` TEXT,
                `position` INTEGER,
                `disc_count` INTEGER,
                `format` TEXT,

                -- Foreign Keys
                `release` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;

        CREATE UNIQUE INDEX IF NOT EXISTS `idx_medias` ON `medias` (`release`, `position`);

        CREATE TABLE IF NOT EXISTS
            `tracks` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `gid` TEXT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `number` TEXT NOT NULL,
                `position` INTEGER NOT NULL,

                -- Foreign Keys
                `media` INTEGER NOT NULL REFERENCES `medias` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `recording` TEXT REFERENCES `recordings_gid_redirect` (`gid`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
             
        CREATE UNIQUE INDEX IF NOT EXISTS `idx_tracks` ON `tracks` (`media`, `position`)
"#
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("releases"))
        .execute(conn)
        .await?;

    Ok(())
}
