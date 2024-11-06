use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use sqlx::FromRow;

use crate::utils::macros::{get_and_fetch::impl_get_and_fetch, impl_redirections};

#[derive(Debug, Default, Clone, FromRow, Upsert, MainEntity)]
#[database(
    table = "labels",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
pub struct Label {
    pub id: i64,
    pub mbid: String,
    pub name: String,
    pub label_type: Option<String>,
    pub sort_name: Option<String>,
    pub disambiguation: Option<String>,
    pub country: Option<String>,
    pub label_code: Option<i64>,
    pub annotation: Option<String>,

    pub full_update_date: Option<i64>,
}

impl_redirections!(Label, "labels");
impl_get_and_fetch!(Label);

impl crate::RowId for Label {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}
