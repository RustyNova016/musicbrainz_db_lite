pub(super) async fn create_genre_tables(
    conn: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE
            `genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER
            ) STRICT;

        CREATE INDEX `idx_genre` ON `genre` (`name`);
"#,
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
