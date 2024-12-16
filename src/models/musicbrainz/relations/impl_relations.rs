macro_rules! impl_inner_relation {
    ($right_entity: ty, $fn_name: ident) => {
        pub async fn $fn_name(
            &self,
            conn: &mut sqlx::SqliteConnection,
        ) -> Result<Vec<crate::models::musicbrainz::relations::Relation<Self, $right_entity>>, crate::Error> {
            <Self as crate::models::musicbrainz::relations::traits::HasRelation<$right_entity>>::get_entity_relations(self, conn).await
        }
    };
}

macro_rules! impl_relations {
    ($left_entity: ty) => {
        impl $left_entity {
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::artist::Artist,
                get_artist_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::genre::Genre,
                get_genre_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::label::Label,
                get_label_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::recording::Recording,
                get_recording_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::release::Release,
                get_release_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::release_group::ReleaseGroup,
                get_release_group_relations
            );
            crate::models::musicbrainz::relations::impl_relations::impl_inner_relation!(
                crate::models::musicbrainz::work::Work,
                get_work_relations
            );
        }
    };
}

pub(crate) use impl_inner_relation;
pub(crate) use impl_relations;
