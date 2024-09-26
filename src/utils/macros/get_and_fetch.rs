macro_rules! impl_get_and_fetch {
    ($row_struct: ty) => {
        impl $row_struct {
            pub async fn get_or_fetch(conn: &mut sqlx::SqliteConnection, mbid: &str) -> Result<Self, crate::Error> {
                let artist = Self::find_by_mbid(conn, mbid).await?;
        
                match artist {
                    Some(val) => Ok(val),
                    None => Self::fetch_and_save(conn, mbid).await,
                }
            }
        }
    };
}

pub(crate) use impl_get_and_fetch;
