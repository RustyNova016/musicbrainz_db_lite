#![no_main]

use libfuzzer_sys::fuzz_target;

use musicbrainz_db_lite::api::SaveToDatabase;
use musicbrainz_db_lite::entity::recording::Recording as MBRecording;
use musicbrainz_db_lite::DBClient;

fuzz_target!(|data: MBRecording| {
    run(data)
});

#[tokio::main]
async fn run(data: MBRecording) {
    let client = DBClient::connect_in_memory_and_create().await.unwrap();
    let conn = &mut *client.connection.acquire_guarded().await;
    match data.save(conn).await {
        Ok(_) => {}
        Err(err) =>  {
            match err {
                musicbrainz_db_lite::Error::MissingMBID(_) => {},
                _ => panic!("{}", err),
            }
        }
    }
}