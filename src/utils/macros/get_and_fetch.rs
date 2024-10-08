macro_rules! impl_get_and_fetch {
    ($row_struct: ty) => {
        impl $row_struct {
            pub async fn get_or_fetch(conn: &mut sqlx::SqliteConnection, mbid: &str) -> Result<Option<Self>, crate::Error> {
                let data = Self::find_by_mbid(conn, mbid).await?;
        
                match data {
                    Some(val) => Ok(Some(val)),
                    None => Self::fetch_and_save(conn, mbid).await,
                }
            }
        }
    };
}

pub(crate) use impl_get_and_fetch;
