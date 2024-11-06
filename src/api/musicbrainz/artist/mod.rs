pub mod fetching;

use musicbrainz_rs_nova::entity::artist::Artist as MBArtist;
use sqlx::SqliteConnection;

use crate::api::SaveToDatabase;
use crate::models::musicbrainz::artist::Artist;
use crate::Error;

impl Artist {
    pub fn merge_api_data(self, new: MBArtist) -> Self {
        Self {
            annotation: new.annotation.or(self.annotation),
            id: self.id,
            country: new.country.or(self.country),
            disambiguation: new.disambiguation,
            mbid: new.id,
            name: new.name,
            sort_name: new.sort_name,
            full_update_date: self.full_update_date,
        }
    }

    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBArtist,
    ) -> Result<Self, crate::Error> {
        Artist::add_redirect_mbid(conn, &value.id).await?;
        Artist::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Artist::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    /// Save a recording from the api data. It also save the relationships
    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBArtist,
    ) -> Result<Self, crate::Error> {
        let artist = Artist::save_api_response(&mut *conn, value.clone()).await?;

        if let Some(relations) = value.relations {
            for rel in relations {
                match artist.save_relation(conn, rel).await {
                    Ok(_) => {}
                    Err(Error::RelationNotImplemented) => {}
                    Err(err) => {
                        Err(err)?;
                    }
                }
            }
        }

        Ok(artist)
    }
}

impl SaveToDatabase for MBArtist {
    type ReturnedData = Artist;

    async fn save(self, client: &mut SqliteConnection) -> Result<Self::ReturnedData, crate::Error> {
        Artist::save_api_response(client, self).await
    }
}
