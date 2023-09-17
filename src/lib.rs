pub use rust_decimal::Decimal;
pub mod client;
pub mod common;
pub mod oauth;
pub mod payer;
pub mod payments;

/// The base URL for Mercado Pago API
pub static API_BASE_URL: &str = "https://api.mercadopago.com";
