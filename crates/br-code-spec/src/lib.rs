//! Described on: https://www.bcb.gov.br/content/estabilidadefinanceira/pix/Regulamento_Pix/II_ManualdePadroesparaIniciacaodoPix_versao2-3-0.pdf

use crc::{Algorithm, Crc, CRC_16_GENIBUS, CRC_16_XMODEM};
use serde::{Deserialize, Serialize};

use br_code_spec_derive::BrCodeEncoder;
use helpers::Size;
use json_payload::{DynamicCalendar, DynamicDebtor};

pub(crate) mod helpers;
mod json_payload;
mod lexer;

#[derive(Serialize, Deserialize)]
enum QrCode {
    Dynamic,
    Static,
}

#[derive(BrCodeEncoder, Clone, Debug, Serialize, Deserialize)]
struct AdditionalData {
    #[encoder(id = "05")]
    /// Default de "***"
    /// Não deve ser preenchido no dinâmico, caso seja, deve ser ignorado.
    txid: String,
}

#[derive(BrCodeEncoder, Clone, Debug, Serialize, Deserialize)]
struct MerchantAccountInformation {
    #[encoder(id = "00")]
    merchant_gui: String,
    #[encoder(id = "01")]
    /// Não deve conter o prefixo de procolo, ex: http.
    /// Acesso deve ser após validações, e exclusivamente em HTTPS.
    merchant_url: String,
}

#[derive(BrCodeEncoder, Clone, Debug, Serialize, Deserialize)]
struct PixSchema {
    #[encoder(id = "00")]
    /// 1 byte
    pub format_indicator: String,

    #[encoder(id = "01")]
    /// Está presente para indicar que não deve ser iniciado mais de um pagamento com este mesmo QR Code.
    pub point_of_initiation_method: Option<String>,

    #[encoder(id = "26")]
    pub merchant_account_information: MerchantAccountInformation,

    #[encoder(id = "52")]
    pub merchant_category_code: String,
    #[encoder(id = "53")]
    pub transaction_currency: String,
    #[encoder(id = "58")]
    /// 2 Bytes
    pub country_code: String,

    #[encoder(id = "59")]
    /// Nome com tamanho
    pub merchant_name: String,

    #[encoder(id = "60")]
    /// Nome da cidade com tamanho
    pub merchant_city: String,

    #[encoder(id = "62")]
    pub additional_data: Option<AdditionalData>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bc_sample() -> &'static str {
        "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-426655440000 5204000053039865802BR5913Fulano de \
         Tal6008BRASILIA62070503***63041D3D"
    }

    #[derive(BrCodeEncoder, Debug, Clone)]
    struct SampleBrCode {
        #[encoder(id = "00")]
        format_indicator: String,

        #[encoder(id = "59")]
        merchant_name: String,
    }

    #[derive(BrCodeEncoder, Debug, Clone)]
    struct SampleBrCodeOption {
        #[encoder(id = "00")]
        format_indicator: String,

        #[encoder(id = "59")]
        merchant_name: String,

        #[encoder(id = "60")]
        merchant_category: Option<String>,
    }

    #[derive(BrCodeEncoder, Debug, Clone)]
    struct SampleBrCodeWithInnerOption {
        #[encoder(id = "00")]
        format_indicator: String,

        #[encoder(id = "59")]
        merchant_name: String,

        #[encoder(id = "62")]
        additional_data: InnerSample,
    }

    #[derive(BrCodeEncoder, Debug, Clone)]
    struct InnerSample {
        #[encoder(id = "00")]
        what_is_this: String,
    }

    #[test]
    fn t_non_inner_non_option() {
        let sample = SampleBrCode {
            format_indicator: "01".to_string(),
            merchant_name: "LTDA".to_string(),
        };
        assert_eq!(sample.serialize(), "0002015904LTDA");
    }

    #[test]
    fn t_non_inner_option() {
        let sample = SampleBrCodeOption {
            format_indicator: "01".to_string(),
            merchant_name: "LTDA".to_string(),
            merchant_category: None,
        };
        assert_eq!(sample.serialize(), "0002015904LTDA");
    }

    #[test]
    fn t_inner_option() {
        let inner = InnerSample {
            what_is_this: "01".to_string(),
        };

        let sample = SampleBrCodeWithInnerOption {
            format_indicator: "01".to_string(),
            merchant_name: "LTDA".to_string(),
            additional_data: inner,
        };

        assert_eq!(sample.serialize(), "0002015904LTDA6200000201");
    }
}
