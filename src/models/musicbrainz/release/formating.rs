use crate::models::musicbrainz::release::Release;

impl Release {
    /// Return a string containing the recording name and its artist credits
    ///
    /// Exemple: Never Gonna Give You Up by Rick Astley
    pub async fn format_with_credits(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<String, crate::Error> {
        let credit = self.get_artist_credits_or_fetch(conn).await?.to_string();
        Ok(format!("{} by {}", self.title, credit))
    }
}
