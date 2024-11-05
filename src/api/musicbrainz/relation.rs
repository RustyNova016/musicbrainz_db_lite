use musicbrainz_rs_nova::entity::relations::Relation as MBRelation;

use crate::models::musicbrainz::relations::Relation;
use crate::utils::date_utils::date_to_timestamp;
use crate::RowId;

impl<T, U> Relation<T, U>
where
    T: Send + Unpin + RowId,
    U: Send + Unpin + RowId,
{
    pub async fn save_api_response(
        conn: &mut sqlx::SqliteConnection,
        value: MBRelation,
        entity0: &T,
        entity1: &U
    ) -> Result<Relation<T, U>, crate::Error> {
        let relation = Relation {
            atribute_values: value
                .attribute_values
                .map(|val| serde_json::to_string(&val))
                .transpose()?,
            attribute_ids: value
                .attribute_ids
                .map(|val| serde_json::to_string(&val))
                .transpose()?,
            attributes: value
                .attributes
                .map(|val| serde_json::to_string(&val))
                .transpose()?,
            begin: value.begin.map(|date| date_to_timestamp(date).unwrap()),
            direction: value.direction,
            end: value.end.map(|date| date_to_timestamp(date).unwrap()),
            id: Default::default(),
            entity0: entity0.get_row_id(),
            entity0_phamtom: Default::default(),
            entity1: entity1.get_row_id(),
            entity1_phamtom: Default::default(),
            relation_type: value.relation_type,
            source_credit: value.source_credit,
            target_credit: value.target_credit,
            target_type: value.target_type,
            type_id: value.type_id,
        };

        Ok(relation.upsert(conn).await?)
    }
}
