
use sqlx::Sqlite;
use welds::connections::sqlite::SqliteClient;
use welds::connections::Transaction;
use welds::model_traits::HasSchema;
use welds::model_traits::TableColumns;
use welds::model_traits::TableInfo;
use welds::Client;
use welds::WeldsError;

use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::listen_user_metadata::MessybrainzSubmission;
use crate::models::musicbrainz::user::User;

pub mod extensions;
pub mod welds_utils;

pub async fn check_table_diffs<T>(client: &dyn Client) -> Result<bool, WeldsError>
where
    T: Send + HasSchema,
    <T as HasSchema>::Schema: TableInfo + TableColumns,
{
    let diff = welds::check::schema::<T>(client).await?;
    for d in &diff {
        println!("{}", d);
    }

    // let look over just the columns that have changed types
    for d in &diff {
        if let Some(changed) = d.kind.as_changed() {
            if changed.type_changed() {
                println!("{}", changed);
            }
        }
    }

    Ok(!diff.is_empty())
}

pub async fn check_db_integrity(client: &dyn Client) -> Result<bool, WeldsError> {
    let users = check_table_diffs::<User>(client).await?;
    let listens = check_table_diffs::<Listen>(client).await?;
    let messybrainz = check_table_diffs::<MessybrainzSubmission>(client).await?;

    Ok(users || listens || messybrainz)
}
