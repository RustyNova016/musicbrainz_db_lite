macro_rules! impl_get_relation {
    ($left_entity: ty, $right_entity: ty, $table_name: literal) => {
        impl crate::models::musicbrainz::relations::Relation<$left_entity, $right_entity> {
            pub async fn get_relations_of(
                conn: &mut sqlx::SqliteConnection,
                entity: $left_entity,
            ) -> Result<
                Vec<crate::models::musicbrainz::relations::Relation<$left_entity, $right_entity>>,
                sqlx::Error,
            > {
                let sql = format!("SELECT * FROM {} WHERE `entity0` = ?", $table_name);
                let relations: Vec<
                    crate::models::musicbrainz::relations::Relation<$left_entity, $right_entity>,
                > = sqlx::query_as(&sql).bind(entity.id).fetch_all(conn).await?;

                Ok(relations)
            }
        }
    };
}

pub(crate) use impl_get_relation;

use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::label::Label;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;

impl_get_relation!(Artist, Artist, "l_artists_artists");
impl_get_relation!(Artist, Label, "l_artists_labels");
impl_get_relation!(Artist, Recording, "l_artists_recordings");
impl_get_relation!(Artist, Release, "l_artists_releases");

impl_get_relation!(Label, Artist, "l_artists_labels");
impl_get_relation!(Label, Label, "l_labels_labels");
impl_get_relation!(Label, Recording, "l_labels_recordings");
impl_get_relation!(Label, Release, "l_labels_releases");

impl_get_relation!(Recording, Artist, "l_artists_recordings");
impl_get_relation!(Recording, Label, "l_labels_recordings");
impl_get_relation!(Recording, Recording, "l_recordings_recordings");
impl_get_relation!(Recording, Release, "l_recordings_releases");

impl_get_relation!(Release, Artist, "l_artists_releases");
impl_get_relation!(Release, Label, "l_labels_releases");
impl_get_relation!(Release, Recording, "l_recordings_releases");
impl_get_relation!(Release, Release, "l_releases_releases");
