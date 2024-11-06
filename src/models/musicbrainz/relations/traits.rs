pub trait HasRelation<U> {
    const TABLE: &str;
}

macro_rules! impl_has_relation {
    ($left_entity: ty, $right_entity: ty, $table_name: literal) => {
        impl crate::models::musicbrainz::relations::traits::HasRelation<$right_entity>
            for $left_entity
        {
            const TABLE: &str = $table_name;
        }
    };
}

pub(crate) use impl_has_relation;

use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::label::Label;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;

impl_has_relation!(Artist, Artist, "l_artists_artists");
impl_has_relation!(Artist, Label, "l_artists_labels");
impl_has_relation!(Artist, Recording, "l_artists_recordings");
impl_has_relation!(Artist, Release, "l_artists_releases");

impl_has_relation!(Label, Artist, "l_artists_labels");
impl_has_relation!(Label, Label, "l_labels_labels");
impl_has_relation!(Label, Recording, "l_labels_recordings");
impl_has_relation!(Label, Release, "l_labels_releases");

impl_has_relation!(Recording, Artist, "l_artists_recordings");
impl_has_relation!(Recording, Label, "l_labels_recordings");
impl_has_relation!(Recording, Recording, "l_recordings_recordings");
impl_has_relation!(Recording, Release, "l_recordings_releases");

impl_has_relation!(Release, Artist, "l_artists_releases");
impl_has_relation!(Release, Label, "l_labels_releases");
impl_has_relation!(Release, Recording, "l_recordings_releases");
impl_has_relation!(Release, Release, "l_releases_releases");
