pub(super) fn generate_redirect_table(table_name: &str) -> String {
    format!("CREATE TABLE `{table_name}_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `{table_name}`(`id`),
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
 
 CREATE TRIGGER IF NOT EXISTS `trigger_after_insert_{table_name}` AFTER INSERT ON `{table_name}` FOR EACH ROW BEGIN
    INSERT OR REPLACE INTO {table_name}_gid_redirect VALUES (new.mbid, new.id, 0);
END;")
}
