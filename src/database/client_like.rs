use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::Executor;
use sqlx::SqliteExecutor;

pub trait ClientLike {
    fn get_mb_client(&self) -> &MusicBrainzClient;

    fn get_executor(&self) -> impl SqliteExecutor;
}