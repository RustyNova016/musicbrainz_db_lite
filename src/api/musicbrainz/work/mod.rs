pub mod fetching;
use musicbrainz_rs_nova::entity::work::Work as MBWork;

use crate::models::musicbrainz::work::Work;
use crate::Error;

impl Work {
    /// Save an api response into the database
    pub async fn save_api_response(
        conn: &mut sqlx::SqliteConnection,
        value: MBWork,
    ) -> Result<Self, crate::Error> {
        Self::add_redirect_mbid(conn, &value.id).await?;
        Self::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Self::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    /// Merge an Entity with its counterpart in musicbrainz_rs_nova. It always prefers data from musicbrainz_rs_nova over the cached one
    pub fn merge_api_data(self, new: MBWork) -> Self {
        Self {
            id: self.id,
            mbid: new.id,
            title: new.title,
            annotation: new.annotation.or(self.annotation),
            disambiguation: new.disambiguation.or(self.disambiguation),
            work_type: new
                .work_type
                .map(|w| serde_json::to_string(&w).expect("The enum should be serializable"))
                .or(self.work_type),
            full_update_date: self.full_update_date,
        }
    }

    /// Save the responce from `musicbrainz_rs_nova` and its children relations
    pub async fn save_api_response_recursive(
        conn: &mut sqlx::SqliteConnection,
        value: MBWork,
    ) -> Result<Self, crate::Error> {
        let new_value = Self::save_api_response(conn, value.clone()).await?;

        // Save relations

        if let Some(relations) = value.relations {
            for rel in relations {
                match new_value.save_relation(conn, rel).await {
                    Ok(_) => {}
                    Err(Error::RelationNotImplemented) => {}
                    Err(err) => {
                        Err(err)?;
                    }
                }
            }
        }

        Ok(new_value)
    }
}
