use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    //#[error(transparent)]
    //ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    MusicbrainzError(#[from] musicbrainz_rs_nova::Error),

    #[error(transparent)]
    ListenbrainzError(#[from] listenbrainz::Error),

    #[error(transparent)]
    WeldsError(#[from] welds::WeldsError),

    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),
}

impl From<welds::connections::Error> for Error {
    fn from(value: welds::connections::Error) -> Self {
        Self::WeldsError(value.into())
    }
}
