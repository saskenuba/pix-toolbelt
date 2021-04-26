use std::marker::PhantomData;

use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{ApiResponse, PixClient};

pub struct CobEndpoint<'a> {
    inner: &'a PixClient,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CobPayload {
    pub calendario: Calendario,
    pub devedor: Devedor,
    /// Valor em string
    pub valor: Valor,

    /// Campo da chave PIX do recebedor desta cobrança.
    #[serde(rename = "chave")]
    pub chave_pix_recebedor: String,

    /// Campo que será para que o pagador desta cobrança insira uma informação.
    /// Sua implementação depende do PSP do pagador. Não é garantido seu preenchimento. Verifique.
    #[serde(rename = "solicitacaoPagador")]
    pub solicitacao_pagador: Option<String>,
    #[serde(rename = "infoAdicionais")]
    pub info_adicionais: Option<Vec<InfoAdicionais>>,
}

impl CobPayload {
    /// Creates a new
    fn new(valor: f64, chave_pix_recebedor: String, devedor: Devedor) -> CobPayload {
        Self {
            calendario: Default::default(),
            devedor,
            valor: Valor::new(valor),
            chave_pix_recebedor,
            solicitacao_pagador: None,
            info_adicionais: None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Calendario {
    pub criacao: Option<String>,
    pub apresentacao: Option<String>,
    /// Segundos para a expiração da cobrança, a partir do campo `Calendario.criacao`.
    pub expiracao: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Devedor {
    cnpj: Option<String>,
    cpf: Option<String>,
    nome: String,
}

impl Devedor {
    pub fn new_pessoa_juridica(cnpj: String, nome: String) -> Self {
        Self {
            cnpj: Some(cnpj),
            cpf: None,
            nome,
        }
    }

    pub fn new_pessoa_fisica(cpf: String, nome: String) -> Self {
        Self {
            cnpj: None,
            cpf: Some(cpf),
            nome,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Valor {
    pub original: String,
    // #[serde(rename = "modalidadeAlteracao")]
    // pub modalidade_alteracao: i64,
}

impl Valor {
    // Valor com no máximo duas casas decimais. Caso houver mais que 2 casas, o valor será truncado.
    pub fn new(valor_original: f64) -> Valor {
        Self {
            original: format!("{:.2}", valor_original),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoAdicionais {
    pub nome: String,
    pub valor: String,
}

impl PixClient {
    pub fn cob(&self) -> CobEndpoint {
        CobEndpoint { inner: &self }
    }
}

impl<'a> CobEndpoint<'a> {
    pub fn criar_cobranca_txid(&self, txid: String, payload: CobPayload) -> RequestBuilder {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);

        self.inner.client.request(Method::PUT, endpoint)
    }

    pub fn consultar_cobranca_txid(&self, txid: String, payload: CobPayload) -> RequestBuilder {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);
        self.inner.client.request(Method::GET, endpoint)
    }
    pub fn revisar_cobranca_txid(&self, txid: String, payload: CobPayload) -> RequestBuilder {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);
        self.inner.client.request(Method::PATCH, &self.inner.base_endpoint)
    }

    /// Criar uma cobrança imediata.
    /// Diferente de `criar_cobranca_imediata`, o `txid` é definido pelo PSP.
    pub fn criar_cobranca_imediata(&self, payload: CobPayload) -> ApiResponse<CobResponse> {
        let endpoint = format!("{}/cob", &*self.inner.base_endpoint);
        let request = self.inner.client.request(Method::POST, &self.inner.base_endpoint);

        ApiResponse {
            request,
            response_type: PhantomData::default(),
        }
    }

    pub fn consultar_cobrancas(&self, payload: CobPayload) -> RequestBuilder {
        let endpoint = format!("{}/cob", &*self.inner.base_endpoint);

        self.inner.client.request(Method::GET, &self.inner.base_endpoint)
    }
}

#[derive(Debug, Deserialize)]
pub struct CobResponse {}

#[cfg(test)]
mod tests {
    use crate::Executor;

    use super::*;

    #[tokio::test]
    async fn sample() {
        let certificate = vec![12; 50];
        let client = PixClient::new("a", "a", "a", certificate);

        let payload = CobPayload {
            calendario: Default::default(),
            devedor: Default::default(),
            valor: Default::default(),
            chave_pix_recebedor: "".to_string(),
            solicitacao_pagador: None,
            info_adicionais: None,
        };

        let teste = client.cob().criar_cobranca_imediata(payload).execute().await;
    }

    #[test]
    fn t_value() {
        let new_value = Valor::new(512.614);
        println!(": {:?}", new_value);

        let new_integer_value = Valor::new(400f64);
        println!(": {:?}", new_integer_value);
    }
}
