pub trait HasRelation<U>: RowId {
    /// The name of the table where the relation is stored
    const RELATION_TABLE: &str;

    /// Get the row_id of the entity in the first position
    ///
    /// Ex: In a Artist -> Recording relationship, Artist is the entity0
    fn get_entity0_id(&self, other: &U) -> i64;

    /// Get the row_id of the entity in the second position
    ///
    /// Ex: In a Artist -> Recording relationship, Recording is the entity1
    fn get_entity1_id(&self, other: &U) -> i64;
}

macro_rules! impl_has_relation {
    ($left_entity: ty, $right_entity: ty, $table_name: literal) => {
        impl crate::models::musicbrainz::relations::traits::HasRelation<$right_entity>
            for $left_entity
        {
            const RELATION_TABLE: &str = $table_name;

            fn get_entity0_id(&self, _other: &$right_entity) -> i64 {
                self.get_row_id()
            }

            fn get_entity1_id(&self, other: &$right_entity) -> i64 {
                other.get_row_id()
            }
        }
    };
}

macro_rules! impl_reverse_has_relation {
    ($left_entity: ty, $right_entity: ty, $table_name: literal) => {
        impl crate::models::musicbrainz::relations::traits::HasRelation<$left_entity>
            for $right_entity
        {
            const RELATION_TABLE: &str = $table_name;

            fn get_entity0_id(&self, other: &$left_entity) -> i64 {
                other.get_row_id()
            }

            fn get_entity1_id(&self, _other: &$left_entity) -> i64 {
                self.get_row_id()
            }
        }
    };
}

pub(crate) use impl_reverse_has_relation;

use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::label::Label;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::work::Work;
use crate::RowId;

impl_has_relation!(Artist, Artist, "l_artists_artists");
impl_has_relation!(Artist, Label, "l_artists_labels");
impl_reverse_has_relation!(Artist, Label, "l_artists_labels");
impl_has_relation!(Artist, Recording, "l_artists_recordings");
impl_reverse_has_relation!(Artist, Recording, "l_artists_recordings");
impl_has_relation!(Artist, Release, "l_artists_releases");
impl_reverse_has_relation!(Artist, Release, "l_artists_releases");
impl_has_relation!(Artist, Work, "l_artists_works");
impl_reverse_has_relation!(Artist, Work, "l_artists_works");

impl_has_relation!(Label, Label, "l_labels_labels");
impl_has_relation!(Label, Recording, "l_labels_recordings");
impl_reverse_has_relation!(Label, Recording, "l_labels_recordings");
impl_has_relation!(Label, Release, "l_labels_releases");
impl_reverse_has_relation!(Label, Release, "l_labels_releases");
impl_has_relation!(Label, Work, "l_labels_works");
impl_reverse_has_relation!(Label, Work, "l_labels_works");

impl_has_relation!(Recording, Recording, "l_recordings_recordings");
impl_has_relation!(Recording, Release, "l_recordings_releases");
impl_reverse_has_relation!(Recording, Release, "l_recordings_releases");
impl_has_relation!(Recording, Work, "l_recordings_works");
impl_reverse_has_relation!(Recording, Work, "l_recordings_works");

impl_has_relation!(Release, Release, "l_releases_releases");
impl_has_relation!(Release, Work, "l_releases_works");
impl_reverse_has_relation!(Release, Work, "l_releases_works");

impl_has_relation!(Work, Work, "l_works_works");
