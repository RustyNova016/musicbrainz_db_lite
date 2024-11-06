use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use sqlx::prelude::FromRow;
use welds::WeldsModel;

use crate::utils::macros::{get_and_fetch::impl_get_and_fetch, impl_redirections};

#[derive(Debug, WeldsModel, Default, Clone, FromRow, Upsert, MainEntity)]
#[database(
    table = "artists",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
#[welds(table = "artists")]
pub struct Artist {
    #[welds(primary_key)]
    pub id: i64,
    pub mbid: String,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub country: Option<String>,
    pub annotation: Option<String>,

    pub full_update_date: Option<i64>,
}

impl_redirections!(Artist, "artists");
impl_get_and_fetch!(Artist);

impl crate::RowId for Artist {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}
