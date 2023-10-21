use crate::client::MercadoPagoClient;
use crate::common::{resolve_json, MercadoPagoRequestError, SearchResponse};
use crate::subscription::types::{Subscription, SubscriptionSearchParams};
use async_stream::stream;
use reqwest::Method;
use std::pin::Pin;
use tokio_stream::Stream;

pub struct SubscriptionSearchBuilder(pub SubscriptionSearchParams);

impl SubscriptionSearchBuilder {
    pub async fn fetch_all_streamed<'a>(
        self,
        mp_client: &'a MercadoPagoClient,
    ) -> Pin<Box<dyn Stream<Item = Result<Subscription, MercadoPagoRequestError>> + 'a>> {
        const DEFAULT_PAGE_LIMIT: usize = 30;
        Box::pin(stream! {
            let options = self.0;
            let limit = options.limit.unwrap_or(DEFAULT_PAGE_LIMIT);
            let mut offset = options.offset.unwrap_or_default();
            loop {
                let res = match mp_client
                    .start_request(Method::GET, "/preapproval/search")
                    .query(&SubscriptionSearchParams {
                        offset: Some(offset),
                        limit: Some(limit),
                        ..options.clone()
                    })
                    .send()
                    .await {
                        Ok(page) => page,
                        // .next() return Some(Err(MercadoPagoRequestError))
                        Err(err) => {
                            yield Err(err.into());
                            continue;
                        }
                    };
                let page = match resolve_json::<SearchResponse<Subscription>>(res).await {
                    Ok(page) => page,
                    // .next() return Some(Err(MercadoPagoRequestError))
                    Err(err) => {
                        yield Err(err);
                        continue;
                    }
                };

                for payment in page.results {
                    // .next() return Some(Ok(Subscription))
                    yield Ok(payment)
                }

                offset += limit;
                if offset >= page.paging.total {
                    // .next() return None
                    return
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::create_test_client;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn search_subscription() {
        let mp_client = create_test_client();

        let mut response = SubscriptionSearchBuilder(SubscriptionSearchParams {
            limit: Some(2),
            ..Default::default()
        })
        .fetch_all_streamed(&mp_client)
        .await;

        if let Some(Err(v)) = response.next().await {
            panic!("Failed to fetch first subscription: {v:?}")
        }
    }
}
