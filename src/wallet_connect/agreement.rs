use reqwest::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::client::MercadoPagoClient;
/// Contains information about the actions the user should take and the amount to be paid
#[derive(Deserialize, Serialize)]
pub struct Data {
    #[serde(with = "rust_decimal::serde::float")]
    validation_amount: Decimal,
    description: String,
}
/// Contains information that the seller needs to identify the user
#[derive(Deserialize, Serialize)]
pub struct ExternalUser {
    id: String,
    /// A small description about the user, e.g. this can contain the user's name.
    description: String,
}
macro_rules! setters {
    ($(
        $(#[$outer:meta])*
        $name:ident:$type:ty
    ),*) => {
        $(
            $(#[$outer])*
            pub fn $name(mut self, $name: $type) -> Self {
                self.$name = $name;
                self
            }
        )*
    };
}
#[must_use]
pub struct Builder {
    return_url: String,
    client_id: Option<String>,
    platform_id: Option<String>,
    agreement_data: Option<Data>,
    external_flow_id: Option<String>,
    external_user: Option<ExternalUser>,
}
impl Builder {
    setters! {
        // X-Platform-ID header, this is similar to User-Agent in http, you can put your project name
        // here, if you're building a solution on top of mercado pago, you might want to provide this
        // option.
        platform_id: Option<String>,
        // Set client id of the application
        client_id: Option<String>,

        /// Callback URL
        ///
        /// After the user authorizes or denies the wallet connection, the user will be redirected to
        /// this URL
        return_url: String,
        /// Set agreement data
        agreement_data: Option<Data>,
        /// An arbitrary custom ID that can be used to lookup for this agreement, **this is not used as
        /// the actual agreement id**.
        external_flow_id: Option<String>,
        /// Set external user information
        external_user: Option<ExternalUser>
    }
    /// Sends a `Create Agreement` request to the mercado pago API with the provided options
    ///
    /// # Errors
    /// Sending the request may fail due to network or serialization issues
    pub async fn send(self, client: &MercadoPagoClient) -> Result<Agreement, reqwest::Error> {
        mod create_agreement {
            use super::{Data, ExternalUser, Serialize};
            #[derive(Serialize)]
            pub struct RequestBody {
                pub return_url: String,
                pub agreement_data: Option<Data>,
                pub external_flow_id: Option<String>,
                pub external_user: Option<ExternalUser>,
            }
            #[derive(Serialize)]
            pub struct Query {
                #[serde(rename = "client.id")]
                pub client_id: Option<String>,
            }
        }
        let request_body = create_agreement::RequestBody {
            return_url: self.return_url,
            agreement_data: self.agreement_data,
            external_flow_id: self.external_flow_id,
            external_user: self.external_user,
        };
        let query = create_agreement::Query {
            client_id: self.client_id,
        };
        let mut request = client.start_request(Method::POST, "/v2/wallet_connect/agreements");
        if let Some(platform_id) = self.platform_id {
            request = request.header("X-Platform-ID", platform_id);
        }
        request
            .query(&query)
            .json(&request_body)
            .send()
            .await?
            .json()
            .await
    }
}
#[derive(Deserialize)]
pub struct Agreement {
    agreement_id: String,
    agreement_uri: String,
}
impl Agreement {
    /// Returns a reference to the agreement id in Self
    #[must_use]
    pub fn id(&self) -> &str {
        &self.agreement_id
    }
    /// Returns a reference to the agreement URI in Self
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.agreement_uri
    }
}
