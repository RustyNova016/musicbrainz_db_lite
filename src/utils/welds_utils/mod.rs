use welds::connections::sqlite::SqliteClient;
use welds::connections::Transaction;

pub enum ClientOrTransaction<'t> {
    Client(&'t SqliteClient),
    Transaction(Transaction<'t>),
}

//#[async_trait]
//impl<'t> Client for ClientOrTransaction<'t> {
//    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult, WeldsError> {
//        match self {
//            Self::Client(val) => val.execute(sql, params).await,
//            Self::Transaction(val) => val.execute(sql, params).await
//        }
//    }
//}
