use sqlx::query;
use sqlx::SqlitePool;

pub async fn create_listenbrainz_triggers(client: &SqlitePool) -> Result<(), sqlx::Error> {
    query!(r#"CREATE TRIGGER IF NOT EXISTS `trigger_after_insert_recordings` AFTER INSERT ON `recordings` FOR EACH ROW BEGIN
    INSERT OR REPLACE INTO recordings_gid_redirect VALUES (new.mbid, new.id, 0);
END;"#).execute(client).await?;

    Ok(())
}

//    CREATE TRIGGER `set_mbid_in_recording_redirect` BEFORE INSERT ON `msid_mapping` BEGIN
//        -- This set the mbid in the redirect table
//        INSERT OR IGNORE INTO `recordings_gid_redirect` VALUES (NULL, new.recording_mbid, NULL);
//    END
