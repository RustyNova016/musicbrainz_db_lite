use welds::connections::sqlite::SqliteClient;
use welds::Client;
use welds::TransactStart;
use welds::WeldsError;

pub mod listenbrainz;

/// This trait is implemented by all the entities that are able to be saved to the database
pub trait SaveToDatabase {
    /// Save the object into the database, but doesn't commit the transaction
    fn save_in_transaction(
        &self,
        client: &dyn Client,
    ) -> impl std::future::Future<Output = Result<(), WeldsError>> + Send;

    /// Save the object into the database
    async fn save(&self, client: &SqliteClient) -> Result<(), WeldsError> {
        let trans = client.begin().await?;
        self.save_in_transaction(&trans).await?;
        trans.commit().await?;
        Ok(())
    }

    async fn save_no_transactions(&self, client: &SqliteClient) -> Result<(), WeldsError> {
        self.save(client).await
    }
}
