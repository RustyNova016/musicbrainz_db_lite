use sqlx::query;
use sqlx::SqlitePool;


//    CREATE TRIGGER `set_mbid_in_recording_redirect` BEFORE INSERT ON `msid_mapping` BEGIN
//        -- This set the mbid in the redirect table
//        INSERT OR IGNORE INTO `recordings_gid_redirect` VALUES (NULL, new.recording_mbid, NULL);
//    END
