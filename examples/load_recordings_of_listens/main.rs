use std::{fs::{self, File}, sync::Arc};

use listenbrainz::raw::Client;
use musicbrainz_db_lite::{
    api::listenbrainz::listen_collection::SaveListenPayload,
    database::create_database,
    models::{
        listenbrainz::listen::Listen,
        musicbrainz::{recording::Recording, user::User},
    }, Error,
};
use tokio_stream::{self as stream};
use welds::{connections::sqlite::SqliteClient, WeldsError};

/// Connect and setup a DB to test on
pub async fn setup_file_database() -> Result<SqliteClient, Error> {
    if std::fs::exists("./examples/load_recordings_of_listens/db.db").unwrap() {
        fs::remove_file("./examples/load_recordings_of_listens/db.db").unwrap();
    }

    File::create_new("./examples/load_recordings_of_listens/db.db").unwrap();
    let client = welds::connections::sqlite::connect("sqlite:./examples/load_recordings_of_listens/db.db").await?;
    create_database(&client).await?;
    Ok(client)
}

#[tokio::main]
async fn main() {
    let client = Arc::new(setup_file_database().await.unwrap());

    // Get some listens
    let lb_client = Client::new();
    lb_client
        .user_listens("RustyNova", None, Some(1726041017), Some(500))
        .unwrap()
        .payload
        .save_listen_payload_in_transaction(client.as_ref(), 1726041017, 500)
        .await
        .unwrap();


    // Now get the missing recordings
    let recordings = Listen::get_unfetched_recordings_of_user(
        &client,
        &User::find_by_name(client.as_ref(), "RustyNova")
            .await
            .unwrap()
            .unwrap(),
    )
    .await
    .unwrap();

    let mut result = Vec::new();
    for recording in recordings {
        result.push(Recording::fetch_all_and_save(&client, &recording).await.unwrap());
    }
    
    // The recordings are now ready
}
