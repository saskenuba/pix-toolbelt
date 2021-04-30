use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{ApiRequest, PixClient};

pub struct OauthTokenEndpoint<'a> {
    inner: &'a PixClient,
}

#[derive(Debug, Serialize, Deserialize)]
struct OauthTokenPayload {
    grant_type: String,
}

impl Default for OauthTokenPayload {
    fn default() -> Self {
        Self {
            grant_type: "client_credentials".to_string(),
        }
    }
}

impl PixClient {
    pub fn oauth(&self) -> OauthTokenEndpoint {
        OauthTokenEndpoint { inner: &self }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OauthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
}

impl OauthTokenEndpoint<'_> {
    pub fn autenticar(&self, full_custom_endpoint: Option<String>) -> ApiRequest<OauthTokenResponse> {
        let endpoint = full_custom_endpoint.unwrap_or_else(|| format!("{}/oauth/token", self.inner.base_endpoint));

        self.inner
            .request_with_headers(Method::POST, &*endpoint, OauthTokenPayload::default())
    }
}
