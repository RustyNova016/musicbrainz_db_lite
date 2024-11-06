macro_rules! impl_has_relation {
    ($left_entity: ty) => {
        impl $left_entity {
            pub(crate) async fn save_relation(
                &self,
                conn: &mut sqlx::SqliteConnection,
                api_relation: musicbrainz_rs_nova::entity::relations::Relation,
            ) -> Result<(), crate::Error> {
                Ok(match api_relation.content.clone() {
                    musicbrainz_rs_nova::entity::relations::RelationContent::Artist(value) => {
                        let entity1 = Artist::save_api_response(conn, *value).await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    musicbrainz_rs_nova::entity::relations::RelationContent::Label(value) => {
                        let entity1 = crate::models::musicbrainz::label::Label::save_api_response(
                            conn, *value,
                        )
                        .await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    musicbrainz_rs_nova::entity::relations::RelationContent::Recording(value) => {
                        let entity1 = Recording::save_api_response(conn, *value).await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    musicbrainz_rs_nova::entity::relations::RelationContent::Release(value) => {
                        let entity1 =
                            crate::models::musicbrainz::release::Release::save_api_response(
                                conn, *value,
                            )
                            .await?;

                        crate::models::musicbrainz::relations::Relation::save_api_response_inner(
                            conn,
                            api_relation,
                            self,
                            &entity1,
                        )
                        .await?;
                    }
                    _ => Err(crate::Error::RelationNotImplemented)?,
                })
            }
        }
    };
}

use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::label::Label;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;
pub(crate) use impl_has_relation;

impl_has_relation!(Artist);
impl_has_relation!(Label);
impl_has_relation!(Recording);
impl_has_relation!(Release);
