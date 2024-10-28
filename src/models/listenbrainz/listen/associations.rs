use itertools::Itertools;

use crate::models::listenbrainz::msid_mapping::MsidMapping;
use crate::models::musicbrainz::recording::redirect::RecordingGidRedirect;
use crate::utils::sqlx_utils::entity_relations::{
    inner_join_values, EntityRelations, JoinRelation,
};

use super::Listen;
