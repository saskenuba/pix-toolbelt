use std::borrow::Cow;

use pix_brcode::qr_dinamico::PixDinamicoSchema;

use crate::cob::{CobrancaImediata, Location};

impl From<CobrancaImediata> for PixDinamicoSchema<'_> {
    fn from(cob: CobrancaImediata) -> Self {
        let mut new_pix_qrcode = PixDinamicoSchema::standard();
        let valor_original: Cow<_> = cob.valor.original.into();

        let location = cob.location.unwrap();
        new_pix_qrcode.merchant_account_information.merchant_location_url = location.url.into();
        new_pix_qrcode.transaction_amount = Some(valor_original);

        new_pix_qrcode
    }
}
