#[allow(unused_imports)]
use crate::{client::MercadoPagoClient, payments::types::PaymentCreateOptions};
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

/// Used for solving json responses from Mercado Pago. If there is an error, [`MercadoPagoRequestError`] handles both the request body errors from Mercado Pago and Reqwest errors.
pub(crate) async fn resolve_json<T>(response: Response) -> Result<T, MercadoPagoRequestError>
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

/// Enum to handle Mercado Pago errors and Reqwest errors
#[derive(Error, Debug)]
pub enum MercadoPagoRequestError {
    #[error("{0}")]
    Request(#[from] reqwest::Error),
    #[error("MercadoPago Error: {0:?}")]
    MercadoPago(MercadoPagoError),
}

/// Body sent by Mercado Pago when there is something wrong
#[derive(Deserialize, Serialize, Debug)]
pub struct MercadoPagoError {
    /// Resume of the error
    pub message: String,
    /// Identification of the error. It usually has to do with the HTTP status.
    pub error: String,
    /// HTTP Status
    pub status: u16,
    /// A list of causes of the error
    pub cause: Vec<MercadoPagoErrorCause>,
}

/// Error cause
#[derive(Deserialize, Serialize, Debug)]
pub struct MercadoPagoErrorCause {
    /// Code related to Mercado Pago errors. It should be referenced in the documentation for each route.
    pub code: u32,
    /// Brief description of the error
    pub description: String,
    /// Date when error occurs
    ///
    /// ## Important Note
    /// This field is returning a date with some UUID together. It should be fixed later.
    ///
    /// `"08-09-2023T22:33:32UTC;c68defe3-5b82-4775-bc45-4349daa88bb0"`
    #[serde(rename = "data")]
    pub date: String,
}

/// Function to create client for testing
#[cfg(test)]
#[must_use]
pub(crate) fn create_test_client() -> MercadoPagoClient {
    use crate::client::MercadoPagoClientBuilder;

    dotenvy::dotenv().ok();

    MercadoPagoClientBuilder::builder(std::env::var("MERCADO_PAGO_ACCESS").unwrap()).build()
}

/// Function to return test payment option
#[cfg(test)]
#[must_use]
pub(crate) fn get_test_payment_options() -> PaymentCreateOptions {
    use crate::{payer::Payer, payments::types::PaymentMethodId, Decimal};

    PaymentCreateOptions {
        description: Some("Test".to_string()),
        payer: Payer {
            email: "test@testmail.uk".to_string(),
            first_name: None,
            last_name: None,
            entity_type: None,
            id: None,
            identification: None,
            r#type: None,
        },
        transaction_amount: Decimal::new(10, 0),
        payment_method_id: PaymentMethodId::Pix,
        ..Default::default()
    }
}
