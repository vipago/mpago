use reqwest::Method;

use crate::{
    common::{MercadoPagoError, MercadoPagoRequestError},
    API_BASE_URL,
};

/// Client for Mercado Pago
pub struct MercadoPagoClient {
    access_token: String,
    client_http: reqwest::Client,
    base_url: String,
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
    /// use mpago::client::MercadoPagoClientBuilder;
    ///
    /// let client = MercadoPagoClientBuilder::builder("SOME_ACCESS_TOKEN").build();
    ///
    /// client.start_request(request::Method::POST, "/v1/payment_methods")
    /// ```
    pub(crate) fn start_request(
        &self,
        method: Method,
        path: impl Into<String>,
    ) -> reqwest::RequestBuilder {
        self.client_http
            .request(method, format!("{}{}", self.base_url, path.into()))
            .bearer_auth(&self.access_token)
    }

    /// Check if credentials (`access_token`) are valid
    ///
    /// # Errors
    /// This returns an error if the credentials are invalid, or if it couldn't complete the
    /// request
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
pub struct MercadoPagoClientBuilder {
    access_token: String,
    base_url: String,
}

impl MercadoPagoClientBuilder {
    /// Create a new client builder
    #[must_use]
    pub fn builder(access_token: impl Into<String>) -> Self {
        MercadoPagoClientBuilder {
            access_token: access_token.into(),
            base_url: API_BASE_URL.to_string(),
        }
    }

    /// Make the client use a custom base url.
    #[must_use]
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Build a [`MercadoPagoClient`] with the current builder.
    #[must_use]
    pub fn build(self) -> MercadoPagoClient {
        MercadoPagoClient {
            access_token: self.access_token,
            base_url: self.base_url,
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

        let client =
            MercadoPagoClientBuilder::builder(std::env::var("MERCADO_PAGO_ACCESS").unwrap())
                .build();

        assert!(client.check_credentials().await.is_ok());
    }
}
