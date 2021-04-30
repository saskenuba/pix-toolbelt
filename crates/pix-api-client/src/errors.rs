use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type ApiResult<T> = Result<T, PixError>;

#[derive(Debug, Serialize, Deserialize)]
struct GenericErrorMessage {
    nome: String,
    mensagem: String,
    errors: Option<Vec<Erros>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Erros {
    chave: String,
    caminho: String,
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

    #[error("There is something wrong with the payload this library sent.")]
    PayloadError,

    #[error("`{0}`")]
    Other(String),

    #[error(transparent)]
    NonCompliantResponse(#[from] serde_json::Error),

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),
}
