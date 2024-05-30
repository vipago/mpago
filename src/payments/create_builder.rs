use reqwest::Method;
use rust_decimal::Decimal;

use crate::{
    client::MercadoPagoClient,
    common::{resolve_json, MercadoPagoRequestError},
    payer::Payer,
    payments::types::PaymentResponse,
};

use super::types::{AdditionalInfo, PaymentCreateOptions, PaymentMethodId, ProductItem};

/// Builder for creating a payment
///
/// # Arguments
///
/// * `options` - Options to create the payment.
/// * `idempotency_key` - Idempotency key is a unique value that is used to prevent duplicate processing of requests
///
/// # Example
/// ```
/// use mpago::{payments::PaymentCreateBuilder, Decimal};
///
/// PaymentCreateBuilder(
///     PaymentCreateOptions {
///         transaction_amount: Decimal::from(25, 0), // 25.0
///         installments: 1,
///         description: Some("Some product".to_string()),
///         payment_method_id: PaymentMethodId::Pix,
///         payer: Payer {
///             email: "test_user@testmail.com".to_string(),
///             ..Default::default()
///         },
///         ..Default::default()
///     },
///     None
/// )
/// ```
///
/// # Docs
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments/post>
pub struct PaymentCreateBuilder(pub PaymentCreateOptions, pub Option<String>);

impl PaymentCreateBuilder {
    /// Sets the items for `additional_info.items`
    ///
    /// # Arguments
    ///
    /// * `items` - An iterator of the items.
    ///
    /// # Example
    /// ```
    /// use mpago::{Decimal, payments::PaymentCreateBuilder};
    ///
    /// PaymentCreateBuilder(
    ///     PaymentCreateOptions {
    ///         transaction_amount: Decimal::new(25, 0), // 25
    ///         installments: 1,
    ///         description: Some("Some product".to_string()),
    ///         payment_method_id: PaymentMethodId::Pix,
    ///         payer: Payer {
    ///             email: "test_user@testmail.com".to_string(),
    ///             ..Default::default()
    ///         },
    ///         ..Default::default()
    ///     },
    ///    None
    /// )
    /// .set_items(
    ///     [
    ///         ProductItem {
    ///             // `quantity` need to be String due to the Mercado Pago API
    ///             quantity: Some("1".to_string()),
    ///             unit_price: Some(Decimal::new(25, 0)), // 25
    ///             title: Some("Some product".to_string()),
    ///             id: Some("1".to_string()),
    ///             ..Default::default()
    ///         }
    ///     ]
    ///    .into_iter(),
    ///);
    /// ```
    pub fn set_items(mut self, items: impl Iterator<Item = ProductItem>) -> Self {
        let builder_items = &mut self.0.additional_info.items;

        *builder_items = items.collect();

        self
    }

    /// Add items in `additional_info.items`
    ///
    /// # Arguments
    ///
    /// * `items` - An iterator of the items.
    ///
    /// # Example
    /// ```
    /// use mpago::{Decimal, payments::PaymentCreateBuilder};
    ///
    /// PaymentCreateBuilder(
    ///     PaymentCreateOptions {
    ///         transaction_amount: Decimal::new(25, 0), // 25
    ///         installments: 1,
    ///         description: Some("Some product".to_string()),
    ///         payment_method_id: PaymentMethodId::Pix,
    ///         payer: Payer {
    ///             email: "test_user@testmail.com".to_string(),
    ///             ..Default::default()
    ///         },
    ///         ..Default::default()
    ///     },
    ///   None
    /// )
    /// .add_items(
    ///     [
    ///         ProductItem {
    ///             // `quantity` need to be String due to the Mercado Pago API
    ///             quantity: Some("1".to_string()),
    ///             unit_price: Some(Decimal::new(25, 0)), // 25
    ///             title: Some("Some product".to_string()),
    ///             id: Some("1".to_string()),
    ///             ..Default::default()
    ///         }
    ///     ]
    ///    .into_iter(),
    ///);
    /// ```
    pub fn add_items(mut self, items: impl Iterator<Item = ProductItem>) -> Self {
        let builder_items = &mut self.0.additional_info.items;

        builder_items.extend(items);

        self
    }

    /// Send the request
    pub async fn send(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<PaymentResponse, MercadoPagoRequestError> {
        let mut req = mp_client
            .start_request(Method::POST, "/v1/payments")
            .json(&self.0);

        if let Some(idempotency_key) = self.1 {
            req = req.header("X-Idempotency-Key", idempotency_key);
        }

        let res = req.send().await?;

        resolve_json::<PaymentResponse>(res).await
    }

    /// Returns a [`PaymentCreateBuilder`]
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the purchased product, the payment reason.
    /// * `payer` - Payer info
    /// * `payment_method_id` - Indicates the identifier of the selected payment method for making the payment.
    /// * `transaction_amount` - Amount of the payment
    /// * `idempotency_key` - Idempotency key is a unique value that is used to prevent duplicate processing of requests
    ///
    /// # Example
    ///
    /// ```
    /// use mpago::{Decimal, payments::PaymentCreateBuilder};
    ///
    /// PaymentCreateBuilder::create(
    ///     "some product",
    ///     Payer {
    ///         email: "someemail@testmail.com".to_string(),
    ///         ..Default::default()
    ///     },
    ///     PaymentMethodId::Pix,
    ///     Decimal::new(20, 0),
    /// );
    /// ```
    ///
    /// # Docs
    /// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments/post>
    pub fn create(
        description: impl ToString,
        payer: Payer,
        payment_method_id: PaymentMethodId,
        transaction_amount: Decimal,
        idempotency_key: Option<String>,
    ) -> PaymentCreateBuilder {
        PaymentCreateBuilder(
            PaymentCreateOptions {
                description: Some(description.to_string()),
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
            },
            idempotency_key,
        )
    }
}

#[cfg(test)]
#[cfg(ignore)]
mod tests {
    use crate::{
        common::{create_test_client, get_test_payment_options},
        payments::types::{PaymentCreateOptions, ProductItem},
    };
    use rust_decimal::Decimal;

    use super::PaymentCreateBuilder;

    #[tokio::test]
    async fn payment_create() {
        let mp_client = create_test_client();

        let res = PaymentCreateBuilder(get_test_payment_options(), None)
            .send(&mp_client)
            .await
            .unwrap();

        println!("{res:?}");
    }

    #[tokio::test]
    async fn fail_payment_create() {
        let mp_client = create_test_client();

        let res = PaymentCreateBuilder(
            PaymentCreateOptions {
                ..Default::default()
            },
            None,
        )
        .send(&mp_client)
        .await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn create_with_products() {
        let mp_client = create_test_client();

        let builder = PaymentCreateBuilder(get_test_payment_options(), None).add_items(
            [ProductItem {
                quantity: Some("1".to_string()),
                unit_price: Some(Decimal::new(10, 0)),
                title: Some("Test product".to_string()),
                id: Some("1".to_string()),
                ..Default::default()
            }]
            .into_iter(),
        );

        assert_eq!(builder.0.additional_info.items[0].id, Some("1".to_string()));

        let builder = builder.set_items(
            [ProductItem {
                quantity: Some("1".to_string()),
                unit_price: Some(Decimal::new(10, 0)),
                title: Some("Test product2".to_string()),
                id: Some("2".to_string()),
                ..Default::default()
            }]
            .into_iter(),
        );

        assert_eq!(builder.0.additional_info.items[0].id, Some("2".to_string()));

        builder.send(&mp_client).await.unwrap();
    }
}
