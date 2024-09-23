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
    if std::fs::exists("./examples/load_all_listens_of_user/db.db").unwrap() {
        fs::remove_file("./examples/load_all_listens_of_user/db.db").unwrap();
    }

    File::create_new("./examples/load_all_listens_of_user/db.db").unwrap();
    let client =
        welds::connections::sqlite::connect("sqlite:./examples/load_all_listens_of_user/db.db")
            .await?;
    create_database(&client).await?;
    Ok(client)
}

#[tokio::main]
async fn main() {
    setup_file_database().await.unwrap();
    let client = DBClient::connect("./examples/load_all_listens_of_user/db.db")
        .await
        .unwrap();

    use std::time::Instant;
    let now = Instant::now();
    Listen::fetch_latest_listens_of_user(client.as_welds_client(), "RustyNova")
        .await
        .unwrap();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
