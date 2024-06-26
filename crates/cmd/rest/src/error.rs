use axum::{http::StatusCode, response::IntoResponse};
use derive_more::From;
use serde::Serialize;
use tracing::instrument;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    #[from]
    Ticket(tickets::Error),

    // Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailedCtxNotInRequestExt,
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            Self::AuthFailTokenWrongFormat
            | Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailedCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            Self::Ticket(ref ticket_error) => match ticket_error {
                tickets::Error::TicketDeleteFailIdNotFound { .. }
                | tickets::Error::InvalidTicketId { .. } => {
                    (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
                }
            },
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl IntoResponse for Error {
    #[instrument]
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
