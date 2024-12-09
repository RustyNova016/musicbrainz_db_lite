-- Add migration script here

CREATE TABLE `tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER
            ) STRICT;
CREATE INDEX `idx_tag` ON `tag` (`name`);