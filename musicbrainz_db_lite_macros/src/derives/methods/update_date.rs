use proc_macro2::TokenStream;
use syn::Ident;

pub fn impl_update_date(struct_name: &Ident, table_name: &str, pk: &str) -> TokenStream {
    let sql = format!(
        "UPDATE `{}` SET `full_update_date` = ? WHERE {} = ?",
        table_name, pk
    );

    quote::quote! {
        /// Reset the full update date to be now
        pub async fn reset_full_update_date(conn: &mut sqlx::SqliteConnection, id: i64) -> Result<(), sqlx::Error> {
            sqlx::query(#sql).bind(chrono::Utc::now().timestamp()).bind(id).execute(conn).await?;
            Ok(())
        }

        /// Get from the database and perform an update if the data isn't fully present
        pub async fn get_or_fetch_as_complete(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
            match self.full_update_date {
                Some(_) => Ok(self.clone()),
                None => self.refetch(conn).await
            }     
        }

        /// Get from the database and perform an update if the data isn't fully present
        pub async fn get_or_fetch_as_complete_from_mbid(conn: &mut sqlx::SqliteConnection, mbid: &str) -> Result<Option<Self>, crate::Error> {
            match Self::find_by_mbid(conn, mbid).await? {
                Some(data) => {
                    if data.full_update_date.is_none() {
                        return Ok(Some(data.refetch(conn).await?))
                    }
                    Ok(Some(data))
                },
                None => Self::fetch_and_save(conn, mbid).await
            }
        }

        /// Refresh the data in the database by refetching the entity
        pub async fn refetch(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
            Self::fetch_and_save(conn, &self.mbid).await?.ok_or(crate::Error::UnknownUpstream(self.mbid.clone()))
        }
    }
}
