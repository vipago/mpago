use reqwest::Method;

use crate::{
    client::MercadoPagoClient,
    common::{resolve_json, MercadoPagoRequestError},
    payments::types::PaymentResponse,
};

use super::types::{PaymentCreateOptions, ProductItem};

pub struct PaymentCreateBuilder(pub PaymentCreateOptions);

impl PaymentCreateBuilder {
    pub fn set_items(mut self, items: impl Iterator<Item = ProductItem>) -> Self {
        let builder_items = &mut self.0.additional_info.items;

        *builder_items = items.collect();

        self
    }

    pub fn add_items(mut self, items: impl Iterator<Item = ProductItem>) -> Self {
        let builder_items = &mut self.0.additional_info.items;

        builder_items.extend(items);

        self
    }

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
