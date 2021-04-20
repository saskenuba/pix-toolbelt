use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MerchantAccountInformation {
    #[serde(rename = "00")]
    merchant_gui: String,
    #[serde(rename = "01")]
    /// URL hosted by PSP
    location: String,
}

#[derive(Serialize, Deserialize)]
pub struct DynamicCalendar {}

#[derive(Serialize, Deserialize)]
/// Deve conter CPF ou CNPJ
pub struct DynamicDebtor {
    cpf: Option<String>,
    cnpj: Option<String>,
    nome: String,
}

#[derive(Serialize, Deserialize)]
struct Valor {
    original: Option<i32>,
    abatimento: Option<i32>,
    desconto: Option<i32>,
    juros: Option<i32>,
    multa: Option<i32>,
    #[serde(rename = "final")]
    /// Valor final da cobrança, considerados abatimentos, desconto, juros e multa.
    /// Ressalvado o campo original, se todos os demais campos estiverem zerados, o App do PSP do pagador deve exibir
    /// apenas o campo final.
    valor_final: i32,
}

#[derive(Serialize, Deserialize)]
/// Utilizado
struct PixDynamicPayload {
    #[serde(rename = "revisao")]
    revisao_cobranca: i64,
    #[serde(flatten)]
    calendario: DynamicCalendar,

    #[serde(flatten)]
    devedor: DynamicDebtor,
    #[serde(rename = "txid")]
    identificador_transacao: String,
}
