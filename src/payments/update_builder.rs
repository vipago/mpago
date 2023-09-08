use reqwest::Method;

use crate::{
    client::MercadoPagoClient,
    common::{resolve_json, MercadoPagoRequestError},
};

use super::types::{PartialPaymentResult, PaymentResponse, PaymentStatus, PaymentUpdateOptions};

pub struct PaymentUpdateBuilder(pub u64, pub PaymentUpdateOptions);

impl PaymentUpdateBuilder {
    pub async fn send(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<PaymentResponse, MercadoPagoRequestError> {
        let res = mp_client
            .start_request(Method::PUT, format!("/v1/payments/{}", self.0))
            .json(&self.1)
            .send()
            .await?;

        resolve_json::<PaymentResponse>(res).await
    }

    pub async fn cancel_payment(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<PaymentResponse, MercadoPagoRequestError> {
        let res = mp_client
            .start_request(Method::PUT, format!("/v1/payments/{}", self.0))
            .json(&PaymentUpdateOptions {
                status: Some(PaymentStatus::Cancelled),
                ..Default::default()
            })
            .send()
            .await?;

        resolve_json::<PaymentResponse>(res).await
    }
}

impl PaymentResponse {
    pub async fn cancel_payment(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<PaymentResponse, MercadoPagoRequestError> {
        PaymentUpdateBuilder(
            self.id,
            PaymentUpdateOptions {
                status: Some(PaymentStatus::Cancelled),
                ..Default::default()
            },
        )
        .send(mp_client)
        .await
    }
}

impl PartialPaymentResult {
    pub async fn cancel_payment(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<PaymentResponse, MercadoPagoRequestError> {
        PaymentUpdateBuilder(
            self.id,
            PaymentUpdateOptions {
                status: Some(PaymentStatus::Cancelled),
                ..Default::default()
            },
        )
        .send(mp_client)
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{create_test_client, get_test_payment_options},
        payments::{create_builder::PaymentCreateBuilder, types::PaymentStatus},
    };

    #[tokio::test]
    async fn cancel_payment() {
        let mp_client = create_test_client();

        let payment = PaymentCreateBuilder(get_test_payment_options())
            .send(&mp_client)
            .await
            .unwrap();

        let cancel_response = payment.cancel_payment(&mp_client).await.unwrap();

        assert_eq!(cancel_response.status, PaymentStatus::Cancelled);
    }
}
