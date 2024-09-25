pub mod fetching;
use musicbrainz_rs_nova::entity::artist::Artist as MBArtist;
use sqlx::SqliteConnection;
use welds::state::DbState;

use crate::{
    api::{SaveToDatabase, SaveToDatabaseOld},
    models::musicbrainz::artists::Artist,
};

impl From<&MBArtist> for Artist {
    fn from(value: &MBArtist) -> Self {
        Self {
            id: Default::default(),
            mbid: value.id.clone(),
            name: value.name.clone(),
            sort_name: value.sort_name.clone(),
            disambiguation: value.disambiguation.clone(),
            country: value.country.clone(),
            annotation: value.annotation.clone(),
        }
    }
}

impl SaveToDatabase for MBArtist {
    type ReturnedData = Artist;

    async fn save(&self, client: &mut SqliteConnection) -> Result<Self::ReturnedData, sqlx::Error> {
        Artist::from(self).upsert(client).await
    }
}
