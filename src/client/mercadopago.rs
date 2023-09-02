use reqwest::Method;
use thiserror::Error;

use crate::API_BASE_URL;

pub struct MercadoPagoClient {
    access_token: String,
    client_http: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum CredentialsError {
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
}

impl MercadoPagoClient {
    pub fn create_with_access_token(access_token: impl ToString) -> Self {
        Self {
            access_token: access_token.to_string(),
            client_http: reqwest::Client::new(),
        }
    }

    pub fn start_request(
        &self,
        method: reqwest::Method,
        path: impl ToString,
    ) -> reqwest::RequestBuilder {
        self.client_http
            .request(method, format!("{API_BASE_URL}{}", path.to_string()))
            .bearer_auth(&self.access_token)
    }

    pub async fn check_credentials(&self) -> Result<(), CredentialsError> {
        let response = self
            .start_request(Method::GET, "/v1/payment_methods")
            .send()
            .await?;

        match response.status().as_u16() {
            200 => Ok(()),
            400 => Err(CredentialsError::BadRequest),
            401 => Err(CredentialsError::Unauthorized),
            404 => Err(CredentialsError::NotFound),
            _ => Err(CredentialsError::InternalServerError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MercadoPagoClient;

    #[tokio::test]
    async fn create_client() {
        dotenvy::dotenv().ok();

        let client = MercadoPagoClient::create_with_access_token(
            std::env::var("MERCADO_PAGO_ACCESS").unwrap(),
        );

        assert!(client.check_credentials().await.is_ok())
    }
}
