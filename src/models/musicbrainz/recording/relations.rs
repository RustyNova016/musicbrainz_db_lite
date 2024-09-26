use sqlx::SqliteConnection;

use crate::models::musicbrainz::artist_credit::ArtistCredits;

use super::Recording;

impl Recording {
    pub async fn get_artist_credits(
        &self,
        conn: &mut SqliteConnection,
    ) -> Option<Result<ArtistCredits, sqlx::Error>> {
        match self.artist_credit {
            Some(id) => Some(ArtistCredits::find_by_id(conn, id).await),
            None => None,
        }
    }

    pub async fn set_artist_credits(
        &mut self,
        conn: &mut SqliteConnection,
        credits_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE `recordings` SET artist_credit = ? WHERE id = ?",
            credits_id,
            self.id
        )
        .execute(conn)
        .await?;

        self.artist_credit = Some(credits_id);

        Ok(())
    }
}
