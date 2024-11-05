use sqlx::SqliteConnection;
use welds::connections::sqlite::SqliteClient;
use welds::Client;
use welds::TransactStart;
use welds::WeldsError;

pub mod listenbrainz;
pub mod musicbrainz;

/// This trait is implemented by all the entities that are able to be saved to the database
#[deprecated]
pub trait SaveToDatabaseOld {
    type ReturnedData;

    /// Save the object into the database, with a Client or transaction (without commit)
    fn save(
        &self,
        client: &dyn Client,
    ) -> impl std::future::Future<Output = Result<Self::ReturnedData, WeldsError>> + Send;

    /// Save the object into the database.
    /// This operation create a transaction, only commiting once all childrens have been inserted
    async fn save_wrapped_in_transaction(
        &self,
        client: &SqliteClient,
    ) -> Result<Self::ReturnedData, WeldsError> {
        let trans = client.begin().await?;
        let data = self.save(&trans).await?;
        trans.commit().await?;
        Ok(data)
    }

    /// Save the object into the database.
    /// This operation will not create a transation, and all childrens are commited directly
    async fn save_no_transactions(
        &self,
        client: &SqliteClient,
    ) -> Result<Self::ReturnedData, WeldsError> {
        self.save(client).await
    }
}

/// This trait is implemented by all the entities that are able to be saved to the database
pub trait SaveToDatabase {
    type ReturnedData;

    /// Save the object into the database, with a Client or transaction (without commit)
    fn save(
        self,
        conn: &mut SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Self::ReturnedData, crate::Error>> + Send;
}

pub trait SaveFromAPIToDatabase {
    type ReturnedData;

    /// Upsert the full data, overiding possibly unfetched fields
    fn full_upsert(
        &self,
        conn: &mut SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Self::ReturnedData, sqlx::Error>> + Send;

    /// Upsert fields by merging the data if absent
    fn soft_upsert(
        &self,
        conn: &mut SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Self::ReturnedData, sqlx::Error>> + Send;
}
