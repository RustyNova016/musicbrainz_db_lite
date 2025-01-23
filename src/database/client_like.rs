use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::Executor;
use sqlx::SqliteExecutor;

pub trait ClientLike<'c> {
    fn get_mb_client(self) -> &'c MusicBrainzClient;

    async fn get_executor(self) -> Result<impl sqlx::SqliteExecutor<'c>, crate::Error>;
}