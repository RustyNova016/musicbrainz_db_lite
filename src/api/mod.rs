pub mod musicbrainz;
use welds::connections::sqlite::SqliteClient;
use welds::Client;
use welds::TransactStart;
use welds::WeldsError;

pub mod listenbrainz;

/// This trait is implemented by all the entities that are able to be saved to the database
pub trait SaveToDatabase {
    /// Save the object into the database, with a Client or transaction (without commit)
    fn save(
        &self,
        client: &dyn Client,
    ) -> impl std::future::Future<Output = Result<(), WeldsError>> + Send;

    /// Save the object into the database.
    /// This operation create a transaction, only commiting once all childrens have been inserted
    async fn save_wrapped_in_transaction(&self, client: &SqliteClient) -> Result<(), WeldsError> {
        let trans = client.begin().await?;
        self.save(&trans).await?;
        trans.commit().await?;
        Ok(())
    }

    /// Save the object into the database.
    /// This operation will not create a transation, and all childrens are commited directly
    async fn save_no_transactions(&self, client: &SqliteClient) -> Result<(), WeldsError> {
        self.save(client).await
    }
}
