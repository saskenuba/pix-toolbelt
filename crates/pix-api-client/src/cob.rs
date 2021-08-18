use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{ApiRequest, PixClient};

pub struct CobEndpoint<'a> {
    inner: &'a PixClient,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CobrancaStatus {
    ATIVA,
    CONCLUIDA,
    REMOVIDA_PELO_USUARIO_RECEBEDOR,
    REMOVIDA_PELO_PSP,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CobrancaImediata {
    /// Por default, expira em 3600 segundos, i.e 1h
    pub calendario: Calendario,
    pub devedor: Devedor,
    /// Valor em string
    pub valor: Valor,

    /// Campo da chave PIX do recebedor desta cobrança.
    #[serde(rename = "chave")]
    pub chave_pix_recebedor: String,

    /// Id da Transação. Exclusivo como resposta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,

    #[serde(rename = "loc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Campo que será para que o pagador desta cobrança insira uma informação.
    /// Sua implementação depende do PSP do pagador. Não é garantido seu preenchimento. Verifique.
    #[serde(rename = "solicitacaoPagador")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solicitacao_pagador: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "infoAdicionais")]
    pub info_adicionais: Option<Vec<InfoAdicionais>>,
}

impl CobrancaImediata {
    /// Creates a new
    pub fn new(valor: f64, chave_pix_recebedor: String, devedor: Devedor) -> CobrancaImediata {
        let valor = Valor::new(valor, false);

        Self {
            calendario: Default::default(),
            devedor,
            valor,
            chave_pix_recebedor,
            txid: None,
            location: None,
            status: None,
            solicitacao_pagador: None,
            info_adicionais: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Calendario {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criacao: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apresentacao: Option<String>,
    /// Segundos para a expiração da cobrança, a partir do campo `Calendario.criacao`.
    pub expiracao: i64,
}

impl Default for Calendario {
    fn default() -> Self {
        Self {
            criacao: None,
            apresentacao: None,
            expiracao: 3600,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Devedor {
    #[serde(skip_serializing_if = "Option::is_none")]
    cnpj: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// A ausencia deste campo, que é o mesmo que 0, significa que a cobrança não poderá ter seu valor alterado.
    /// No caso do valor de 1, significa que o valor final poderá ser alterado pelo pagador.
    #[serde(rename = "modalidadeAlteracao")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permite_alteracao: Option<i32>,
}

impl Valor {
    // Valor com no máximo duas casas decimais. Caso houver mais que 2 casas, o valor será truncado.
    pub fn new(valor_original: f64, permite_alteracao: bool) -> Valor {
        let permite_alteracao = if permite_alteracao { Some(1) } else { None };
        Self {
            original: format!("{:.2}", valor_original),
            permite_alteracao,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    id: i64,
    #[serde(rename = "location")]
    pub url: String,
    #[serde(rename = "tipoCob")]
    tipo_cob: String,
    criacao: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfoAdicionais {
    pub nome: String,
    pub valor: String,
}

impl PixClient {
    pub fn cob(&self) -> CobEndpoint {
        CobEndpoint { inner: self }
    }
}

impl<'a> CobEndpoint<'a> {
    pub fn criar_cobranca_txid(&self, txid: String, payload: CobrancaImediata) -> ApiRequest<CobrancaImediata> {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);
        self.inner.request_with_headers(Method::PUT, &*endpoint, payload)
    }

    pub fn consultar_cobranca_txid(&self, txid: String, payload: CobrancaImediata) -> ApiRequest<CobrancaImediata> {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);
        self.inner.request_with_headers(Method::GET, &*endpoint, payload)
    }
    pub fn revisar_cobranca_txid(&self, txid: String, payload: CobrancaImediata) -> ApiRequest<CobrancaImediata> {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);
        self.inner.request_with_headers(Method::POST, &*endpoint, payload)
    }

    /// Criar uma cobrança imediata.
    /// Diferente de `criar_cobranca_imediata`, o `txid` é definido pelo PSP.
    pub fn criar_cobranca_imediata(&self, payload: CobrancaImediata) -> ApiRequest<CobrancaImediata> {
        let endpoint = format!("{}/cob", &*self.inner.base_endpoint);
        self.inner.request_with_headers(Method::POST, &*endpoint, payload)
    }

    pub fn consultar_cobrancas(&self) -> RequestBuilder {
        let endpoint = format!("{}/cob", &*self.inner.base_endpoint);
        self.inner.inner_client.request(Method::GET, &endpoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_value() {
        let new_value = Valor::new(512.614, false);
        println!(": {:?}", new_value);

        let new_integer_value = Valor::new(400f64, false);
        println!(": {:?}", new_integer_value);
    }
}
