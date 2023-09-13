use reqwest::Method;

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
///
/// # Example
/// ```
/// use mpago::payments::PaymentCreateBuilder;
///
/// PaymentCreateBuilder(
///     PaymentCreateOptions {
///         transaction_amount: 25.0,
///         installments: 1,
///         description: "Some product".to_string(),
///         payment_method_id: PaymentMethodId::Pix,
///         payer: Payer {
///             email: "test_user@testmail.com".to_string(),
///             ..Default::default()
///         },
///         ..Default::default()
///     }
/// )
/// ```
///
/// # Docs
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments/post>
pub struct PaymentCreateBuilder(pub PaymentCreateOptions);

impl PaymentCreateBuilder {
    /// Sets the items for `additonal_info.items`
    ///
    /// # Arguments
    ///
    /// * `items` - An iterator of the items.
    ///
    /// # Example
    /// ```
    /// use mpago::payments::PaymentCreateBuilder;
    ///
    /// PaymentCreateBuilder(
    ///     PaymentCreateOptions {
    ///         transaction_amount: 25.0,
    ///         installments: 1,
    ///         description: "Some product".to_string(),
    ///         payment_method_id: PaymentMethodId::Pix,
    ///         payer: Payer {
    ///             email: "test_user@testmail.com".to_string(),
    ///             ..Default::default()
    ///         },
    ///         ..Default::default()
    ///     }
    /// )
    /// .set_items(
    ///     [
    ///         ProductItem {
    ///             // `quantity` and `unit_price` need to be String due to the Mercado Pago API
    ///             quantity: Some("1".to_string()),
    ///             unit_price: Some("25.0".to_string()),
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

    /// Add items in `additonal_info.items`
    ///
    /// # Arguments
    ///
    /// * `items` - An iterator of the items.
    ///
    /// # Example
    /// ```
    /// use mpago::payments::PaymentCreateBuilder;
    ///
    /// PaymentCreateBuilder(
    ///     PaymentCreateOptions {
    ///         transaction_amount: 25.0,
    ///         installments: 1,
    ///         description: "Some product".to_string(),
    ///         payment_method_id: PaymentMethodId::Pix,
    ///         payer: Payer {
    ///             email: "test_user@testmail.com".to_string(),
    ///             ..Default::default()
    ///         },
    ///         ..Default::default()
    ///     }
    /// )
    /// .add_items(
    ///     [
    ///         ProductItem {
    ///             // `quantity` and `unit_price` need to be String due to the Mercado Pago API
    ///             quantity: Some("1".to_string()),
    ///             unit_price: Some("25.0".to_string()),
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
        let res = mp_client
            .start_request(Method::POST, "/v1/payments")
            .json(&self.0)
            .send()
            .await?;

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
    ///
    /// # Example
    ///
    /// ```
    /// use mpago::payments::PaymentCreateBuilder;
    ///
    /// PaymentCreateBuilder::create(
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
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{create_test_client, get_test_payment_options},
        payments::types::{PaymentCreateOptions, ProductItem},
    };

    use super::PaymentCreateBuilder;

    #[tokio::test]
    async fn payment_create() {
        let mp_client = create_test_client();

        let res = PaymentCreateBuilder(get_test_payment_options())
            .send(&mp_client)
            .await
            .unwrap();

        println!("{res:?}");
    }

    #[tokio::test]
    async fn fail_payment_create() {
        let mp_client = create_test_client();

        let res = PaymentCreateBuilder(PaymentCreateOptions {
            ..Default::default()
        })
        .send(&mp_client)
        .await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn create_with_products() {
        let mp_client = create_test_client();

        let builder = PaymentCreateBuilder(get_test_payment_options()).add_items(
            [ProductItem {
                quantity: Some("1".to_string()),
                unit_price: Some("10.0".to_string()),
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
                unit_price: Some("10.0".to_string()),
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
