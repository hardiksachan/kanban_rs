use serde::Serialize;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Model errors
    TicketDeleteFailIdNotFound { id: Uuid },
    //// Auth errors
    //AuthFailNoAuthTokenCookie,
    //AuthFailTokenWrongFormat,
    //AuthFailedCtxNotInRequestExt,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
