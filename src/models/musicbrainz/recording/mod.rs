use chrono::Duration;
use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use serde::Deserialize;
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::Row as _;
use sqlx::FromRow;

use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::{
    artist_credits::impl_artist_credits, get_and_fetch::impl_get_and_fetch, impl_redirections,
};

use super::relations::impl_relations::impl_relations;

pub mod relations;
pub mod finds;

#[derive(Debug, Default, PartialEq, Eq, Clone, Upsert, MainEntity, Deserialize, Serialize)]
#[database(
    table = "recordings",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Recording {
    pub id: i64,
    pub mbid: String,
    pub title: String,
    pub video: Option<bool>,
    pub length: Option<i64>,
    pub disambiguation: Option<String>,
    pub annotation: Option<String>,
    pub first_release_date: Option<i64>,

    pub full_update_date: Option<i64>,

    pub artist_credit: Option<i64>,
}

impl_redirections!(Recording, "recordings");
impl_artist_credits!(Recording, "recordings");
impl_get_and_fetch!(Recording);
impl_relations!(Recording);

impl crate::RowId for Recording {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl Recording {
    pub fn length_as_duration(&self) -> Option<Duration> {
        self.length.and_then(|length| {
            Duration::new(length.div_euclid(1000), length.rem_euclid(1000) as u32)
        })
    }
}

impl HasTable for Recording {
    const TABLE_NAME: &str = "recordings";
    const FOREIGN_FIELD_NAME: &str = "recording";
}

impl HasTags for Recording {}
impl HasGenres for Recording {}

// impl FromRow<'_, SqliteRow> for Recording {
//     fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
//         let video = row.t
//     }
// }

impl FromRow<'_, SqliteRow> for Recording {
    fn from_row(__row: &SqliteRow) -> ::sqlx::Result<Self> {
        let id: i64 = __row.try_get("id")?;
        let mbid: String = __row.try_get("mbid")?;
        let title: String = __row.try_get("title")?;
        let video: Option<i64> = __row.try_get("video")?;
        let video: Option<bool> = video.map(|a| match a {
            0 => false,
            1 => true,
            _ => panic!("Tried casting a i64 to a bool"),
        });
        let length: Option<i64> = __row.try_get("length")?;
        let disambiguation: Option<String> = __row.try_get("disambiguation")?;
        let annotation: Option<String> = __row.try_get("annotation")?;
        let first_release_date: Option<i64> = __row.try_get("first_release_date")?;
        let full_update_date: Option<i64> = __row.try_get("full_update_date")?;
        let artist_credit: Option<i64> = __row.try_get("artist_credit")?;
        ::std::result::Result::Ok(Recording {
            id,
            mbid,
            title,
            video,
            length,
            disambiguation,
            annotation,
            first_release_date,
            full_update_date,
            artist_credit,
        })
    }
}
