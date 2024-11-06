pub mod fetching;

use crate::models::musicbrainz::{label::Label, release::Release};
use crate::Error;
use musicbrainz_rs_nova::entity::label::Label as MBLabel;
use sqlx::SqliteConnection;

impl Label {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBLabel,
    ) -> Result<Self, crate::Error> {
        Self::add_redirect_mbid(conn, &value.id).await?;
        Self::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Self::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    pub fn merge_api_data(self, new: MBLabel) -> Self {
        Self {
            id: self.id,
            annotation: new.annotation.or(self.annotation),
            country: new.country.or(self.country),
            disambiguation: new.disambiguation.or(self.disambiguation),
            full_update_date: self.full_update_date,
            label_code: new.label_code.map(|v| v as i64).or(self.label_code),
            label_type: new
                .label_type
                .map(|v| serde_json::to_string(&v).unwrap())
                .or(self.label_type),
            mbid: new.id,
            name: new.name,
            sort_name: new.sort_name.or(self.sort_name),
        }
    }

    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBLabel,
    ) -> Result<Self, crate::Error> {
        let new_value = Self::save_api_response(conn, value.clone()).await?;

        // Save relations
        if let Some(releases) = value.releases {
            for release in releases {
                Release::save_api_response(conn, release).await?;
            }
        }

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
