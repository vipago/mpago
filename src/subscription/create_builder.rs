use crate::client::MercadoPagoClient;
use crate::common::{resolve_json, MercadoPagoRequestError};
use crate::subscription::types::{
    AutoRecurring, Subscription, SubscriptionCreateOptions, SubscriptionStatus,
};
use reqwest::Method;

pub struct SubscriptionCreateBuilder(pub SubscriptionCreateOptions);

impl SubscriptionCreateBuilder {
    pub async fn send(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<Subscription, MercadoPagoRequestError> {
        let res = mp_client
            .start_request(Method::POST, "/preapproval")
            .json(&self.0)
            .send()
            .await?;

        dbg!("{}", res.status());

        resolve_json::<Subscription>(res).await
    }

    pub fn create_without_plan(
        recurring_info: AutoRecurring,
        payer_email: String,
        reason: String,
        back_url: String,
    ) -> SubscriptionCreateBuilder {
        SubscriptionCreateBuilder(SubscriptionCreateOptions {
            auto_recurring: Some(recurring_info),
            back_url,
            payer_email,
            reason: Some(reason),
            status: SubscriptionStatus::Pending,
            ..Default::default()
        })
    }

    pub fn create_with_plan(
        preapproval_plan_id: String,
        payer_email: String,
        card_token_id: String,
        back_url: String,
    ) -> SubscriptionCreateBuilder {
        SubscriptionCreateBuilder(SubscriptionCreateOptions {
            back_url,
            payer_email,
            card_token_id: Some(card_token_id),
            preapproval_plan_id: Some(preapproval_plan_id),
            status: SubscriptionStatus::Authorized,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{create_test_client, CurrencyId};
    use crate::subscription::types::*;
    use rust_decimal::Decimal;

    #[tokio::test]
    #[ignore]
    async fn create_without_plan() {
        let mp_client = create_test_client();

        let subscription = SubscriptionCreateBuilder::create_without_plan(
            AutoRecurring {
                frequency: 1,
                frequency_type: FrequencyType::Months,
                transaction_amount: Decimal::new(10, 0),
                currency_id: CurrencyId::BRL,
                ..Default::default()
            },
            std::env::var("MP_EMAIL").unwrap(),
            String::from("Testing"),
            String::from("https://google.com"),
        )
        .send(&mp_client)
        .await
        .unwrap();

        println!("{subscription:?}");
    }
}
