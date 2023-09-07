use reqwest::Method;

use crate::{
    client::MercadoPagoClient,
    common::{resolve_json, MercadoPagoRequestError},
};

use super::types::PaymentResponse;

pub struct PaymentGetBuilder(pub u64);

impl PaymentGetBuilder {
    pub async fn send(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<PaymentResponse, MercadoPagoRequestError> {
        let res = mp_client
            .start_request(Method::GET, format!("/v1/payments/{}", self.0))
            .send()
            .await?;

        resolve_json::<PaymentResponse>(res).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{create_test_client, get_test_payment_options},
        payments::create_builder::PaymentCreateBuilder,
    };

    use super::PaymentGetBuilder;
    #[tokio::test]
    async fn get_payment() {
        let mp_client = create_test_client();

        let create_payment = PaymentCreateBuilder(get_test_payment_options())
            .send(&mp_client)
            .await
            .unwrap();

        println!("ID: {}", create_payment.id);

        let get_payment = PaymentGetBuilder(create_payment.id)
            .send(&mp_client)
            .await
            .unwrap();

        println!("{get_payment:?}")
    }

    #[tokio::test]
    async fn fail_get_payment() {
        let mp_client = create_test_client();

        let get_payment = PaymentGetBuilder(1234567890).send(&mp_client).await;

        assert!(get_payment.is_err());
    }
}
