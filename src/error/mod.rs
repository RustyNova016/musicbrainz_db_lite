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

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("The MBID {0} wasn't found in Musicbrainz, but found in the local database. Hint: The upstream MBID might have been deleted")]
    UnknownUpstream(String),

    // Temporary errors
    #[error("Tried to insert a relation that is not yet implemented")]
    RelationNotImplemented, //TODO: Remove when all relations are implemented
}

impl From<welds::connections::Error> for Error {
    fn from(value: welds::connections::Error) -> Self {
        Self::WeldsError(value.into())
    }
}
