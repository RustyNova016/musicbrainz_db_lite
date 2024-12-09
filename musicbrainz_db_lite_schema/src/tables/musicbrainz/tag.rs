pub(super) async fn create_tag_tables(
    conn: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE
            `tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER
            ) STRICT;

        CREATE INDEX `idx_tag` ON `tag` (`name`);
"#,
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
