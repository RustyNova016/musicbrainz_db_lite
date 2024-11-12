use sqlx::SqliteConnection;

use super::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_release_group_tables(
    conn: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE
            `release_groups` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `mbid` TEXT UNIQUE NOT NULL,
                `primary_type_id` TEXT,
                `first_release_date` INTEGER,
                `disambiguation` TEXT,
                `annotation` TEXT,

                -- Foreign Keys
                `artist_credit` INTEGER REFERENCES `artist_credits` (`id`),
                
                -- Database Utils
                `full_update_date` INTEGER CHECK(`full_update_date` > 0)
            ) STRICT;


        CREATE TRIGGER `trigger_after_delete_release_groups` AFTER DELETE ON `release_groups` BEGIN
            -- Clean full update date
            UPDATE `releases` SET `full_update_date` = NULL WHERE `release_group` = OLD.id;

            -- Remove the artist credit
            DELETE FROM `artist_credits` WHERE id = OLD.artist_credit;
        END;
"#,
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("release_groups"))
        .execute(conn)
        .await?;

    Ok(())
}
