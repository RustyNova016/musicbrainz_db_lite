use welds::connections::sqlite::SqliteClient;
use welds::connections::Transaction;
use welds::TransactStart;
use welds::{WeldsError};

pub mod listenbrainz;

/// This trait is implemented by all the entities that are able to be saved to the database
pub trait SaveToDatabase {
    /// Save the object into the database, but doesn't commit the transaction
    fn save_in_transaction<'t>(&self, client: &Transaction<'t>) -> impl std::future::Future<Output = Result<(), WeldsError>> + Send;

    /// Save the object into the database
    async fn save(&self, client: &SqliteClient) -> Result<(), WeldsError> {
        let trans = client.begin().await?;
        self.save_in_transaction(&trans);
        trans.commit().await?;
        Ok(())
    }
}