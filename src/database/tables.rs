use welds::connections::sqlite::SqliteClient;
use welds::Client;
use welds::WeldsError;

pub async fn create_database(client: &SqliteClient) -> Result<(), WeldsError> {
    client.execute("PRAGMA foreign_keys = OFF; 
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS \"users\" (
    `id` INTEGER PRIMARY KEY UNIQUE NOT NULL,
    `name` TEXT UNIQUE NOT NULL
) STRICT;
CREATE TABLE `messybrainz_submission` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `msid` TEXT UNIQUE NOT NULL,
    `recording` TEXT NOT NULL,
    `artist_credit` TEXT NOT NULL,
    `release` TEXT,
    `track_number` TEXT,
    `duration` INTEGER
) STRICT;
CREATE TABLE IF NOT EXISTS \"msid_mapping\" (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `recording_msid` TEXT REFERENCES `messybrainz_submission`(`msid`),
    `recording_mbid` TEXT,
    `user` INTEGER NOT NULL REFERENCES `users`(`id`)
) STRICT;
CREATE TABLE IF NOT EXISTS \"listens\" (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `listened_at` INTEGER NOT NULL,
    `user` TEXT NOT NULL REFERENCES `users`(`name`),
    `recording_msid` TEXT NOT NULL REFERENCES `messybrainz_submission`(`msid`),
    `data` TEXT
) STRICT;
CREATE UNIQUE INDEX `idx_msid_mapping_2` ON `msid_mapping` (`recording_msid`, `recording_mbid`, `user`);
CREATE UNIQUE INDEX `idx_listens` ON `listens` (`listened_at`, `user`, `recording_msid`);
COMMIT;
PRAGMA foreign_keys = ON;", &[]).await?;
    Ok(())
}
