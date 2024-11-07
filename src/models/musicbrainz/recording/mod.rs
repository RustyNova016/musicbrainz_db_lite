use chrono::Duration;
use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use serde::Deserialize;
use serde::Serialize;
use sqlx::{prelude::FromRow, SqliteConnection};
use welds::{state::DbState, WeldsModel};

use crate::utils::macros::{
    artist_credits::impl_artist_credits, get_and_fetch::impl_get_and_fetch, impl_redirections,
};

pub mod redirect;
pub mod relations;

#[derive(
    Debug,
    WeldsModel,
    Default,
    PartialEq,
    Eq,
    Clone,
    FromRow,
    Upsert,
    MainEntity,
    Deserialize,
    Serialize,
)]
#[database(
    table = "recordings",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
#[welds(table = "recordings")]
pub struct Recording {
    #[welds(primary_key)]
    pub id: i64,
    pub mbid: String,
    pub title: String,
    pub video: Option<i64>,
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

impl crate::RowId for Recording {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl Recording {
    pub fn replace(mut row: DbState<Recording>, new: Recording) -> DbState<Self> {
        let id = row.id;

        *row = new;
        row.id = id;

        row
    }

    pub fn length_as_duration(&self) -> Option<Duration> {
        self.length.and_then(|length| {
            Duration::new(length.div_euclid(1000), length.rem_euclid(1000) as u32)
        })
    }

    /// Return a string containing the recording name and its artist credits
    ///
    /// Exemple: Never Gonna Give You Up by Rick Astley
    pub async fn format_with_credits(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<String, crate::Error> {
        let credit = self.get_artist_credits_or_fetch(conn).await?.to_string();
        Ok(format!("{} by {}", self.title, credit))
    }
}
