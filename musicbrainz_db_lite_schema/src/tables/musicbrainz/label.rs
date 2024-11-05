use sqlx::SqliteConnection;

use super::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_label_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS
            `labels` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `label_type` TEXT,
                `sort_name` TEXT,
                `disambiguation` TEXT,
                `country` TEXT,
                `label_code` INTEGER,
                `annotation` TEXT,

                -- Database Utils
                `full_update_date` INTEGER CHECK(`full_update_date` > 0)
            ) STRICT;

"#
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("labels"))
        .execute(conn)
        .await?;

    Ok(())
}
