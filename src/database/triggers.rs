use welds::Client;
use welds::WeldsError;

pub async fn create_listenbrainz_triggers(client: &dyn Client) -> Result<(), WeldsError> {
    client
        .execute(
            "

    ",
            &[],
        )
        .await?;
    Ok(())
}

//    CREATE TRIGGER `set_mbid_in_recording_redirect` BEFORE INSERT ON `msid_mapping` BEGIN
//        -- This set the mbid in the redirect table
//        INSERT OR IGNORE INTO `recording_gid_redirect` VALUES (NULL, new.recording_mbid, NULL);
//    END
