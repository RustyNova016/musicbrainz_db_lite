macro_rules! impl_get_and_fetch {
    ($left_entity: ty, $left_field: ident, $right_entity: ty, &right_field: ident, &right_table_name: literal) => {
        impl EntityRelations<$left_entity, $right_entity> {
            pub async fn inner_join(
                self,
                conn: &mut sqlx::SqliteConnection,
                data: Vec<$left_entity>,
            ) -> Result<Self, crate::Error> {
                let left_ids = data.iter().map(|l| l.recording_mbid.clone()).collect_vec();
        
                let left_ids_string = serde_json::to_string(&left_ids)?;
                let res: Vec<$right_entity> = sqlx::query_as(
                    format!("SELECT * FROM recordings_gid_redirect WHERE gid IN (SELECT value from json_each(?))"),
                    left_ids_string
                )
                .fetch_all(conn)
                .await?;
        
                let mapped = inner_join_values(
                    data.into_iter().map(|l| (l.recording_mbid.clone(), l)),
                    res.into_iter().map(|r| (r.gid.clone(), r)),
                );
        
                Ok(Self { relations: mapped })
            }
        }
        
    };
}

pub(crate) use impl_get_and_fetch;
