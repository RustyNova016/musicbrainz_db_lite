use crate::RowId;

use super::artist::Artist;
use super::label::Label;
use super::recording::Recording;
use super::release::Release;
use super::work::Work;

/// Contain any of the main entities
pub enum MainEntity {
    Artist(Artist),
    Label(Label),
    Recording(Recording),
    Release(Release),
    Work(Work),
}

impl RowId for MainEntity {
    fn get_row_id(&self) -> i64 {
        match self {
            Self::Artist(val) => val.get_row_id(),
            Self::Label(val) => val.get_row_id(),
            Self::Recording(val) => val.get_row_id(),
            Self::Release(val) => val.get_row_id(),
            Self::Work(val) => val.get_row_id(),
        }
    }
}
