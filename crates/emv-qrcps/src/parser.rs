use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

use emv_qrcps_derive::EmvEncoder;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(IntoStaticStr, EnumIter)]
pub enum HasChildren {
    #[strum(serialize = "26")]
    MerchantInfo,
    #[strum(serialize = "62")]
    AdditionalInformation,
}

#[derive(EmvEncoder, Clone, Debug, PartialEq)]
struct MerchantAccountInformation<'a> {
    #[encoder(id = "00")]
    merchant_gui: Cow<'a, str>,
    #[encoder(id = "01")]
    /// Não deve conter o prefixo de procolo, ex: http.
    /// Acesso deve ser após validações, e exclusivamente em HTTPS.
    merchant_url: Cow<'a, str>,
}

pub trait Parsed<'a> {
    fn from_lookup(map: &mut HashMap<&str, &'a str>) -> Self;
}

pub fn base_parser<'a, T>(source_str: &'a str) -> T
where
    T: Parsed<'a>,
{
    let mut cursor = source_str;
    let mut lookup = HashMap::new();

    while let Some((header_id, content_length, rest)) = header_length_remaining(cursor) {
        let length_index = usize::from_str(content_length).unwrap();
        let content = &rest[..length_index];
        let remaining = &rest[length_index..];

        lookup.insert(header_id, content);

        if HasChildren::iter()
            .map(|str| str.into())
            .any(|header_with_son: &str| header_with_son == header_id)
        {
            let mut inner_content = content;

            while let Some((_header, length, remaining)) = header_length_remaining(inner_content) {
                let length_index = usize::from_str(length).unwrap();
                let _content = &remaining[..length_index];
                let remaining = &remaining[length_index..];

                inner_content = remaining;
            }
        }

        cursor = remaining;
    }

    T::from_lookup(&mut lookup)
}

/// Returns (header_id, inner_length, and rest)
pub fn header_length_remaining(pix_string: &str) -> Option<(&str, &str, &str)> {
    pix_string
        .get(4..)
        .map(|remaining| (&pix_string[..2], &pix_string[2..4], remaining))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_merchant() -> &'static str {
        "0028123e4567-e12b-12d1-a456-42720102oi"
    }

    #[test]
    fn t_parser_simple() {
        let basic = MerchantAccountInformation {
            merchant_gui: "123e4567-e12b-12d1-a456-4272".into(),
            merchant_url: "oi".into(),
        };

        assert_eq!(basic, MerchantAccountInformation::from_str(sample_merchant()));
    }

    #[allow(dead_code)]
    fn bacen_static_sample() -> &'static str {
        "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Fulano de \
         Tal6008BRASILIA62070503***63041D3D"
    }

    #[allow(dead_code)]
    fn bacen_dynamic_sample() -> &'static str {
        "00020101021226700014br.gov.bcb.pix2548pix.example.com/\
         8b3da2f39a4140d1a91abd93113bd4415204000053039865802BR5913Fulano de Tal6008BRASILIA62070503***630464E4"
    }

    #[allow(dead_code)]
    fn generated_sample() -> &'static str {
        "00020126530014br.gov.bcb.pix0119saskenuba@gmail.com0208[Pix.ae]520400005303986540550.\
         005802BR5903Pix6003Pix62070503***63048287"
    }

    #[derive(EmvEncoder, Debug, Clone)]
    struct SampleBrCode<'a> {
        #[encoder(id = "00")]
        format_indicator: Cow<'a, str>,

        #[encoder(id = "59")]
        merchant_name: Cow<'a, str>,
    }

    #[derive(EmvEncoder, Debug, Clone)]
    struct SampleBrCodeOption<'a> {
        #[encoder(id = "00")]
        format_indicator: Cow<'a, str>,

        #[encoder(id = "59")]
        merchant_name: Cow<'a, str>,

        #[encoder(id = "60")]
        merchant_category: Option<Cow<'a, str>>,
    }

    #[derive(EmvEncoder, Debug, Clone)]
    struct SampleBrCodeWithInnerOption<'a> {
        #[encoder(id = "00")]
        format_indicator: Cow<'a, str>,

        #[encoder(id = "59")]
        merchant_name: Cow<'a, str>,

        #[encoder(id = "62")]
        additional_data: InnerSample<'a>,
    }

    #[derive(EmvEncoder, Debug, Clone)]
    struct InnerSample<'a> {
        #[encoder(id = "00")]
        what_is_this: Cow<'a, str>,
    }

    #[test]
    fn t_non_inner_non_option() {
        let sample = SampleBrCode {
            format_indicator: "01".into(),
            merchant_name: "LTDA".into(),
        };
        assert_eq!(sample.serialize(), "0002015904LTDA");
    }

    #[test]
    fn t_non_inner_option() {
        let sample = SampleBrCodeOption {
            format_indicator: "01".into(),
            merchant_name: "LTDA".into(),
            merchant_category: None,
        };
        assert_eq!(sample.serialize(), "0002015904LTDA");
    }

    #[test]
    fn t_inner_option() {
        let inner = InnerSample {
            what_is_this: "01".into(),
        };

        let sample = SampleBrCodeWithInnerOption {
            format_indicator: "01".into(),
            merchant_name: "LTDA".into(),
            additional_data: inner,
        };

        assert_eq!(sample.serialize(), "0002015904LTDA6206000201");
    }
}
