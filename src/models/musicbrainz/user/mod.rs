use welds::{state::DbState, Client, WeldsError, WeldsModel};

use crate::models::listenbrainz::listen::Listen;

#[derive(Debug, WeldsModel)]
#[welds(table = "users")]
#[welds(HasMany(listens, Listen, "id"))]
pub struct User {
    #[welds(primary_key)]
    pub id: i32,

    pub name: String,
}

impl User {
    /// Get an user by name, and if not found, create it
    pub async fn get_or_create_user(
        client: &dyn Client,
        name: &str,
    ) -> Result<DbState<User>, WeldsError> {
        if let Some(user) = Self::get_user(client, name).await? {
            return Ok(user);
        }

        let mut user = User::new();
        user.name = name.to_string();
        user.save(client).await?;
        Ok(user)
    }

    /// Fetch an user from the database
    pub async fn get_user(
        client: &dyn Client,
        name: &str,
    ) -> Result<Option<DbState<User>>, WeldsError> {
        Ok(User::all()
            .where_col(|c| c.name.equal(name))
            .limit(1)
            .run(client)
            .await?
            .pop())
    }
}
