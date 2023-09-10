use reqwest::Method;

use crate::{
    common::{MercadoPagoError, MercadoPagoRequestError},
    API_BASE_URL,
};

/// Client for Mercado Pago
pub struct MercadoPagoClient {
    access_token: String,
    client_http: reqwest::Client,
}

impl MercadoPagoClient {
    /// Request builder that set API url and token
    ///
    /// # Arguments
    ///
    /// * `method` - Http method
    /// * `path` - Also called route. Is the path from API.
    ///
    /// # Example
    /// ```
    /// let client = MercadoPagoClientBuilder::create_with_access_token("SOME_ACCESS_TOKEN");
    ///
    /// client.start_request(request::Method::POST, "/v1/payment_methods")
    /// ```
    pub fn start_request(
        &self,
        method: reqwest::Method,
        path: impl ToString,
    ) -> reqwest::RequestBuilder {
        self.client_http
            .request(method, format!("{API_BASE_URL}{}", path.to_string()))
            .bearer_auth(&self.access_token)
    }

    ///Check if credentials (`access_token`) are valid
    pub async fn check_credentials(&self) -> Result<(), MercadoPagoRequestError> {
        let response = self
            .start_request(Method::GET, "/v1/payment_methods")
            .send()
            .await?;

        match response.status().as_u16() {
            200 => Ok(()),
            _ => Err(MercadoPagoRequestError::MercadoPago(
                response.json::<MercadoPagoError>().await?,
            )),
        }
    }
}

/// Builder for [`MercadoPagoClient`]
pub struct MercadoPagoClientBuilder {}

impl MercadoPagoClientBuilder {
    /// Create a [`MercadoPagoClient`] with the access_token
    pub fn create_with_access_token(access_token: impl ToString) -> MercadoPagoClient {
        MercadoPagoClient {
            access_token: access_token.to_string(),
            client_http: reqwest::Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MercadoPagoClientBuilder;

    #[tokio::test]
    async fn create_client_with_access_token() {
        dotenvy::dotenv().ok();

        let client = MercadoPagoClientBuilder::create_with_access_token(
            std::env::var("MERCADO_PAGO_ACCESS").unwrap(),
        );

        assert!(client.check_credentials().await.is_ok())
    }
}
