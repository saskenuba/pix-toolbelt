use std::borrow::Cow;

use pix_brcode::qr_dinamico::PixDinamicoSchema;

use crate::cob::CobrancaImediata;

pub trait FromResponse<'a, T> {
    fn from_cobranca_imediata_basic<MN: Into<Cow<'a, str>>, MC: Into<Cow<'a, str>>>(
        cob: CobrancaImediata,
        merchant_name: MN,
        merchant_city: MC,
    ) -> T;
}

impl<'a> FromResponse<'a, PixDinamicoSchema<'a>> for PixDinamicoSchema<'a> {
    fn from_cobranca_imediata_basic<MN: Into<Cow<'a, str>>, MC: Into<Cow<'a, str>>>(
        cob: CobrancaImediata,
        merchant_name: MN,
        merchant_city: MC,
    ) -> PixDinamicoSchema<'a> {
        let valor_original: Cow<_> = cob.valor.original.into();
        let location = cob.location.unwrap();

        PixDinamicoSchema::standard(merchant_name, merchant_city, valor_original, location.url)
    }
}
