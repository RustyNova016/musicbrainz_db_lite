PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE `artists` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        `mbid` TEXT UNIQUE NOT NULL ,
        `name` TEXT NOT NULL,
        `sort_name` TEXT NOT NULL,
        `disambiguation` TEXT NOT NULL,
        `country` TEXT,
        `annotation` TEXT
    ) STRICT;
CREATE TABLE `artist_credits_item` (
        `artist_credit` INTEGER REFERENCES `artist_credits` (`id`),
        `position` INTEGER NOT NULL,
        `name` TEXT NOT NULL,
        `artist_gid` TEXT NOT NULL REFERENCES `artists_gid_redirect` (`gid`),
        `join_phrase` TEXT NOT NULL,

        PRIMARY KEY (`artist_credit`, `position`)
    ) STRICT;
CREATE TABLE `artist_credits` (`id` INTEGER PRIMARY KEY AUTOINCREMENT) STRICT;
CREATE TABLE `artists_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `artists`(`id`),
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `recordings` (
            `id` INTEGER PRIMARY KEY NOT NULL, 
            `mbid` TEXT UNIQUE NOT NULL, 
            `title` TEXT NOT NULL, 
            `length` INTEGER, 
            `disambiguation` TEXT, 
            `annotation` TEXT,

            -- Foreign keys
            `artist_credit` INTEGER REFERENCES `artist_credits`(`id`)
        ) STRICT;
CREATE TABLE `recordings_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `recordings`(`id`),
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS "users" (
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
CREATE TABLE IF NOT EXISTS "msid_mapping" (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `recording_msid` TEXT NOT NULL REFERENCES `messybrainz_submission`(`msid`),
    `recording_mbid` TEXT NOT NULL REFERENCES `recordings_gid_redirect`(`gid`),
    `user` INTEGER NOT NULL REFERENCES `users`(`id`)
) STRICT;
CREATE TABLE IF NOT EXISTS "listens" (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `listened_at` INTEGER NOT NULL,
    `user` TEXT NOT NULL REFERENCES `users`(`name`),
    `recording_msid` TEXT NOT NULL REFERENCES `messybrainz_submission`(`msid`),
    `data` TEXT
) STRICT;
DELETE FROM sqlite_sequence;
CREATE TRIGGER `trigger_after_insert_artists` AFTER INSERT ON `artists` FOR EACH ROW BEGIN
    INSERT OR REPLACE INTO artists_gid_redirect VALUES (new.mbid, new.id, 0);
END;
CREATE TRIGGER `trigger_after_insert_recordings` AFTER INSERT ON `recordings` FOR EACH ROW BEGIN
    INSERT OR REPLACE INTO recordings_gid_redirect VALUES (new.mbid, new.id, 0);
END;
CREATE UNIQUE INDEX `idx_msid_mapping_2` ON `msid_mapping` (`recording_msid`, `recording_mbid`, `user`);
CREATE UNIQUE INDEX `idx_listens` ON `listens` (`listened_at`, `user`, `recording_msid`);
COMMIT;
