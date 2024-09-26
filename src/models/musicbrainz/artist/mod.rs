use musicbrainz_db_lite_macros::Upsert;
use sqlx::prelude::FromRow;
use welds::WeldsModel;

use crate::utils::macros::{get_and_fetch::impl_get_and_fetch, impl_redirections};

#[derive(Debug, WeldsModel, Default, Clone, FromRow, Upsert)]
#[database(name="artists", null_fields(id), ignore_update_keys(id, mbid))]
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
}

impl_redirections!(Artist, "artists");
impl_get_and_fetch!(Artist);
