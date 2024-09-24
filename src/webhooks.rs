use std::num::ParseIntError;
use std::str;

use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct WebhookBody {
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
    pub id: u64,
    pub live_mode: bool,
    pub r#type: WebhookType,
    pub date_created: String,
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
    pub user_id: u64,
    pub api_version: String,
    pub action: String,
    pub data: Option<WebhookData>,
}

#[derive(Deserialize, Debug)]
pub struct WebhookData {
    #[serde(deserialize_with = "serde_aux::prelude::deserialize_option_number_from_string")]
    pub id: Option<u64>,
}

impl WebhookBody {
    pub fn valid_origin(
        &self,
        key: &[u8],
        x_signature_header: String,
        x_request_id: Option<String>,
    ) -> bool {
        if let Ok(v) = WebhookHeader::try_from(x_signature_header) {
            let mut hasher =
                HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");

            hasher.update(
                format!(
                    "id:{};{}ts:{};",
                    self.id,
                    if let Some(x_request_id) = x_request_id {
                        format!("request-id:{};", x_request_id)
                    } else {
                        String::new()
                    },
                    v.ts
                )
                .as_bytes(),
            );

            let result = hasher.finalize().into_bytes();

            let hash = result.as_slice();

            let mut hash_hex = String::new();

            for byte in hash {
                hash_hex.push_str(&format!("{:02x}", byte));
            }

            hash_hex == v.v1
        } else {
            false
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebhookType {
    Payment,
    SubscriptionPreApproval,
    SubscriptionPreapprovalPlan,
    SubscriptionAuthorizedPayment,
    PointIntegrationWh,
    TopicClaimsIntegrationWh,
}

pub struct WebhookHeader {
    pub ts: u64,
    pub v1: String,
}

impl TryFrom<String> for WebhookHeader {
    type Error = ParseIntError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut ts = 0;
        let mut v1 = String::new();

        for pair in s.split(',') {
            let mut split = pair.split('=');
            match split.next() {
                Some("ts") => ts = split.next().unwrap_or("").parse()?,
                Some("v1") => v1 = split.next().unwrap_or("").to_string(),
                _ => {}
            }
        }

        Ok(WebhookHeader { ts, v1 })
    }
}

#[cfg(test)]
mod webhook_tests {
    use crate::webhooks::{WebhookBody, WebhookHeader, WebhookType};
    use hex_literal::hex;

    const KEY: &[u8] = &hex!("b00b15");

    #[test]
    fn test_webhook_header() {
        let header =
            WebhookHeader::try_from("ts=1234567890,v1=1234567890abcdef".to_string()).unwrap();

        assert_eq!(header.ts, 1234567890);
        assert_eq!(header.v1, "1234567890abcdef");
    }

    #[test]
    fn test_webhook_valid_without_request_id() {
        let body = WebhookBody {
            id: 1234567890,
            live_mode: false,
            r#type: WebhookType::Payment,
            date_created: "2021-01-0100:00:00Z".to_string(),
            user_id: 1234567890,
            api_version: "v1".to_string(),
            action: "payment.created".to_string(),
            data: None,
        };

        assert!(body.valid_origin(
            KEY,
            "ts=1717037131000,v1=aace269406ac439a100b7a06480cf7c1d84c46fab0ce24e5acd0ca363847953b"
                .to_owned(),
            None
        ));
    }

    #[test]
    fn test_webhook_valid_with_request_id() {
        let body = WebhookBody {
            id: 1234567890,
            live_mode: false,
            r#type: WebhookType::Payment,
            date_created: "2021-01-0100:00:00Z".to_string(),
            user_id: 1234567890,
            api_version: "v1".to_string(),
            action: "payment.created".to_string(),
            data: None,
        };

        assert!(body.valid_origin(
            KEY,
            "ts=1717037131000,v1=72fc8fedd2bbe13efdfe045be61872f7ce6004ffda8d22c7440db5fc003503fb"
                .to_owned(),
            Some("69420".to_string())
        ));
    }
}
