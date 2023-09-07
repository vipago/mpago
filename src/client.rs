use reqwest::Method;

use crate::{
    common::{MercadoPagoError, MercadoPagoRequestError},
    API_BASE_URL,
};

pub struct MercadoPagoClient {
    access_token: String,
    client_http: reqwest::Client,
}

impl MercadoPagoClient {
    pub fn start_request(
        &self,
        method: reqwest::Method,
        path: impl ToString,
    ) -> reqwest::RequestBuilder {
        self.client_http
            .request(method, format!("{API_BASE_URL}{}", path.to_string()))
            .bearer_auth(&self.access_token)
    }

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

pub struct MercadoPagoClientBuilder {}

impl MercadoPagoClientBuilder {
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
