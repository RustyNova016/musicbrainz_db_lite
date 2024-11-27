/// Trigger that delete the artist credit attached to the entity when this entity is deleted
pub async fn after_delete_table_artist_credits(
    conn: &mut sqlx::SqliteConnection,
    table: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(&format!(
        "
    CREATE TRIGGER `trigger_after_delete_{table}_artist_credits` AFTER DELETE ON `{table}` BEGIN
        DELETE FROM artist_credits WHERE artist_credit.id = OLD.artist_credit;
    END
    "
    ))
    .execute(conn)
    .await
}
