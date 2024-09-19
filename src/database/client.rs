use welds::{connections::sqlite::{self, SqliteClient}, WeldsError};

pub struct DBClient {
    connection: SqliteClient
}

impl DBClient {
    pub async fn connect(path: &str) -> Result<DBClient, WeldsError> {
        Ok(
            Self {
                connection: sqlite::connect(&format!("sqlite:{}", path)).await?
            }
        )
    }

}