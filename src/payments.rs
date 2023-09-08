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

pub fn search(options: PaymentSearchOptions) -> PaymentSearchBuilder {
    PaymentSearchBuilder(options)
}

pub fn get(id: u64) -> PaymentGetBuilder {
    PaymentGetBuilder(id)
}

pub fn update(id: u64, options: PaymentUpdateOptions) -> PaymentUpdateBuilder {
    PaymentUpdateBuilder(id, options)
}
