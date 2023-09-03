use thiserror::Error;

pub mod client;
pub mod oauth;

pub static API_BASE_URL: &str = "https://api.mercadopago.com";

#[derive(Debug, Error)]
pub enum APIError {
    #[error("Credentials are requires")]
    BadRequest,
    #[error("Credentials are invalid")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Error while sending request: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Authorization failed: {0}")]
    AuthorizationError(String),
}
