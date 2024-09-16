use std::fs;
use std::fs::File;

use musicbrainz_db_lite::database::create_database;
use musicbrainz_db_lite::models::listenbrainz::listen::selects::ListenMappingFilter;
use musicbrainz_db_lite::models::listenbrainz::listen::selects::ListenQuery;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    if fs::exists("./tests/test_db.db").unwrap() {
        fs::remove_file("./tests/test_db.db").unwrap();
    }

    File::create_new("./tests/test_db.db").unwrap();
    let client = welds::connections::sqlite::connect("sqlite:./tests/test_db.db")
        .await
        .unwrap();
    create_database(&client).await.unwrap();

    let query = ListenQuery {
        user: "RustyNova".to_string(),
        unmapped: ListenMappingFilter::Any,
        fetch_latest_listens: true,
    };

    let res = query.run(&client).await.unwrap();
    println!("{}", res.len())
}
