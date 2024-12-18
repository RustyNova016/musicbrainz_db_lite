use crate::database::client::DBClient;
use crate::models::musicbrainz::recording::Recording;

pub async fn assert_has_recording_recording_rel(
    conn: &mut sqlx::SqliteConnection,
    left: &str,
    right: &str,
    relation_type: &str,
) {
    let left = Recording::get_or_fetch(conn, left).await.unwrap().unwrap();
    let right = Recording::get_or_fetch(conn, right).await.unwrap().unwrap();

    let rels = left.get_recording_relations(conn).await.unwrap();

    println!("Rels {:#?}", rels);
    for rel in rels {
        if rel.relation_type == relation_type {
            let ent = rel.get_entity_1_as_right(conn).await.unwrap();

            if ent.mbid == right.mbid {
                return;
            }

            println!("MBID missmatch")
        }
    }

    panic!("Relation not found")
}

#[tokio::test]
#[serial_test::serial]
async fn should_have_relation() {
    let client = DBClient::create_test_file_database().await.unwrap();
    let conn = &mut *client.connection.acquire().await.unwrap();
    assert_has_recording_recording_rel(
        conn,
        "365f441b-59a6-4d7c-9c1a-5afd52741319",
        "0d6d588e-e568-4e43-b119-0c0e79dc3a43",
        "remix",
    )
    .await
}
