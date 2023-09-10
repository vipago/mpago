use crate::{
    common::{resolve_json, MercadoPagoRequestError},
    API_BASE_URL,
};

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "grant_type")]
pub enum OAuthRequestBody {
    /// Request body for creating an authorization code for OAuth
    AuthorizationCode {
        /// Private key to be used in some plugins for generating payments.
        client_secret: String,
        /// Unique ID that identifies your application/integration.
        client_id: String,
        /// Code granted by the authentication server so that the application can obtain an access token and an associated refresh token.
        code: String,
        /// URL provided in the Redirect URL field of your application.
        redirect_uri: String,
    },

    /// Request body for refreshing an access code for OAuth
    RefreshToken {
        /// Private key to be used in some plugins for generating payments.
        client_secret: String,
        /// Unique ID that identifies your application/integration.
        client_id: String,
        /// Value received when the access token is created.
        refresh_token: String,
    },
}

/// Response body from OAuth routes
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OAuthResponseBody {
    /// Security code that identifies the user, their privileges, and an application used in various public origin requests to access protected resources.
    access_token: String,
    /// Information required for the token to be used correctly to access protected resources.
    ///
    /// The `"bearer"` type token is the only one supported by the authorization server.
    token_type: String,
    /// Fixed expiration time of the access_token expressed in seconds.
    ///
    /// By default, the expiration time is 180 days (15,552,000 seconds).
    expires_in: u64,
    /// Scopes are used in the process of authorization and consent for APIs, allowing the application to specify what access it is requesting and for the user to grant access to specific resources.
    scope: String,
    /// Identification number (Mercado Pago ID) automatically generated when an account is created on Mercado Pago. It is a unique number that identifies the Mercado Pago seller and is the owner of the application.
    user_id: u64,
    /// Code for temporal grants used to obtain access tokens so that authorization and access to resources remain valid before the expiration of the access token's validity period.
    ///
    /// Only usable once.
    refresh_token: String,
    /// Public key of the application, which is typically used in the frontend and allows, for example, knowledge of payment methods and data encryption of the card.
    public_key: String,
    /// Indicates whether the application is in production mode or test mode.
    live_mode: bool,
}

/// Struct representing OAuth
///
/// # Docs
/// <https://www.mercadopago.com.br/developers/pt/reference/oauth/_oauth_token/post>
pub struct OAuth {}

impl OAuth {
    /// Create an access token integrating an account to an application
    ///
    /// # Arguments
    ///
    /// * `client_id` - Unique ID that identifies your application/integration.
    /// * `client_secret` - Private key to be used in some plugins for generating payments.
    /// * `code` - Code granted by the authentication server so that the application can obtain an access token and an associated refresh token.
    /// * `redirect_uri` - URL provided in the Redirect URL field of your application.
    ///
    /// # How to get the code
    /// <https://www.mercadopago.com.br/developers/pt/docs/checkout-pro/additional-content/security/oauth/creation>
    ///
    /// # Example
    /// ```
    /// OAuth::create_access(
    ///     "8971239781",
    ///     "RcHGkCg2VTL6cxrxzBSDQydT",
    ///     "TG-817289123-241983636",
    ///     "https://someniceurl.com/mercadopago/"
    /// )
    /// ```
    ///
    /// # Docs
    /// <https://www.mercadopago.com.br/developers/pt/reference/oauth/_oauth_token/post>
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

    /// Refresh an access token made by an integration
    ///
    /// # Arguments
    ///
    /// * `client_id` - Unique ID that identifies your application/integration.
    /// * `client_secret` - Private key to be used in some plugins for generating payments.
    /// * `refresh_token` - Value received when the access token is created.
    ///
    /// # Example
    /// ```
    /// OAuth::refresh_access(
    ///     "8971239781",
    ///     "RcHGkCg2VTL6cxrxzBSDQydT",
    ///     "TG-78293722-241983636",
    /// ).await?;
    /// ```
    ///
    /// # Docs
    /// <https://www.mercadopago.com.br/developers/pt/reference/oauth/_oauth_token/post>
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
