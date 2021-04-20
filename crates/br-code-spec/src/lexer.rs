use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(IntoStaticStr, EnumIter)]
enum HasChildren {
    #[strum(serialize = "26")]
    MerchantInfo,
    #[strum(serialize = "62")]
    AdditionalInformation,
}

#[derive(IntoStaticStr, EnumIter)]
enum LookupTable {
    #[strum(serialize = "01")]
    PointOfInitiationMethod,
    #[strum(serialize = "26")]
    MerchantAccountInformation,
    #[strum(serialize = "53")]
    TransactionCurrency,
    #[strum(serialize = "58")]
    CountryCode,
    #[strum(serialize = "59")]
    MerchantName,
    #[strum(serialize = "60")]
    MerchantCity,
    #[strum(serialize = "62")]
    AdditionalData,
    #[strum(serialize = "63")]
    Crc16,
}

fn parser(pix_string: &str) {
    let mut cursor = pix_string;

    while let Some((header_id, content_length, rest)) = header_length_remaining(cursor) {
        println!("{:?}{:?}", header_id, content_length);

        let length_index = usize::from_str(content_length).unwrap();
        let content = &rest[..length_index];
        let remaining = &rest[length_index..];

        println!("content: {:?}", content);
        println!("remaining: {:?}", remaining);

        if HasChildren::iter()
            .map(|str| str.into())
            .any(|header_with_son: &str| header_with_son == header_id)
        {
            let mut inner_content = content;
            println!("Header has son.");

            while let Some((header, length, remaining)) = header_length_remaining(inner_content) {
                println!("{:?}{:?}", header, length);

                let length_index = usize::from_str(length).unwrap();
                let content = &remaining[..length_index];
                let remaining = &remaining[length_index..];
                println!("content: {:?}", content);
                println!("remaining: {:?}", remaining);

                inner_content = remaining;
            }
        }

        cursor = remaining;
    }
}

/// Returns (header_id, inner_length, and rest)
fn header_length_remaining(pix_string: &str) -> Option<(&str, &str, &str)> {
    pix_string
        .get(4..)
        .map(|remaining| (&pix_string[..2], &pix_string[2..4], remaining))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers;

    fn bacen_static_sample() -> &'static str {
        "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Fulano de Tal6008BRASILIA62070503***63041D3D"
    }

    fn bacen_dynamic_sample() -> &'static str {
        "00020101021226700014br.gov.bcb.pix2548pix.example.com/8b3da2f39a4140d1a91abd93113bd4415204000053039865802BR5913Fulano de Tal6008BRASILIA62070503***630464E4"
    }

    fn generated_sample() -> &'static str {
        "00020126530014br.gov.bcb.pix0119saskenuba@gmail.com0208[Pix.ae]520400005303986540550.005802BR5903Pix6003Pix62070503***63048287"
    }

    #[test]
    fn t_dynamic_sample() {
        parser(bacen_dynamic_sample())
    }

    #[test]
    fn t_generated_sample() {
        parser(generated_sample())
    }

    #[test]
    fn t_static_sample() {
        parser(bacen_static_sample())
    }

    #[test]
    fn string_validation() {
        let validate = helpers::calculate_crc16(bacen_static_sample());
    }
}
