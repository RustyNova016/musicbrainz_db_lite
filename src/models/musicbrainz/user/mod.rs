use sqlx::{Executor, Sqlite, SqliteConnection};
use welds::{state::DbState, WeldsModel};

use crate::models::listenbrainz::listen::Listen;

#[derive(Debug, WeldsModel, sqlx::FromRow)]
#[welds(table = "users")]
#[welds(HasMany(listens, Listen, "id"))]
pub struct User {
    #[welds(primary_key)]
    pub id: i64,

    pub name: String,
}

impl User {
    pub async fn insert_or_ignore(
        client: impl Executor<'_, Database = Sqlite>,
        name: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT OR IGNORE INTO users VALUES (NULL, ?)", name)
            .execute(client)
            .await?;
        Ok(())
    }

    /// Finds an user by its name
    pub async fn find_by_name(
        conn: &mut SqliteConnection,
        name: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE name = ?", name)
            .fetch_optional(conn)
            .await
    }
}
