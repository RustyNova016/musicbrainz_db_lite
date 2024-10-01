use musicbrainz_db_lite_macros::Upsert;
use redirect::RecordingGidRedirect;
use sqlx::prelude::FromRow;
use welds::{state::DbState, WeldsModel};

use crate::utils::macros::{artist_credits::impl_artist_credits, get_and_fetch::impl_get_and_fetch, impl_redirections};

pub mod redirect;

#[derive(Debug, WeldsModel, Default, FromRow, Upsert)]
#[database(name="recordings", null_fields(id), ignore_update_keys(id, mbid))]
#[welds(table = "recordings")]
#[welds(HasMany(mbid, RecordingGidRedirect, "new_id"))]
pub struct Recording {
    #[welds(primary_key)]
    pub id: i64,

    pub mbid: String,

    pub title: String,

    pub length: Option<i64>,

    pub disambiguation: Option<String>,

    pub annotation: Option<String>,

    pub(crate) artist_credit: Option<i64>,
}

impl_redirections!(Recording, "recordings");
impl_artist_credits!(Recording, "recordings");
impl_get_and_fetch!(Recording);

impl Recording {
    pub fn replace(mut row: DbState<Recording>, new: Recording) -> DbState<Self> {
        let id = row.id;

        *row = new;
        row.id = id;

        row
    }
}
