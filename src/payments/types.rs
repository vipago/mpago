use iso_currency::Currency;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_with::skip_serializing_none;

use crate::payer::{AdditionalInfoPayer, Payer};

/// # PaymentUpdateOptions
/// Struct to use in [`PaymentUpdateBuilder`](crate::payments::PaymentUpdateBuilder)
///
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments_id/put>
#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
pub struct PaymentUpdateOptions {
    /// It's a boolean field that exists in two-step payments (such as debit cards). In this type of payment, which is done asynchronously, first, the purchase amount is reserved (capture = false). This amount is captured and not immediately debited from the account. When the money is actually transferred to the collector (the recipient of the payment), the capture of the amount is performed (capture = true).
    pub capture: Option<bool>,
    /// Payment expiration date. The valid format for the attribute is as follows - "yyyy-MM-dd'T'HH:mm:ssz". For example - 2022-11-17T09:37:52.000-04:00.
    pub date_of_expiration: Option<String>,
    pub status: Option<PaymentStatus>,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub transaction_amount: Option<Decimal>,
}

/// # PaymentSearchOptions
/// Struct to use in [`PaymentSearchBuilder`](crate::payments::PaymentSearchBuilder)
///
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments_search/get>
#[skip_serializing_none]
#[derive(Serialize, Debug, Default, Clone)]
pub struct PaymentSearchOptions {
    /// Parameter used for sorting a list of payments.
    pub sort: Option<PaymentSearchSort>,
    /// Quantity of payments returned.
    pub limit: Option<usize>,
    /// Quantity of payments to skip.
    pub offset: Option<usize>,
    /// Sorts the payment in ascending or descending order.
    pub criteria: Option<PaymentSearchCriteria>,
    /// It is an external reference for the payment.
    ///
    /// It can be, for example, a hash code from the Central Bank, serving as an origin identifier for the transaction.
    pub external_reference: Option<String>,
    /// Defines which date field Mercado Pago should check when searching using `begin_date` and `end_date`
    ///
    /// What this means is that you can filter by update, created date or whatever date
    pub range: Option<PaymentSearchRange>,
    /// Sets the start of the search interval for payments.
    ///
    /// Its format can be a relative date - `"NOW-XDAYS"`, `"NOW-XMONTHS"` - or an absolute date in [`ISO8601`](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    ///
    /// If not specified, it defaults to `"NOW-3MONTHS"`.
    pub begin_date: Option<String>,
    /// Sets the end of the search interval for payments.
    ///
    /// Its format can be a relative date - `"NOW-XDAYS"`, `"NOW-XMONTHS"` - or an absolute date in [`ISO8601`](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    ///
    /// If not specified, it defaults to `"NOW-3MONTHS"`.
    pub end_date: Option<String>,
}

/// Parameter used to define the search interval for payments.
///
/// It is related to `begin_date` and `end_date`
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentSearchRange {
    DateApproved,
    DateCreated,
    DateLastUpdated,
    MoneyReleaseDate,
}

/// Sorts the payment in ascending or descending order.
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentSearchCriteria {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// Parameter used for sorting a list of payments.
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentSearchSort {
    DateApproved,
    DateCreated,
    DateLastUpdated,
    Id,
    MoneyReleaseDate,
}

/// # PartialPaymentResult
/// Essential information of Payment response.
///
/// Used in [`PaymentSearchResponse`] to save memory.
#[derive(Deserialize, Debug)]
pub struct PartialPaymentResult {
    pub id: u64,
    /// Payment create date. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_created: String,
    /// Date when payment was approved. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_approved: Option<String>,
    /// Date when payment was last updated. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_last_update: Option<String>,
    /// Date when payment will expire. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_of_expiration: String,
    /// Type of operation
    pub operation_type: OperationType,
    /// Payment method ID. Indicates the ID of the selected payment method for making the payment.
    pub payment_method_id: PaymentMethodId,
    /// It is the type of payment method (credit card, bank transfer, boleto, ATM, etc.).
    pub payment_type_id: PaymentTypeId,
    pub status: PaymentStatus,
    /// Detail of the outcome of the collection.
    pub status_detail: Option<PaymentStatusDetail>,
    /// Identifier of the currency used in the payment.
    pub currency_id: Option<CurrencyId>,
    /// Description sent in the payment creation request.
    pub description: Option<String>,
    /// Indicates whether the payment was made in a production environment or a test environment. If `true`, the refund will be processed in production mode. If `false`, the refund will be processed in sandbox mode.
    pub live_mode: bool,
    /// Transaction authorization code for payments with `"payment_method_type"` of the types `"credit_card"`, `"debit_card"`, and `"voucher_card"` (voucher card for benefits, like Alelo).
    ///
    /// In summary, this code is used for card transactions. The code is numeric and consists of 6 digits.
    pub authorization_code: Option<String>,
    /// Payer's information - ID (identification number), email, identification (type and document number).
    pub payer: Payer,
    // pub metadata: T,
    /// It is an external reference for the payment. It can be, for example, a hash code from the Central Bank, serving as an origin identifier for the transaction.
    pub external_reference: Option<String>,
    #[serde(with = "rust_decimal::serde::float")]
    pub transaction_amount: Decimal,
    pub installments: u32,
    pub processing_mode: PaymentProcessingMode,
}

/// # PaymentSearchResponse
/// Used in pagination of [`PaymentSearchBuilder`](crate::payments::PaymentSearchBuilder)
///
/// Response from `/v1/payments/search`
///
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments_search/get>
#[derive(Deserialize, Debug)]
pub struct PaymentSearchResponse {
    pub paging: Paging,
    pub results: Vec<PartialPaymentResult>,
}

/// Pagination information for search results.
#[derive(Deserialize, Debug)]
pub struct Paging {
    /// Total number of items in the charge.
    pub total: usize,
    /// The maximum number of entries to be returned.
    pub limit: usize,
    /// Total number of skiped items
    pub offset: usize,
}

/// # PaymentCreateOptions
/// Used as the request body for creating payments
///
///  See [`PaymentCreateBuilder`](crate::payments::PaymentCreateBuilder) if you want to create payments.
///
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments/post>
#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct PaymentCreateOptions {
    /// At the Payments level, it's primarily data, and we forward this information to other APIs, such as Risco, for scoring and fraud prevention, and to Taxes to determine them for international payments.
    pub additional_info: AdditionalInfo,
    /// Commission (fee) that third parties (integrators) charge their clients, in this case, sellers, for using the marketplace platform and other services. This is a monetary amount determined by the integrator for the seller.
    #[serde(with = "rust_decimal::serde::float_option")]
    pub application_fee: Option<Decimal>,
    /// When set to `true`, payments can only be `"approved"` or `"rejected"`. Otherwise, they can also result in being `"in_process"`.
    pub binary_mode: Option<bool>,
    /// URL to which Mercado Pago makes the final redirection (only for bank transfers).
    pub callback_url: Option<String>,
    /// It is the identifier of the entity that models the nature of discounts. All coupons come from a single campaign. The campaign configures, among other things, the available budget balance, dates during which the coupons can be used, rules for coupon application, among others. It represents the promise of a discount.
    pub campaign_id: Option<u32>,
    /// It's a boolean field present in two-step payments (like debit card payments).
    ///
    /// In this type of payment, which is done asynchronously, first, the purchase amount is reserved (`capture = false`). This amount is captured and not immediately debited from the account.
    /// When the money is actually transferred to the collector (the recipient of the payment), the capture of the amount is performed (`capture = true`).
    pub capture: Option<bool>,
    /// It is the value of the discount coupon.
    #[serde(with = "rust_decimal::serde::float_option")]
    pub coupon_amount: Option<Decimal>,
    /// Discount campaign with a specific code.
    pub coupon_code: Option<String>,
    /// Date when payment will expire. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_of_expiration: Option<String>,
    /// Description of the purchased product, the payment reason.
    pub description: Option<String>,
    /// Attribute that commonly contains an agreement on how much will be charged to the user (typically, this field is more relevant for Marketplace payments). Pricing and fees are calculated based on this identifier.
    pub differential_pricing_id: Option<u32>,
    /// It is an external reference for the payment. It can be, for example, a hash code from the Central Bank, serving as an origin identifier for the transaction.
    pub external_reference: Option<String>,
    pub installments: u32,
    /// It is the identifier of the card issuer being used in a credit or debit card payment.
    pub issuer_id: Option<String>,
    // pub metadata: T,
    /// Notification URL available to receive notifications of events related to the payment.
    pub notification_url: Option<String>,
    /// Payer info
    pub payer: Payer,
    /// Indicates the identifier of the selected payment method for making the payment.
    pub payment_method_id: PaymentMethodId,
    /// Description with which the payment will appear on the card statement (e.g., `MERCADOPAGO`).
    pub statement_descriptor: Option<String>,
    /// Card token identifier (required for credit cards). The card token is created from the card's own information, increasing security during the payment process. Additionally, once the token is used for a specific purchase, it is discarded, and a new token is required for future purchases.
    pub token: Option<String>,
    #[serde(with = "rust_decimal::serde::float")]
    pub transaction_amount: Decimal,
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
            description: Some("".to_string()),
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
            transaction_amount: Decimal::new(0, 1),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PaymentResponse {
    pub id: u64,
    /// Payment create date. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_created: String,
    /// Date when payment was approved. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_approved: Option<String>,
    /// Date when payment was last updated. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_last_update: Option<String>,
    /// Date when payment will expire. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_of_expiration: String,
    /// The date on which the payment is settled, and the money is made available in the Collector's Mercado Pago account (the recipient of the payment).
    pub money_release_date: Option<String>,
    /// Type of operation
    pub operation_type: OperationType,
    /// It is the identifier of the card issuer being used in a credit or debit card payment.
    pub issuer_id: Option<String>,
    /// Indicates the identifier of the selected payment method for making the payment.
    pub payment_method_id: PaymentMethodId,
    /// It is the type of payment method (credit card, bank transfer, boleto, ATM, etc.).
    pub payment_type_id: PaymentTypeId,
    pub status: PaymentStatus,
    /// Detail of the outcome of the collection.
    pub status_detail: Option<PaymentStatusDetail>,
    /// Identifier of the currency used in the payment.
    pub currency_id: Option<CurrencyId>,
    /// Description sent in the payment creation request.
    pub description: Option<String>,
    /// Indicates whether the payment was made in a production environment or a test environment. If `true`, the refund will be processed in production mode. If `false`, the refund will be processed in sandbox mode.
    pub live_mode: bool,
    /// Transaction authorization code for payments with `"payment_method_type"` of the types `"credit_card"`, `"debit_card"`, and `"voucher_card"` (voucher card for benefits, like Alelo).
    ///
    /// In summary, this code is used for card transactions. The code is numeric and consists of 6 digits.
    pub authorization_code: Option<String>,
    /// This field is used to identify if a payment is "PNF" (payment in the flow). Payment in the flow is a method of releasing funds where the installments received by a seller are released over the course of months (corresponding to the number of installments). The possible values for this field are `None` or `"payment_in_flow"`.
    pub money_release_schema: Option<String>,
    /// Corresponds to the values of taxes calculated for the payment.
    #[serde(with = "rust_decimal::serde::float")]
    pub taxes_amount: Decimal,
    /// Basically, it is an object that allows for converting payments of the CBT (Cross Border Trade) type, which are international payments made in foreign currency, into dollars.
    pub counter_currency: Option<String>,
    /// Shipping charge amount.
    #[serde(with = "rust_decimal::serde::float")]
    pub shipping_amount: Decimal,
    /// Digital identifier of the Point of Sale (POS). These are physical sales points that use card terminals for transactions.
    pub pos_id: Option<String>,
    /// Identifier of the store to which the cash register belongs.
    pub store_id: Option<String>,
    /// It is the user who receives the money. For example - A user (payer) purchases a cellphone through the marketplace. The identifier of the store/seller receiving the payment is the `collector_id`.
    pub collector_id: u64,
    pub payer: Payer,
    // pub metadata: T,
    /// At the Payments level, it's primarily data, and we forward this information to other APIs, such as Risco, for scoring and fraud prevention, and to Taxes to determine them for international payments.
    pub additional_info: AdditionalInfo,
    /// It is an external reference for the payment. It can be, for example, a hash code from the Central Bank, serving as an origin identifier for the transaction.
    pub external_reference: Option<String>,
    #[serde(with = "rust_decimal::serde::float")]
    pub transaction_amount: Decimal,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub transaction_amount_refunded: Option<Decimal>,
    /// It is the value of the discount coupon.
    #[serde(with = "rust_decimal::serde::float_option")]
    pub coupon_amount: Option<Decimal>,
    /// Attribute that commonly contains an agreement on how much will be charged to the user (typically, this field is more relevant for Marketplace payments). Pricing and fees are calculated based on this identifier.
    pub differencial_pricing_id: Option<String>,
    /// Pricing scheme applied by Mercado Pago. It is a field that represents information about a type of financing (installment plan).
    ///
    /// For example, `"ahora12"` is a schema that indicates that the payment is divided into 12 installments. Additionally, the financing may have an additional cost, and this cost is included in the same response, indicating who it applies to (payer/collector).
    pub deduction_schema: Option<String>,
    pub transaction_details: Option<PaymentTransactionDetails>,
    pub fee_details: Vec<FeeDetails>,
    /// Indicates whether the payment amount has been captured or is pending capture.
    pub captured: bool,
    /// When set to `true`, payments can only be `"approved"` or `"rejected"`. Otherwise, they can also result in being `"in_process"`.
    pub binary_mode: bool,
    /// Identifier provided to the issuing bank to authorize payments.
    pub call_for_authorize_id: Option<String>,
    /// Description with which the payment will appear on the card statement (e.g., `MERCADOPAGO`).
    pub statement_descriptor: Option<String>,
    pub installments: u32,
    pub card: Option<PaymentCard>,
    /// Notification URL available to receive notifications of events related to the payment.
    pub notification_url: Option<String>,
    pub processing_mode: PaymentProcessingMode,
    /// Merchant store code identifier. Applies only to the gateway model (because the delivery of money to the merchant does not go through the Mercado Pago system).
    pub merchant_account_id: Option<String>,
    pub acquirer: Option<String>,
    /// Store number (applies to the gateway model).
    pub mechant_number: Option<String>,
    /// Information about the application that processes the payment and receives regulatory data.
    pub point_of_interaction: PaymentPointOfInteraction,
}

/// Information about the application that processes the payment and receives regulatory data.
#[derive(Deserialize, Debug)]
pub struct PaymentPointOfInteraction {
    pub r#type: PaymentTypeId,
    pub sub_type: Option<String>,
    /// Information about the application that processes the payment.
    pub application_data: Option<ApplicationData>,
    /// Information about the pending payment that was generated.
    pub transaction_data: Option<TransactionData>,
}

/// Information about the pending payment that was generated.
#[derive(Deserialize, Debug)]
pub struct TransactionData {
    /// Base64 representation of the QR code image to be scanned for payment completion.
    pub qr_code_base64: Option<String>,
    /// Alphanumeric code for copying and pasting to complete the payment.
    pub qr_code: Option<String>,
    /// A link to the rendered pix, with QR codes and instructions to pay
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
    /// The merchant will use the Mercado Pago merchant codes and will take advantage of the financial advantages that Mercado Pago offers.
    Aggregator,
    /// For the merchant it is necessary to have their own merchant codes for online sales and to have an agreement with each of the desired means of payment.
    Gateway,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentCard {
    pub id: Option<String>,
    /// BIN (Bank Identification Number) of the card. It's the initial set of digits in a credit card number that identifies the issuing bank or financial institution.
    pub first_six_digits: Option<String>,
    pub last_four_digits: Option<String>,
    pub expiration_month: Option<u8>,
    pub expiration_year: Option<u16>,
    /// Card registration date. [ISO8601](https://www.ionos.com/digitalguide/websites/web-development/iso-8601/) format.
    pub date_created: Option<String>,
    /// Date when the last payment event was registered.
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
    /// The number refers to the identifier of the user in question. If it's a CPF, for example, it will have 11 digits.
    pub number: Option<String>,
    pub r#type: Option<IdentificationType>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeeDetails {
    /// Commission detail.
    pub r#type: FeeDetailsType,
    #[serde(with = "rust_decimal::serde::float")]
    pub amount: Decimal,
    /// Who absorbs the commission cost.
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
    /// Unique identifier for the payment method.
    pub payment_method_reference_id: Option<String>,
    #[serde(with = "rust_decimal::serde::float")]
    pub net_received_amount: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub total_paid_amount: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub overpaid_amount: Decimal,
    pub external_resource_url: Option<String>,
    #[serde(with = "rust_decimal::serde::float")]
    pub installment_amount: Decimal,
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

/// Detail of the outcome of the collection.
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
    /// For untracked payment status detail
    #[serde(other)]
    Unknown(String),
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]

pub enum PaymentStatus {
    /// The user has not completed the payment process (for example, for generating a payment via boleto, it will be considered completed when the user makes the corresponding payment).
    Pending,
    /// The payment was approved and credited.
    Approved,
    /// The payment was authorized but has not been captured yet.
    Authorized,
    /// The payment is under analysis.
    InProcess,
    /// The user has initiated a dispute.
    InMediation,
    /// The payment was rejected (the user can attempt the payment again).
    Rejected,
    /// Either one of the parties canceled the payment, or the payment has expired.
    Cancelled,
    /// The payment was refunded to the user.
    Refunded,
    /// A chargeback was initiated on the buyer's credit card.
    ChargedBack,
    /// For untracked payment status
    #[serde(other)]
    Unknown(String),
}

/// It is the type of payment method (credit card, bank transfer, boleto, ATM, etc.).
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentTypeId {
    /// Money in the Mercado Pago account.
    AccountMoney,
    /// Boletos, Caixa Electronica Payment, PayCash and Oxxo, etc.
    Ticket,
    /// Bank transfer (pix).
    BankTransfer,
    /// ATM payment (widely used in Mexico through BBVA Bancomer).
    Atm,
    /// Payment by credit card.
    CreditCard,
    /// Payment by debit card.
    DebitCard,
    /// Payment by prepaid card.
    PrepaidCard,
    /// Purchases with Mercado Crédito.
    DigitalCurrency,
    /// PayPal
    DigitalWallet,
    /// Alelo benefits, Sodexo.
    VoucherCard,
    /// Payment with cryptocurrencies such as Ethereum and Bitcoin.
    CryptoTranfer,
    /// For untracked payment type
    #[serde(other)]
    Unknown(String),
}

/// Is the operation type
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    /// When money is put into an investment, such as CDB, in the Mercado Pago application.
    Investment,
    ///Typification by default of a purchase being paid using Mercado Pago.
    RegularPayment,
    /// Funds transfer between two users.
    MoneyTransfer,
    /// Automatic recurring payment due to an active user subscription.
    RecurringPayment,
    /// Money income in the user's account.
    AccountFund,
    /// Addition of money to an existing payment, made through a Mercado Pago account.
    PaymentAddition,
    /// Recharge of a user's cellphone account.
    CellphoneRecharge,
    /// Payment done through a Point of Sale.
    PosPayment,
    /// Payment to exchange currency for a user.
    MoneyExchange,
    /// For untracked operation type
    #[serde(other)]
    Unknown(String),
}

/// Payment method ID. Indicates the ID of the selected payment method for making the payment.
///
/// Check <https://www.mercadopago.com.br/developers/pt/reference/payment_methods/_payment_methods/get>
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
    /// Internal protocol (IP) originating from the request (only for bank transfers).
    pub ip_address: Option<String>,
    /// List of items to be paid.
    #[serde(default)]
    pub items: Vec<ProductItem>,
    /// The payer is the one making the payment. This field is an object that contains the payer's information.
    pub payer: Option<AdditionalInfoPayer>,
    /// An object that comprises all the information for shipping the customer's purchase.
    pub shipments: Option<Shipments>,
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
    /// It is the identifier of the product listing purchased. For example `"MLB2907679857"`.
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub picture_url: Option<String>,
    /// It is the category of the item that was purchased.
    ///
    /// Two main forms of `category_id` can be mentioned: categories entered through a code, like `"MLB189908"`, or those that are a tag, like `"phone"`.
    pub category_id: Option<String>,
    pub quantity: Option<String>,
    #[serde(with = "rust_decimal::serde::str_option")]
    pub unit_price: Option<Decimal>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReceiverAddress {
    /// Payer's postal code (ZIP code).
    pub zip_code: String,
    pub state_name: String,
    pub city_name: String,
    /// Street where the payer lives.
    pub street_name: String,
    /// House or property number where the payer lives.
    pub street_number: u32,
    /// Floor of the delivery address.
    pub floor: String,
    /// Apartment number of the delivery address.
    pub apartment: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PhoneNumber {
    /// Area code where the payer resides.
    pub area_code: String,
    /// Payer's phone number.
    pub number: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Shipments {
    /// Object that comprises the shipping address of the purchase recipient.
    pub receiver_address: ReceiverAddress,
    /// Barcode width.
    pub width: u32,
    /// Barcode height.
    pub height: u32,
}
