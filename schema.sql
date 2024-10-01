PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE `artists` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        `mbid` TEXT UNIQUE NOT NULL ,
        `name` TEXT NOT NULL,
        `sort_name` TEXT NOT NULL,
        `disambiguation` TEXT NOT NULL,
        `country` TEXT,
        `annotation` TEXT,
        `full_update_date` INTEGER
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
    `new_id` TEXT REFERENCES `artists`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `recordings` (
            `id` INTEGER PRIMARY KEY NOT NULL, 
            `mbid` TEXT UNIQUE NOT NULL, 
            `title` TEXT NOT NULL, 
            `length` INTEGER, 
            `disambiguation` TEXT, 
            `annotation` TEXT,

            `full_update_date` INTEGER,

            -- Foreign keys
            `artist_credit` INTEGER REFERENCES `artist_credits`(`id`)
        ) STRICT;
CREATE TABLE `recordings_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `releases` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `mbid` TEXT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `date` INTEGER,
                `country` TEXT,
                `quality` TEXT,
                `status` TEXT,
                `barcode` TEXT,
                `disambiguation` TEXT,
                `packaging` TEXT,
                `annotation` TEXT,

                `full_update_date` INTEGER,

                -- Foreign Keys
                `artist_credit` INTEGER REFERENCES `artist_credits` (`id`)
            ) STRICT;
CREATE TABLE `medias` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `track_count` INTEGER NOT NULL,
                `title` TEXT,
                `position` INTEGER,
                `disc_count` INTEGER,
                `format` TEXT,

                -- Foreign Keys
                `release` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `tracks` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `gid` TEXT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `number` TEXT NOT NULL,
                `position` INTEGER NOT NULL,

                -- Foreign Keys
                `media` INTEGER NOT NULL REFERENCES `medias` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `recording` TEXT REFERENCES `recordings_gid_redirect` (`gid`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `releases_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `releases`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
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
CREATE TABLE `metadata` (
    schema_version INTEGER NOT NULL
) STRICT;
DELETE FROM sqlite_sequence;
CREATE TRIGGER `trigger_after_insert_artists` AFTER INSERT ON `artists` FOR EACH ROW BEGIN
    INSERT INTO artists_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE TRIGGER `trigger_after_insert_recordings` AFTER INSERT ON `recordings` FOR EACH ROW BEGIN
    INSERT INTO recordings_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE UNIQUE INDEX `idx_medias` ON `medias` (`release`, `position`);
CREATE UNIQUE INDEX `idx_tracks` ON `tracks` (`media`, `position`);
CREATE TRIGGER `trigger_after_insert_releases` AFTER INSERT ON `releases` FOR EACH ROW BEGIN
    INSERT INTO releases_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE UNIQUE INDEX `idx_msid_mapping_2` ON `msid_mapping` (`recording_msid`, `recording_mbid`, `user`);
CREATE UNIQUE INDEX `idx_listens` ON `listens` (`listened_at`, `user`, `recording_msid`);
COMMIT;
