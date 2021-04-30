use reqwest::Method;
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

/// Base response object used by the `WebHook` for any transaction.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebHookCallbackResponse {
    pub pix: Vec<PixInput>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PixInput {
    #[serde(rename = "endToEndId")]
    pub end_to_end_id: String,
    /// Transaction id
    pub txid: Option<String>,
    /// Beneficiary's Pix Key
    pub chave: String,
    pub valor: String,
    pub horario: String,

    #[serde(rename = "infoPagador")]
    pub info_pagador: Option<String>,
    pub devolucoes: Option<Vec<Devolucoes>>,
    pub tipo: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Devolucoes {
    /// Id gerado pelo cliente para representar unicamente uma devolução.
    pub id: String,
    /// ReturnIdentification que transita na PACS004.
    #[serde(rename = "rtrId")]
    pub rtr_id: String,
    pub valor: String,
    pub horario: Horario,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Horario {
    pub solicitacao: String,
}

impl<'a> WebhookEndpoint<'a> {
    pub fn criar_por_chave(&self, chave_pix: String, webhook_url: String) -> ApiRequest<WebHookResponse> {
        let endpoint = format!("{}/webhook/{}", &*self.inner.base_endpoint, chave_pix);
        let payload = WebHookPayload::new(webhook_url);
        self.inner.request_with_headers(Method::PUT, &endpoint, payload)
    }

    pub fn consultar_por_chave(&self, chave_pix: String, webhook_url: String) -> ApiRequest<WebHookResponse> {
        let endpoint = format!("{}/webhook/{}", &*self.inner.base_endpoint, chave_pix);
        let payload = WebHookPayload::new(webhook_url);
        self.inner.request_with_headers(Method::GET, &endpoint, payload)
    }
    pub fn cancelar_por_chave(&self, chave_pix: String, webhook_url: String) -> ApiRequest<WebHookResponse> {
        let endpoint = format!("{}/webhook/{}", &*self.inner.base_endpoint, chave_pix);
        let payload = WebHookPayload::new(webhook_url);
        self.inner.request_with_headers(Method::DELETE, &endpoint, payload)
    }

    /// Criar uma cobrança imediata.
    /// Diferente de `criar_cobranca_imediata`, o `txid` é definido pelo PSP.
    pub fn consultar_todos(&self) -> ApiRequest<WebHookResponse> {
        let endpoint = format!("{}/webhook", &*self.inner.base_endpoint);
        self.inner.request_with_headers(Method::GET, &endpoint, None::<&str>)
    }
}
