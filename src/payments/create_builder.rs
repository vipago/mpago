use reqwest::Method;

use crate::{client::MercadoPagoClient, payments::types::PaymentCreateResponse};

use super::types::PaymentCreateOptions;

pub struct PaymentCreateBuilder(pub PaymentCreateOptions);

impl PaymentCreateBuilder {
    pub fn add_payer(&mut self) {}

    pub async fn send(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<PaymentCreateResponse, reqwest::Error> {
        let res = mp_client
            .start_request(Method::POST, "/v1/payments")
            .json(&self.0)
            .send()
            .await?;

        let body: PaymentCreateResponse = res.error_for_status()?.json().await?;

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::MercadoPagoClientBuilder,
        payer::Payer,
        payments::types::{PaymentCreateOptions, PaymentMethodId},
    };

    use super::PaymentCreateBuilder;

    #[tokio::test]
    async fn payment_create() {
        dotenvy::dotenv().ok();

        let client = MercadoPagoClientBuilder::create_with_access_token(
            std::env::var("MERCADO_PAGO_ACCESS").unwrap(),
        );

        let res = PaymentCreateBuilder(PaymentCreateOptions {
            description: "Test".to_string(),
            payer: Payer {
                email: "test@testmail.uk".to_string(),
                first_name: None,
                last_name: None,
                entity_type: None,
                id: None,
                identification: None,
                r#type: None,
            },
            transaction_amount: 10.0,
            payment_method_id: PaymentMethodId::Pix,
            ..Default::default()
        })
        .send(&client)
        .await
        .unwrap();

        println!("{res:?}");
    }
}
