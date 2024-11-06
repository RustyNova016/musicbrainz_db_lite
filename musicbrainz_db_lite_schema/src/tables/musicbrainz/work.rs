use sqlx::SqliteConnection;

use super::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_work_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS
            `works` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `work_type` TEXT,
                `disambiguation` TEXT,
                `annotation` TEXT,

                -- Database Utils
                `full_update_date` INTEGER CHECK(`full_update_date` > 0)
            ) STRICT;

"#,
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("works"))
        .execute(conn)
        .await?;

    Ok(())
}
