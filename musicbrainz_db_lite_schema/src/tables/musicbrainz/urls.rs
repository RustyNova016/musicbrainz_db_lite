use super::gid_redirect_tables::generate_redirect_table;
use super::tag::create_tag_tables;

pub(super) async fn create_url_tables(
    conn: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
CREATE TABLE
    `urls` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `mbid` TEXT NOT NULL,
        `ressource` TEXT NOT NULL
    ) STRICT;
"#,
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("urls"))
    .execute(&mut *conn)
    .await?;

    create_tag_tables(conn, "url", "urls").await?;

    Ok(())
}
