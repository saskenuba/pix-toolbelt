//! Described on:
//! Pix Manual:      https://www.bcb.gov.br/content/estabilidadefinanceira/pix/Regulamento_Pix/II_ManualdePadroesparaIniciacaodoPix_versao2-3-0.pdf
//! BR Code Manual:  https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf
//! Arranjos QRCode: https://www.bcb.gov.br/content/estabilidadefinanceira/forumpireunioes/Anexo%20I%20-%20Padr%C3%B5es%20para%20Inicia%C3%A7%C3%A3o%20do%20PIX.pdf
//!
//! Múltiplos arranjos podem existir em um mesmo QR Code

use std::borrow::Cow;

use emv_qrcps::EmvEncoder;

mod json_payload;
pub mod qr_dinamico;
pub mod qr_estatico;

#[derive(EmvEncoder, Clone, Debug)]
struct AdditionalData<'a> {
    #[encoder(id = "05")]
    /// Default de "***"
    /// Não deve ser preenchido no dinâmico, caso seja, deve ser ignorado.
    txid: Cow<'a, str>,
}

#[derive(EmvEncoder, Clone, Debug)]
struct MerchantAccountInformation<'a> {
    #[encoder(id = "00")]
    merchant_gui: Cow<'a, str>,
    #[encoder(id = "01")]
    /// Não deve conter o prefixo de procolo, ex: http.
    /// Acesso deve ser após validações, e exclusivamente em HTTPS.
    merchant_url: Option<Cow<'a, str>>,
}

#[derive(EmvEncoder, Clone, Debug)]
struct PixSchema<'a> {
    #[encoder(id = "00")]
    /// Versão do Payload QRCPS-MPM. Default em "01"
    pub format_indicator: Cow<'a, str>,

    #[encoder(id = "01")]
    /// Está presente para indicar que não deve ser iniciado mais de um pagamento com este mesmo QR Code.
    pub point_of_initiation_method: Option<Cow<'a, str>>,

    #[encoder(id = "26")]
    pub merchant_account_information: MerchantAccountInformation<'a>,

    #[encoder(id = "52")]
    pub merchant_category_code: Cow<'a, str>,
    #[encoder(id = "53")]
    pub transaction_currency: Cow<'a, str>,
    #[encoder(id = "54")]
    pub transaction_amount: Option<Cow<'a, str>>,

    #[encoder(id = "58")]
    /// ISO3166-1 alpha 2 Country Code
    pub country_code: Cow<'a, str>,

    #[encoder(id = "59")]
    /// Recipient's name
    pub merchant_name: Cow<'a, str>,

    #[encoder(id = "60")]
    /// City where transaction occurred
    pub merchant_city: Cow<'a, str>,

    #[encoder(id = "61")]
    /// Postal Code
    pub postal_code: Option<Cow<'a, str>>,
    // #[encoder(id = "62")]
    // pub additional_data: Option<AdditionalData<'a>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM.OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38"
    }

    #[test]
    fn t_() {
        let pix_schema_read = PixSchema::from_str(sample());

        println!("schema: {:#?}", pix_schema_read);

        assert_eq!(pix_schema_read.serialize_with_src(), sample())
    }
}
