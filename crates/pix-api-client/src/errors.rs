use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type ApiResult<T> = Result<T, PixError>;

#[derive(Serialize, Deserialize)]
struct GenericErrorMessage {
    nome: String,
    mensagem: String,
}

#[derive(Debug, Error)]
pub enum PixError {
    /// Error 401
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// Error 401
    #[error("Access token is expired. Renew it")]
    ExpiredToken(String),

    #[error("Response differs from spec")]
    NonCompliantResponse,

    #[error("There is something wrong with the payload this library sent.")]
    PayloadError,

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),
}
