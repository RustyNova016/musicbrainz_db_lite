use musicbrainz_rs_nova::entity::release::Release as MBRelease;
use sqlx::SqliteConnection;

use crate::models::musicbrainz::artist_credit::ArtistCredits;
use crate::models::musicbrainz::release::LabelInfo;
use crate::models::musicbrainz::release::Media;
use crate::models::musicbrainz::release::Release;
use crate::utils::date_utils::date_to_timestamp;
use crate::Error;

pub mod fetching;
pub mod label_info;
pub mod media;
pub mod tracks;

impl Release {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBRelease,
    ) -> Result<Self, crate::Error> {
        Release::add_redirect_mbid(conn, &value.id).await?;
        Release::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Release::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    pub fn merge_api_data(self, new: MBRelease) -> Self {
        Self {
            id: self.id,
            annotation: new.annotation.or(self.annotation),
            mbid: new.id,
            artist_credit: self.artist_credit,
            barcode: new.barcode.or(self.barcode),
            country: new.country.or(self.country),
            date: new
                .date
                .map(|date| date_to_timestamp(date).unwrap())
                .or(self.date),
            disambiguation: new.disambiguation.or(self.disambiguation),
            packaging: self.packaging, //TODO: Packaging to string
            title: new.title,
            quality: self.quality, //TODO: Quality to string
            status: self.status,   //TODO: Status to string
            full_update_date: self.full_update_date,
        }
    }

    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBRelease,
    ) -> Result<Self, crate::Error> {
        let mut new_release = Release::save_api_response(conn, value.clone()).await?;

        // Save relations
        if let Some(artist_credits) = value.artist_credit.clone() {
            let credits = ArtistCredits::save_api_response(conn, artist_credits).await?;
            new_release.set_artist_credits(conn, credits.0).await?;
        }

        if let Some(values) = value.media.clone() {
            Media::save_api_response(conn, values, new_release.id).await?;
        }

        if let Some(values) = value.label_info {
            LabelInfo::save_api_response(conn, values, new_release.id).await?;
        }

        if let Some(relations) = value.relations {
            for rel in relations {
                match new_release.save_relation(conn, rel).await {
                    Ok(_) => {}
                    Err(Error::RelationNotImplemented) => {}
                    Err(err) => {
                        Err(err)?;
                    }
                }
            }
        }

        Ok(new_release)
    }
}
