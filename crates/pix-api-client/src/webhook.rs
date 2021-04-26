use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{ApiRequest, PixClient};

pub struct WebhookEndpoint<'a> {
    inner: &'a PixClient,
}

#[derive(Debug, Serialize, Deserialize)]
struct WebHookPayload {
    #[serde(rename = "webhookUrl")]
    webhook_url: String,
}

impl WebHookPayload {
    pub fn new(webhook_url: String) -> WebHookPayload {
        Self { webhook_url }
    }
}

impl PixClient {
    pub fn webhook(&self) -> WebhookEndpoint {
        WebhookEndpoint { inner: &self }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebHookResponse {}

impl<'a> WebhookEndpoint<'a> {
    pub fn criar_por_chave(&self, chave_pix: String, webhook_url: String) -> ApiRequest<WebHookResponse> {
        let endpoint = format!("{}/webhook/{}", &*self.inner.base_endpoint, chave_pix);
        let request = self.inner.client.request(Method::PUT, endpoint);
        let payload = WebHookPayload::new(webhook_url);

        ApiRequest::new(request.json(&payload))
    }

    pub fn consultar_por_chave(&self, chave_pix: String, webhook_url: String) -> RequestBuilder {
        let endpoint = format!("{}/webook/{}", &*self.inner.base_endpoint, chave_pix);
        self.inner.client.request(Method::GET, endpoint)
    }
    pub fn cancelar_por_chave(&self, chave_pix: String, webhook_url: String) -> RequestBuilder {
        let endpoint = format!("{}/webhook/{}", &*self.inner.base_endpoint, chave_pix);
        self.inner.client.request(Method::DELETE, &self.inner.base_endpoint)
    }

    /// Criar uma cobrança imediata.
    /// Diferente de `criar_cobranca_imediata`, o `txid` é definido pelo PSP.
    pub fn consultar_todos(&self) -> ApiRequest<CobResponse> {
        let endpoint = format!("{}/webook", &*self.inner.base_endpoint);
        let request = self.inner.client.request(Method::GET, &self.inner.base_endpoint);

        ApiRequest::new(request)
    }
}

#[derive(Debug, Deserialize)]
pub struct CobResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Executor;

    #[tokio::test]
    async fn sample() {
        let certificate = vec![12; 50];
        let client = PixClient::new("a", "a", "a", certificate);

        let teste = client.webhook().criar_por_chave("abas".to_string(), "link".to_string());
    }
}
