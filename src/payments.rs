use crate::payer::Payer;

use self::create_builder::PaymentCreateBuilder;
use self::types::{AdditionalInfo, PaymentCreateOptions, PaymentMethodId};

pub mod create_builder;
pub mod types;

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

pub fn search() {
    todo!()
}

pub fn get() {
    todo!()
}

pub fn update() {
    todo!()
}
