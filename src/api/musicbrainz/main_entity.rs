use musicbrainz_rs_nova::entity::relations::RelationContent;

use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::label::Label;
use crate::models::musicbrainz::main_entities::MainEntity;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;
use crate::Error;

impl MainEntity {
    pub async fn save_relation_content(
        conn: &mut sqlx::SqliteConnection,
        value: RelationContent,
    ) -> Result<Self, crate::Error> {
        Ok(match value {
            RelationContent::Artist(value) => {
                Self::Artist(Artist::save_api_response(conn, *value).await?)
            }
            RelationContent::Label(value) => {
                Self::Label(Label::save_api_response(conn, *value).await?)
            }
            RelationContent::Recording(value) => {
                Self::Recording(Recording::save_api_response(conn, *value).await?)
            }
            RelationContent::Release(value) => {
                Self::Release(Release::save_api_response(conn, *value).await?)
            }
            _ => Err(Error::RelationNotImplemented)?,
        })
    }
}
