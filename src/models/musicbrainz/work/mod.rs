use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use sqlx::FromRow;

use crate::utils::macros::{get_and_fetch::impl_get_and_fetch, impl_redirections};

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Upsert, MainEntity)]
#[database(
    table = "Works",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
pub struct Work {
    pub id: i64,
    pub mbid: String,
    pub title: String,
    pub work_type: Option<String>,
    pub disambiguation: Option<String>,
    pub annotation: Option<String>,

    pub full_update_date: Option<i64>,
}

impl_redirections!(Work, "Works");
impl_get_and_fetch!(Work);

impl crate::RowId for Work {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}
