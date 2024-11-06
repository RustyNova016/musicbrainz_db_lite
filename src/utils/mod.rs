pub mod date_utils;
pub mod macros;
pub mod querry_builder;
pub mod sqlx_utils;
use welds::model_traits::HasSchema;
use welds::model_traits::TableColumns;
use welds::model_traits::TableInfo;
use welds::Client;
use welds::WeldsError;

use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::listen_user_metadata::MessybrainzSubmission;
use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::recording::redirect::RecordingGidRedirect;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::user::User;

pub mod extensions;
pub mod welds_utils;

// Return True if the schema is correct
pub async fn check_table_schema<T>(client: &dyn Client) -> Result<bool, WeldsError>
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

    Ok(diff.is_empty())
}

// Return True if the schema is correct
pub async fn check_db_integrity(client: &dyn Client) -> Result<bool, WeldsError> {
    let users = check_table_schema::<User>(client).await?;
    let listens = check_table_schema::<Listen>(client).await?;
    let messybrainz = check_table_schema::<MessybrainzSubmission>(client).await?;
    let record_redirect = check_table_schema::<RecordingGidRedirect>(client).await?;
    let recordings = check_table_schema::<Recording>(client).await?;
    let artists = check_table_schema::<Artist>(client).await?;

    Ok(users && listens && messybrainz && record_redirect && recordings && artists)
}
