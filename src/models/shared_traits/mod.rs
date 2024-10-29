pub trait RowId {
    /// Returns an unique i64 number taht identify the row
    fn get_row_id(&self) -> i64;
}

pub trait Upsertable: Sized {
    fn upsert(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send;
}
