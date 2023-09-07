use crate::{
    common::{resolve_json, MercadoPagoRequestError},
    API_BASE_URL,
};

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "grant_type")]
pub enum OAuthRequestBody {
    AuthorizationCode {
        client_secret: String,
        client_id: String,
        code: String,
        redirect_uri: String,
    },

    RefreshToken {
        client_secret: String,
        client_id: String,
        refresh_token: String,
    },
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OAuthResponseBody {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
    user_id: u64,
    refresh_token: String,
    public_key: String,
    live_mode: bool,
}

pub struct OAuth {}

impl OAuth {
    pub async fn create_access(
        client_id: impl ToString,
        client_secret: impl ToString,
        code: impl ToString,
        redirect_uri: impl ToString,
    ) -> Result<OAuthResponseBody, MercadoPagoRequestError> {
        let client_http = reqwest::Client::new();

        let authorization_response = client_http
            .post(format!("{API_BASE_URL}/oauth/token"))
            .json(&OAuthRequestBody::AuthorizationCode {
                client_secret: client_secret.to_string(),
                client_id: client_id.to_string(),
                code: code.to_string(),
                redirect_uri: redirect_uri.to_string(),
            })
            .send()
            .await?;

        resolve_json::<OAuthResponseBody>(authorization_response).await
    }

    pub async fn refresh_access(
        client_id: impl ToString,
        client_secret: impl ToString,
        refresh_token: impl ToString,
    ) -> Result<OAuthResponseBody, MercadoPagoRequestError> {
        let client_http = reqwest::Client::new();

        let authorization_response = client_http
            .post(format!("{API_BASE_URL}/oauth/token"))
            .json(&OAuthRequestBody::RefreshToken {
                client_secret: client_secret.to_string(),
                client_id: client_id.to_string(),
                refresh_token: refresh_token.to_string(),
            })
            .send()
            .await?;

        resolve_json::<OAuthResponseBody>(authorization_response).await
    }
}

#[cfg(test)]
mod tests {
    use crate::oauth::OAuth;
    #[tokio::test]
    #[ignore = "This test can't be automated. It needs a code provided by login. Check https://www.mercadopago.com.br/developers/pt/docs/checkout-pro/additional-content/security/oauth/creation"]
    async fn test_create_and_refresh_access() {
        let create_res = OAuth::create_access("", "", "", "").await.unwrap();

        println!("{create_res:?}");

        let refresh_res = OAuth::refresh_access("", "", "").await.unwrap();

        println!("{refresh_res:?}");
    }
}
