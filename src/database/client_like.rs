use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::Executor;
use sqlx::SqliteExecutor;

use crate::utils::sqlx_utils::get_exec::GetExecutor;

pub trait ClientLike<'c>: GetExecutor<'c> {
    fn get_mb_client(self) -> &'c MusicBrainzClient;
}