use crate::common::CurrencyId;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SubscriptionCreateOptions {
    pub auto_recurring: Option<AutoRecurring>,
    pub back_url: String,
    pub card_token_id: Option<String>,
    pub external_reference: Option<String>,
    pub payer_email: String,
    pub preapproval_plan_id: Option<String>,
    pub reason: Option<String>,
    pub status: SubscriptionStatus,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SubscriptionSearchParams {
    q: Option<String>,
    payer_id: Option<u16>,
    payer_email: Option<String>,
    preapproval_plan_id: Option<String>,
    #[serde(with = "rust_decimal::serde::float_option")]
    transaction_amount: Option<Decimal>,
    semaphore: Option<SubscriptionSemaphore>,
    sort: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription {
    pub id: String,
    pub application_id: u128,
    pub collector_id: u32,
    pub preapproval_plan_id: Option<String>,
    pub reason: String,
    pub external_reference: Option<String>,
    pub back_url: String,
    pub init_point: String,
    pub auto_recurring: AutoRecurring,
    pub payer_id: u32,
    pub card_id: Option<u16>,
    pub payment_method_id: Option<u16>,
    pub next_payment_date: Option<String>,
    pub date_created: Option<String>,
    pub last_modified: Option<String>,
    pub status: SubscriptionStatus,
    pub summarized: Option<SubscriptionSummarized>,
    pub first_invoice_offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateSubscriptionRequest {
    pub auto_recurring: Option<AutoRecurring>,
    pub back_url: Option<String>,
    pub card_token_id: Option<u32>,
    pub external_reference: Option<String>,
    pub reason: Option<String>,
    pub status: Option<SubscriptionStatus>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UpdateAutoRecurring {
    #[serde(with = "rust_decimal::serde::float_option")]
    pub transaction_amount: Option<Decimal>,
    pub currency_id: Option<CurrencyId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionSummarized {
    pub quotas: Option<u32>,
    pub charge_quantity: Option<u32>,
    pub pending_charge_quantity: Option<u32>,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub charged_amount: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub pending_charge_amount: Option<Decimal>,
    pub semaphore: Option<SubscriptionSemaphore>,
    pub last_charged_date: Option<String>,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub last_charged_amount: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSemaphore {
    Green,
    Yellow,
    Ged,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    #[default]
    Pending,
    Authorized,
    Paused,
    Cancelled,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AutoRecurring {
    pub frequency: u8,
    pub frequency_type: FrequencyType,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub currency_id: CurrencyId,
    #[serde(with = "rust_decimal::serde::float")]
    pub transaction_amount: Decimal,
    pub free_trial: Option<FreeTrial>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreeTrial {
    pub frequency: u8,
    pub frequency_type: FrequencyType,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FrequencyType {
    #[default]
    Days,
    Months,
}
