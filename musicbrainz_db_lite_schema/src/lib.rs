use sqlx::Connection;
use tables::create_listenbrainz_tables;
use tables::create_musicbrainz_tables;
use tables::listenbrainz::generate_listenbrainz_database;

pub mod tables;

pub async fn create_database(conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    let mut trans: sqlx::Transaction<'_, sqlx::Sqlite> = conn.begin().await?;

    create_musicbrainz_tables(&mut trans).await?;
    create_listenbrainz_tables(&mut trans).await?;
    generate_listenbrainz_database(&mut trans).await?;
    trans.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::create_database;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_generate_schema() {
        // Set up db file
        if std::fs::exists("./schema.db").unwrap() {
            fs::remove_file("./schema.db").unwrap();
        }

        File::create_new("./schema.db").unwrap();
        let sql_pool = sqlx::SqlitePool::connect_lazy("sqlite:./schema.db").unwrap();

        // Create Database
        create_database(&mut sql_pool.acquire().await.unwrap())
            .await
            .unwrap();

        //assert!(check_db_integrity(&client).await.is_ok_and(|v| v));

        let out = Command::new("sqlite3")
            .arg("./schema.db")
            .arg(".dump ")
            .output()
            .unwrap();

        File::create("./schema.sql")
            .unwrap()
            .write_all(&out.stdout)
            .unwrap();
    }
}
