use std::fs::{self, File};

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
use welds::connections::sqlite::SqliteClient;

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
    //let mut clog = colog::default_builder();
    //clog.filter(None, log::LevelFilter::Trace);
    //clog.init();

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
    let recordings = Listen::get_recordings_of_user(
        client.as_welds_client(),
        &User::find_by_name(
            &mut *client
                .as_welds_client()
                .as_sqlx_pool()
                .acquire()
                .await
                .unwrap(),
            "RustyNova",
        )
        .await
        .unwrap()
        .unwrap(),
    )
    .await
    .unwrap();

    println!("Fetching {} recordings", recordings.len());

    let mut result = Vec::new();
    for recording in recordings {
        println!();
        println!(" === New Recording ===");
        println!("Looking up: {recording}");

        //let mut trans = client.begin_transaction().await.unwrap();
        //let conn = &mut *trans;
        let conn = &mut *client.as_sqlx_pool().acquire().await.unwrap();

        let recording = Recording::get_or_fetch_as_complete_from_mbid(conn, &recording)
            .await
            .unwrap()
            .unwrap();

        println!("Getting Releases...");
        let releases = recording.get_releases_or_fetch(conn).await.unwrap();

        for release in releases {
            println!("Getting medias...");
            let _medias = release.get_medias_or_fetch(conn).await.unwrap();
        }

        println!(
            "Got: {} by {}",
            recording.title,
            recording.get_artist_credits_or_fetch(conn).await.unwrap()
        );

        let _ = conn;
        //trans.commit().await.unwrap();
        result.push(recording);
    }

    // The recordings are now ready
}
