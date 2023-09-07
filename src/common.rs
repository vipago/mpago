use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

#[allow(unused_imports)]
use crate::{client::MercadoPagoClient, payments::types::PaymentCreateOptions};

pub async fn resolve_json<T>(response: Response) -> Result<T, MercadoPagoRequestError>
where
    T: DeserializeOwned,
{
    match response.status().as_u16() {
        200..=299 => Ok(response.json::<T>().await?),
        _ => Err(MercadoPagoRequestError::MercadoPago(
            response.json::<MercadoPagoError>().await?,
        )),
    }
}

#[derive(Error, Debug)]
pub enum MercadoPagoRequestError {
    #[error("{0}")]
    Request(#[from] reqwest::Error),
    #[error("MercadoPago Error: {0:?}")]
    MercadoPago(MercadoPagoError),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MercadoPagoError {
    pub message: String,
    pub error: String,
    pub status: u16,
    pub cause: Vec<MercadoPagoErrorCause>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MercadoPagoErrorCause {
    pub code: u32,
    pub description: String,
    #[serde(rename = "data")]
    pub date: String,
}

#[cfg(test)]
pub fn create_test_client() -> MercadoPagoClient {
    use crate::client::MercadoPagoClientBuilder;

    dotenvy::dotenv().ok();

    MercadoPagoClientBuilder::create_with_access_token(
        std::env::var("MERCADO_PAGO_ACCESS").unwrap(),
    )
}

#[cfg(test)]
pub fn get_test_payment_options() -> PaymentCreateOptions {
    use crate::{payer::Payer, payments::types::PaymentMethodId};

    PaymentCreateOptions {
        description: "Test".to_string(),
        payer: Payer {
            email: "test@testmail.uk".to_string(),
            first_name: None,
            last_name: None,
            entity_type: None,
            id: None,
            identification: None,
            r#type: None,
        },
        transaction_amount: 10.0,
        payment_method_id: PaymentMethodId::Pix,
        ..Default::default()
    }
}
