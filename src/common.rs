#[allow(unused_imports)]
use crate::{client::MercadoPagoClient, payments::types::PaymentCreateOptions};
use iso_currency::Currency;
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use thiserror::Error;

/// Used for solving json responses from Mercado Pago. If there is an error, [`MercadoPagoRequestError`] handles both the request body errors from Mercado Pago and Reqwest errors.
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
    pub error: Option<String>,
    /// HTTP Status
    pub status: u16,
    /// A list of causes of the error
    pub cause: Option<Vec<MercadoPagoErrorCause>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResponse<T> {
    pub paging: Paging,
    pub results: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Paging {
    /// Total number of items in the charge.
    pub total: usize,
    /// The maximum number of entries to be returned.
    pub limit: usize,
    /// Total number of skiped items
    pub offset: usize,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq, Default)]
pub enum CurrencyId {
    ARS,
    #[default]
    BRL,
    CLP,
    MXN,
    COP,
    PEN,
    UYU,
    VES,
    MCN,
    BTC,
    USD,
    USDP,
    DCE,
    ETH,
    FDI,
    CDB,
    #[serde(other)]
    Unknown(String),
}

impl From<Currency> for CurrencyId {
    /// Parse ISO currency to `CurrencyId`
    fn from(value: Currency) -> Self {
        match value {
            Currency::ARS => Self::ARS,
            Currency::BRL => Self::BRL,
            Currency::CLP => Self::CLP,
            Currency::MXN => Self::MXN,
            Currency::COP => Self::COP,
            Currency::PEN => Self::PEN,
            Currency::UYU => Self::UYU,
            Currency::VES => Self::VES,
            Currency::USD => Self::USD,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

impl TryFrom<CurrencyId> for Currency {
    type Error = String;

    /// Try parse `CurrencyId` to ISO currency
    fn try_from(value: CurrencyId) -> Result<Self, Self::Error> {
        match value {
            CurrencyId::ARS => Ok(Currency::ARS),
            CurrencyId::BRL => Ok(Currency::BRL),
            CurrencyId::CLP => Ok(Currency::CLP),
            CurrencyId::MXN => Ok(Currency::MXN),
            CurrencyId::COP => Ok(Currency::COP),
            CurrencyId::PEN => Ok(Currency::PEN),
            CurrencyId::UYU => Ok(Currency::UYU),
            CurrencyId::VES => Ok(Currency::VES),
            CurrencyId::USD => Ok(Currency::USD),
            CurrencyId::Unknown(v) => v.parse::<Currency>().map_err(|e| e.to_string()),
            v => Err(format!("Unsupported currency: {v}")),
        }
    }
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
pub fn create_test_client() -> MercadoPagoClient {
    use crate::client::MercadoPagoClientBuilder;

    dotenvy::dotenv().ok();

    MercadoPagoClientBuilder::builder(std::env::var("MERCADO_PAGO_ACCESS").unwrap()).build()
}

/// Function to return test payment option
#[cfg(test)]
pub fn get_test_payment_options() -> PaymentCreateOptions {
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
