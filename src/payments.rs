use crate::payer::Payer;

use self::create_builder::PaymentCreateBuilder;
use self::get_builder::PaymentGetBuilder;
use self::search_builder::PaymentSearchBuilder;
use self::types::{
    AdditionalInfo, PaymentCreateOptions, PaymentMethodId, PaymentSearchOptions,
    PaymentUpdateOptions,
};
use self::update_builder::PaymentUpdateBuilder;

pub mod create_builder;
pub mod get_builder;
pub mod search_builder;
pub mod types;
pub mod update_builder;

/// Returns a [`PaymentCreateBuilder`]
///
/// # Arguments
///
/// * `description` - Description of the purchased product, the payment reason.
/// * `payer` - Payer info
/// * `payment_method_id` - Indicates the identifier of the selected payment method for making the payment.
/// * `transaction_amount` - Amount of the payment
///
/// # Example
///
/// ```
/// payments::create(
///     "some product",
///     Payer {
///         email: "someemail@testmail.com".to_string(),
///         ..Default::default()
///     },
///     PaymentMethodId::Pix,
///     20.0,
/// );
/// ```
///
/// # Docs
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments/post>
pub fn create(
    description: impl ToString,
    payer: Payer,
    payment_method_id: PaymentMethodId,
    transaction_amount: f32,
) -> PaymentCreateBuilder {
    PaymentCreateBuilder(PaymentCreateOptions {
        description: description.to_string(),
        additional_info: AdditionalInfo {
            ip_address: None,
            items: vec![],
            payer: None,
            shipments: None,
        },
        payer,
        payment_method_id,
        transaction_amount,
        ..Default::default()
    })
}

/// Returns a [`PaymentSearchBuilder`]
///
/// # Example
///
/// ```
/// payments::search(
///        PaymentSearchOptions {
///            sort: Some(PaymentSearchSort::DateApproved),
///            limit: Some(10),
///            ..Default::default()
///        }
/// )
/// ```
///
/// # Docs
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments_search/get>
pub fn search(options: PaymentSearchOptions) -> PaymentSearchBuilder {
    PaymentSearchBuilder(options)
}

/// Returns a [`PaymentGetBuilder`]
///
/// # Arguments
///
/// * `id` - Payment ID
///
/// # Example
/// ```
/// payments::get(1737821)
/// ```
///
/// # Docs
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments_id/get>
pub fn get(id: u64) -> PaymentGetBuilder {
    PaymentGetBuilder(id)
}

/// Returns a [`PaymentUpdateBuilder`]
///
/// # Arguments
///
/// * `id` - Payment ID
/// * `options` - Fields to update
///
/// # Example
/// ```
/// payments::update(
///         1737821,
///         PaymentUpdateOptions {
///             status: Some(PaymentStatus::Cancelled),
///             ..Default::default()
///         }
/// )
/// ```
///
/// # Docs
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments_id/put>
pub fn update(id: u64, options: PaymentUpdateOptions) -> PaymentUpdateBuilder {
    PaymentUpdateBuilder(id, options)
}
