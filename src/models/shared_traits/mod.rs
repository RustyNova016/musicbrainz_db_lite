pub trait RowId {
    /// Returns an unique i64 number taht identify the row
    fn get_row_id(&self) -> i64;
}