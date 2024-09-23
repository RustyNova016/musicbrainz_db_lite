use sqlx::{Executor, Sqlite, SqlitePool};
use welds::{connections::sqlite::SqliteClient, state::DbState, Client, WeldsError, WeldsModel};

use crate::{models::listenbrainz::listen::Listen, Error};

#[derive(Debug, WeldsModel, sqlx::FromRow)]
#[welds(table = "users")]
#[welds(HasMany(listens, Listen, "id"))]
pub struct User {
    #[welds(primary_key)]
    pub id: i64,

    pub name: String,
}

impl User {
    pub async fn insert_or_ignore(client: impl Executor<'_, Database = Sqlite>, name: &str) -> Result<(), sqlx::Error>  {
        sqlx::query!("INSERT OR IGNORE INTO users VALUES (NULL, ?)", name)
            .execute(client)
            .await?;
        Ok(())
    }

    /// Finds an user by its name
    pub async fn find_by_name(client: impl Executor<'_, Database = Sqlite>, name: &str) -> Result<Option<DbState<User>>, sqlx::Error>  {
        let res = sqlx::query_as!(User, "SELECT * FROM users WHERE name = ?", name)
            .fetch_one(client)
            .await;

        match res {
            Ok(val) => Ok(Some(DbState::db_loaded(val) )),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err)
        }
    }


}
