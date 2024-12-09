-- Add migration script here

CREATE TABLE `genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER
            ) STRICT;

CREATE INDEX `idx_genre` ON `genre` (`name`);