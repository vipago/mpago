use async_stream::stream;
use reqwest::Method;
use std::pin::Pin;
use tokio_stream::Stream;

use super::{
    get_builder::PaymentGetBuilder,
    types::{PartialPaymentResult, PaymentResponse, PaymentSearchOptions, PaymentSearchResponse},
};
use crate::{
    client::MercadoPagoClient,
    common::{resolve_json, MercadoPagoRequestError},
};

/// Builder for search payments
///
/// # Arguments
///
/// * `options` - Options to search for payments
///
/// # Example
/// ```
/// use mpago::payments::PaymentSearchBuilder;
///
/// PaymentSearchBuilder(
///     PaymentSearchOptions {
///         limit: Some(10),
///         sort: Some(PaymentSearchSort::DateLastUpdated)
///         ..Default::default()
///     }
/// )
/// ```
///
/// # Docs
/// <https://www.mercadopago.com.br/developers/pt/reference/payments/_payments_search/get>
pub struct PaymentSearchBuilder(pub PaymentSearchOptions);

impl PaymentSearchBuilder {
    /// This function creates a stream of payments, it goes through all the pages.
    ///
    /// When you fetch a payment, it will check if you reached the end of a page, if you have, it will fetch another page and return the first payment on that page, other wise it gives you the next payment from the current page.
    pub async fn fetch_all_streamed<'a>(
        self,
        mp_client: &'a MercadoPagoClient,
    ) -> Pin<Box<dyn Stream<Item = Result<PartialPaymentResult, MercadoPagoRequestError>> + 'a>>
    {
        const DEFAULT_PAGE_LIMIT: usize = 30;
        Box::pin(stream! {
            let options = self.0;
            let limit = options.limit.unwrap_or(DEFAULT_PAGE_LIMIT);
            let mut offset = options.offset.unwrap_or_default();
            loop {
                let res = match mp_client
                    .start_request(Method::GET, "/v1/payments/search")
                    .query(&PaymentSearchOptions {
                        offset: Some(offset),
                        limit: Some(limit),
                        ..options.clone()
                    })
                    .send()
                    .await {
                        Ok(page) => page,
                        // .next() retorna Some(Err(MercadoPagoRequestError))
                        Err(err) => {
                            yield Err(err.into());
                            continue;
                        }
                    };
                let page = match resolve_json::<PaymentSearchResponse>(res).await {
                    Ok(page) => page,
                    // .next() retorna Some(Err(MercadoPagoRequestError))
                    Err(err) => {
                        yield Err(err);
                        continue;
                    }
                };

                for payment in page.results {
                    // .next() retorna Some(Ok(PartialPaymentResult))
                    yield Ok(payment)
                }

                offset += limit;
                if offset >= page.paging.total {
                    // .next() retorna None
                    return
                }
            }
        })
    }
}

impl PartialPaymentResult {
    /// Returns the full payment information
    pub async fn fetch_full_payment(
        self,
        mp_client: &MercadoPagoClient,
    ) -> Result<PaymentResponse, MercadoPagoRequestError> {
        PaymentGetBuilder(self.id).send(mp_client).await
    }
}

#[cfg(test)]
#[cfg(ignore)]
mod tests {
    use super::{PaymentSearchBuilder, PaymentSearchOptions};
    use crate::common::create_test_client;
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn search_payments() {
        let mp_client = create_test_client();

        let mut response = PaymentSearchBuilder(PaymentSearchOptions {
            limit: Some(2),
            ..Default::default()
        })
        .fetch_all_streamed(&mp_client)
        .await;

        if let Some(Ok(v)) = response.next().await {
            assert!(v.fetch_full_payment(&mp_client).await.is_ok());
        } else {
            panic!("Failed to fetch first item");
        }
    }
}
