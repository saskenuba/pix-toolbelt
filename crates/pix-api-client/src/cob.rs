use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{ApiRequest, PixClient};

pub struct CobEndpoint<'a> {
    inner: &'a PixClient,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
enum CobrancaStatus {
    ATIVA,
    CONCLUIDA,
    REMOVIDA_PELO_USUARIO_RECEBEDOR,
    REMOVIDA_PELO_PSP,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CobrancaImediata {
    pub calendario: Calendario,
    pub devedor: Devedor,
    /// Valor em string
    pub valor: Valor,

    /// Campo da chave PIX do recebedor desta cobrança.
    #[serde(rename = "chave")]
    pub chave_pix_recebedor: String,

    /// Id da Transação. Exclusivo como resposta.
    pub txid: Option<String>,

    #[serde(rename = "loc")]
    pub location: Option<Location>,

    /// Campo que será para que o pagador desta cobrança insira uma informação.
    /// Sua implementação depende do PSP do pagador. Não é garantido seu preenchimento. Verifique.
    #[serde(rename = "solicitacaoPagador")]
    pub solicitacao_pagador: Option<String>,
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
    /// A ausencia deste campo, que é o mesmo que 0, significa que a cobrança não poderá ter seu valor alterado.
    /// No caso do valor de 1, significa que o valor final poderá ser alterado pelo pagador.
    #[serde(rename = "modalidadeAlteracao")]
    pub permite_alteracao: Option<i32>,
}

impl Valor {
    // Valor com no máximo duas casas decimais. Caso houver mais que 2 casas, o valor será truncado.
    pub fn new(valor_original: f64, permite_alteracao: bool) -> Valor {
        Self {
            original: format!("{:.2}", valor_original),
            permite_alteracao: Some(permite_alteracao as i32),
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
        CobEndpoint { inner: &self }
    }
}

impl<'a> CobEndpoint<'a> {
    pub fn criar_cobranca_txid(&self, txid: String, payload: CobrancaImediata) -> RequestBuilder {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);

        self.inner.client.request(Method::PUT, endpoint)
    }

    pub fn consultar_cobranca_txid(&self, txid: String, payload: CobrancaImediata) -> RequestBuilder {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);
        self.inner.client.request(Method::GET, endpoint)
    }
    pub fn revisar_cobranca_txid(&self, txid: String, payload: CobrancaImediata) -> ApiRequest<CobrancaImediata> {
        let endpoint = format!("{}/cob/{}", &*self.inner.base_endpoint, txid);
        let request = self.inner.client.request(Method::POST, endpoint);

        ApiRequest::new(request)
    }

    /// Criar uma cobrança imediata.
    /// Diferente de `criar_cobranca_imediata`, o `txid` é definido pelo PSP.
    pub fn criar_cobranca_imediata(&self, payload: CobrancaImediata) -> ApiRequest<CobrancaImediata> {
        let endpoint = format!("{}/cob", &*self.inner.base_endpoint);
        let request = self.inner.client.request(Method::POST, endpoint);

        ApiRequest::new(request)
    }

    pub fn consultar_cobrancas(&self, payload: CobrancaImediata) -> RequestBuilder {
        let endpoint = format!("{}/cob", &*self.inner.base_endpoint);

        self.inner.client.request(Method::GET, &self.inner.base_endpoint)
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

        let payload = CobrancaImediata {
            calendario: Default::default(),
            devedor: Default::default(),
            valor: Default::default(),
            chave_pix_recebedor: "".to_string(),
            txid: None,
            location: None,
            solicitacao_pagador: None,
            info_adicionais: None,
        };

        let teste = client.cob().criar_cobranca_imediata(payload).execute().await;
    }

    #[test]
    fn t_value() {
        let new_value = Valor::new(512.614, false);
        println!(": {:?}", new_value);

        let new_integer_value = Valor::new(400f64, false);
        println!(": {:?}", new_integer_value);
    }
}
