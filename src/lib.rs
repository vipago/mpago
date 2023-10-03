#![warn(clippy::pedantic, clippy::perf)]
#![allow(clippy::module_name_repetitions)]
pub use rust_decimal::Decimal;
pub mod client;
pub mod common;
pub mod oauth;
pub mod payer;
pub mod payments;
pub mod wallet_connect;
/// The base URL for Mercado Pago API
pub static API_BASE_URL: &str = "https://api.mercadopago.com";
