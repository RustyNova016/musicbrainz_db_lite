CREATE TABLE `artists_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `artist` INTEGER NOT NULL REFERENCES `artists`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_artist` ON `artists_genre` (`name`, `artist`);
CREATE TABLE `labels_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `label` INTEGER NOT NULL REFERENCES `labels`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_label` ON `labels_genre` (`name`, `label`);
CREATE TABLE `recordings_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `recording` INTEGER NOT NULL REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_recording` ON `recordings_genre` (`name`, `recording`);
CREATE TABLE `release_groups_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `release_group` INTEGER NOT NULL REFERENCES `release_groups`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_release_group` ON `release_groups_genre` (`name`, `release_group`);
CREATE TABLE `releases_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `release` INTEGER NOT NULL REFERENCES `releases`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_release` ON `releases_genre` (`name`, `release`);
CREATE TABLE `works_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `work` INTEGER NOT NULL REFERENCES `works`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_work` ON `works_genre` (`name`, `work`);