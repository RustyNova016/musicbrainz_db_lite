pub trait GetExecutor<'c> {
    async fn get_executor(self) -> Result<impl sqlx::SqliteExecutor<'c>, crate::Error>;
}
