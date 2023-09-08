use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_with::skip_serializing_none;

use crate::payer::{AdditionalInfoPayer, Payer};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PaymentUpdateOptions {
    pub capture: Option<bool>,
    pub date_of_expiration: Option<String>,
    pub status: Option<PaymentStatus>,
    pub transaction_amount: Option<f32>,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct PaymentSearchOptions {
    pub sort: Option<PaymentSearchSort>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub criteria: Option<PaymentSearchCriteria>,
    pub external_reference: Option<String>,
    pub range: Option<String>,
    pub begin_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentSearchCriteria {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentSearchSort {
    DateApproved,
    DateCreated,
    DateLastUpdated,
    Id,
    MoneyReleaseDate,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PartialPaymentResult {
    pub id: u64,
    pub date_created: String,
    pub date_approved: Option<String>,
    pub date_last_update: Option<String>,
    pub date_of_expiration: String,
    pub operation_type: OperationType,
    pub payment_method_id: PaymentMethodId,
    pub payment_type_id: PaymentTypeId,
    pub status: PaymentStatus,
    pub status_detail: Option<PaymentStatusDetail>,
    pub currency_id: Option<CurrencyId>,
    pub description: Option<String>,
    pub live_mode: bool,
    pub authorization_code: Option<String>,
    pub payer: Payer,
    // pub metadata: T,
    pub external_reference: Option<String>,
    pub transaction_amount: f32,
    pub installments: u32,
    pub processing_mode: PaymentProcessingMode,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentSearchResponse {
    pub paging: Paging,
    pub results: Vec<PartialPaymentResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentSearchPaginator {
    pub payment_response: PaymentSearchResponse,
    pub query: PaymentSearchOptions,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Paging {
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentCreateOptions {
    pub additional_info: AdditionalInfo,
    pub application_fee: Option<f32>,
    pub binary_mode: Option<bool>,
    pub callback_url: Option<String>,
    pub campaign_id: Option<u32>,
    pub capture: Option<bool>,
    pub coupon_amount: Option<f32>,
    pub coupon_code: Option<String>,
    pub date_of_expiration: Option<String>,
    pub description: String,
    pub differential_pricing_id: Option<u32>,
    pub external_reference: Option<String>,
    pub installments: u32,
    pub issuer_id: Option<String>,
    // pub metadata: T,
    pub notification_url: Option<String>,
    pub payer: Payer,
    pub payment_method_id: PaymentMethodId,
    pub statement_descriptor: Option<String>,
    pub token: Option<String>,
    pub transaction_amount: f32,
}

impl Default for PaymentCreateOptions {
    fn default() -> Self {
        Self {
            additional_info: AdditionalInfo {
                ip_address: None,
                items: vec![],
                payer: None,
                shipments: None,
            },
            application_fee: None,
            binary_mode: None,
            callback_url: None,
            campaign_id: None,
            capture: None,
            coupon_amount: None,
            coupon_code: None,
            date_of_expiration: None,
            description: "".to_string(),
            differential_pricing_id: None,
            external_reference: None,
            installments: 1,
            issuer_id: None,
            notification_url: None,
            payer: Payer {
                email: "test@testmail.com".to_string(),
                ..Default::default()
            },
            payment_method_id: PaymentMethodId::Pix,
            statement_descriptor: None,
            token: None,
            transaction_amount: 0.0,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentResponse {
    pub id: u64,
    pub date_created: String,
    pub date_approved: Option<String>,
    pub date_last_update: Option<String>,
    pub date_of_expiration: String,
    pub money_release_date: Option<String>,
    pub operation_type: OperationType,
    pub issuer_id: Option<String>,
    pub payment_method_id: PaymentMethodId,
    pub payment_type_id: PaymentTypeId,
    pub status: PaymentStatus,
    pub status_detail: Option<PaymentStatusDetail>,
    pub currency_id: Option<CurrencyId>,
    pub description: Option<String>,
    pub live_mode: bool,
    pub authorization_code: Option<String>,
    pub money_release_schema: Option<String>,
    pub taxes_amount: f32,
    pub counter_currency: Option<String>,
    pub shipping_amount: f32,
    pub pos_id: Option<String>,
    pub store_id: Option<String>,
    pub collector_id: u64,
    pub payer: Payer,
    // pub metadata: T,
    pub additional_info: AdditionalInfo,
    pub external_reference: Option<String>,
    pub transaction_amount: f32,
    pub transaction_amount_refunded: Option<f32>,
    pub coupon_amount: Option<f32>,
    pub differencial_pricing_id: Option<String>,
    pub deduction_schema: Option<String>,
    pub transaction_details: Option<PaymentTransactionDetails>,
    pub fee_details: Vec<FeeDetails>,
    pub captured: bool,
    pub binary_mode: bool,
    pub call_for_authorize_id: Option<String>,
    pub statement_descriptor: Option<String>,
    pub installments: u32,
    pub card: Option<PaymentCard>,
    pub notification_url: Option<String>,
    pub processing_mode: PaymentProcessingMode,
    pub merchant_account_id: Option<String>,
    pub acquirer: Option<String>,
    pub mechant_number: Option<String>,
    pub point_of_interaction: PaymentPointOfInteraction,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentPointOfInteraction {
    pub r#type: PaymentTypeId,
    pub sub_type: Option<String>,
    pub application_data: Option<ApplicationData>,
    pub transaction_data: Option<TransactionData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionData {
    pub qr_code_base64: Option<String>,
    pub qr_code: Option<String>,
    pub ticket_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationData {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentProcessingMode {
    Aggregator,
    Gateway,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentCard {
    pub id: Option<String>,
    pub first_six_digits: Option<String>,
    pub last_four_digits: Option<String>,
    pub expiration_month: Option<u8>,
    pub expiration_year: Option<u16>,
    pub date_created: Option<String>,
    pub date_last_update: Option<String>,
    pub cardholder: Option<Cardholder>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Cardholder {
    pub name: Option<String>,
    pub identification: Option<IdentificationType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CardHolderIdentification {
    pub number: Option<String>,
    pub r#type: Option<IdentificationType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeeDetails {
    pub r#type: FeeDetailsType,
    pub amount: f32,
    pub fee_payer: FeePayer,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeePayer {
    Collector,
    Payer,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FeeDetailsType {
    MercadopagoFee,
    CouponFee,
    FinancingFee,
    ShippingFee,
    ApplicationFee,
    DiscountFee,
    #[serde(other)]
    Unknown(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentTransactionDetails {
    pub payment_method_reference_id: Option<String>,
    pub net_received_amount: f32,
    pub total_paid_amount: f32,
    pub overpaid_amount: f32,
    pub external_resource_url: Option<String>,
    pub installment_amount: f32,
    pub financial_institution: Option<String>,
    pub payable_deferral_period: Option<String>,
    pub acquirer_reference: Option<String>,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
pub enum CurrencyId {
    ARS,
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

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatusDetail {
    Accredited,
    PendingContingency,
    PendingWaitingTransfer,
    PendingReviewManual,
    CcRejectedBadFilledDate,
    CcRejectedBadFilledOther,
    CcRejectedBadFilledSecurityCode,
    CcRejectedBlacklist,
    CcRejectedCallForAuthorize,
    CcRejectedCardDisabled,
    CcRejectedDuplicatedPayment,
    CcRejectedHighRisk,
    CcRejectedInsufficientAmount,
    CcRejectedInvalidInstallments,
    CcRejectedMaxAttempts,
    CcRejectedOtherReason,
    #[serde(other)]
    Unknown(String),
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Approved,
    Authorized,
    InProcess,
    InMediation,
    Rejected,
    Cancelled,
    Refunded,
    ChargedBack,
    #[serde(other)]
    Unknown(String),
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentTypeId {
    AccountMoney,
    Ticket,
    BankTransfer,
    Atm,
    CreditCard,
    DebitCard,
    PrepaidCard,
    DigitalCurrency,
    DigitalWallet,
    VoucherCard,
    CryptoTranfer,
    #[serde(other)]
    Unknown(String),
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Investment,
    RegularPayment,
    MoneyTransfer,
    RecurringPayment,
    AccountFund,
    PaymentAddition,
    CellphoneRecharge,
    PosPayment,
    MoneyExchange,
    #[serde(other)]
    Unknown(String),
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethodId {
    #[default]
    Pix,
    Elo,
    Visa,
    #[serde(rename = "master")]
    MasterCard,
    Hipercard,
    #[serde(rename = "amex")]
    AmericanExpress,
    Cabal,
    Meliplaces,
    #[serde(rename = "bolbradesco")]
    Boleto,
    DebVisa,
    DebElo,
    DebMaster,
    DebCabal,
    Maestro,
    #[serde(rename = "account_money")]
    AccountMoney,
    #[serde(rename = "pec")]
    Loterica,
    #[serde(other)]
    Unknown(String),
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AdditionalInfo {
    pub ip_address: Option<String>,
    #[serde(default)]
    pub items: Vec<ProductItem>,
    pub payer: Option<AdditionalInfoPayer>,
    pub shipments: Option<Shipments>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Individual,
    Association,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
pub enum IdentificationType {
    CPF,
    CNPJ,
    CUIT,
    CUIL,
    DNI,
    CURP,
    RFC,
    CC,
    RUT,
    CI,
    #[serde(other)]
    Unknown(String),
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[skip_serializing_none]
pub struct ProductItem {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub picture_url: Option<String>,
    pub category_id: Option<String>,
    pub quantity: Option<String>,
    pub unit_price: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReceiverAddress {
    pub zip_code: String,
    pub state_name: String,
    pub city_name: String,
    pub street_name: String,
    pub street_number: u32,
    pub floor: String,
    pub apartment: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PhoneNumber {
    pub area_code: String,
    pub number: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shipments {
    pub receiver_address: ReceiverAddress,
    pub width: u32,
    pub height: u32,
}
