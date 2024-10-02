pub mod fetching;
use musicbrainz_rs_nova::entity::artist::Artist as MBArtist;
use sqlx::SqliteConnection;

use crate::{
    api::SaveToDatabase,
    models::musicbrainz::artist::Artist,
};

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
            full_update_date: self.full_update_date
        }
    }

    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBArtist,
    ) -> Result<Self, sqlx::Error> {
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
        ) -> Result<Self, sqlx::Error> {
            let artist = Artist::save_api_response(&mut *conn, value.clone()).await?;
    
            Ok(artist)
        }
}

impl SaveToDatabase for MBArtist {
    type ReturnedData = Artist;

    async fn save(self, client: &mut SqliteConnection) -> Result<Self::ReturnedData, sqlx::Error> {
        Artist::save_api_response(client, self).await
    }
}
