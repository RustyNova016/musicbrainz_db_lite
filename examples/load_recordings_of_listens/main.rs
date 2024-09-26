use std::{
    fs::{self, File},
    sync::Arc,
};

use listenbrainz::raw::Client;
use musicbrainz_db_lite::{
    api::listenbrainz::listen_collection::SaveListenPayload,
    database::{client::DBClient, create_database},
    models::{
        listenbrainz::listen::Listen,
        musicbrainz::{recording::Recording, user::User},
    },
    Error,
};
use tokio_stream::{self as stream};
use welds::{connections::sqlite::SqliteClient, WeldsError};

/// Connect and setup a DB to test on
pub async fn setup_file_database() -> Result<SqliteClient, Error> {
    if std::fs::exists("./examples/load_recordings_of_listens/db.db").unwrap() {
        fs::remove_file("./examples/load_recordings_of_listens/db.db").unwrap();
    }

    File::create_new("./examples/load_recordings_of_listens/db.db").unwrap();
    let client =
        welds::connections::sqlite::connect("sqlite:./examples/load_recordings_of_listens/db.db")
            .await?;
    create_database(&client).await?;
    Ok(client)
}

#[tokio::main]
async fn main() {
    setup_file_database().await.unwrap();
    let client = DBClient::connect("./examples/load_recordings_of_listens/db.db")
        .await
        .unwrap();

    // Get some listens
    println!("Getting some listens");
    let lb_client = Client::new();
    lb_client
        .user_listens("RustyNova", None, Some(1726041017), Some(500))
        .inspect(|_| println!("Saving listens"))
        .unwrap()
        .payload
        .save_listen_payload_in_transaction(client.as_welds_client(), 1726041017, 500)
        .await
        .unwrap();

    // Now get the missing recordings
    println!("Getting the recordings");
    let recordings = Listen::get_unfetched_recordings_of_user(
        client.as_welds_client(),
        &User::find_by_name(client.as_welds_client().as_sqlx_pool(), "RustyNova")
            .await
            .unwrap()
            .unwrap(),
    )
    .await
    .unwrap();

    println!("Fetching {} recordings", recordings.len());

    let mut result = Vec::new();
    for recording in recordings {
        println!("Looking up: {recording}");

        let conn = &mut *client.as_sqlx_pool().acquire().await.unwrap();

        let recording = Recording::fetch_all_and_save(conn, &recording)
            .await
            .unwrap();

        println!(
            "Got: {} by {}",
            recording.title,
            recording.get_artist_credits(conn).await.unwrap().unwrap()
        );

        result.push(recording);
    }

    // The recordings are now ready
}
