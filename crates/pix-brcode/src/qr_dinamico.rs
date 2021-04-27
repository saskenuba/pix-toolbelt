use emv_qrcps::EmvEncoder;
use std::borrow::Cow;

#[derive(EmvEncoder, Clone, Debug)]
pub struct AdditionalData<'a> {
    #[encoder(id = "05")]
    /// Default de "***"
    /// Não deve ser preenchido no dinâmico, caso seja, deve ser ignorado.
    txid: Cow<'a, str>,
}

#[derive(EmvEncoder, Clone, Debug)]
pub struct MerchantAccountInformation<'a> {
    /// Defaults to "br.gov.bcb.pix"
    #[encoder(id = "00")]
    pub merchant_gui: Cow<'a, str>,

    /// Não deve conter o prefixo de procolo, ex: http.
    /// Acesso deve ser após validações, e exclusivamente em HTTPS.
    #[encoder(id = "25")]
    pub merchant_location_url: Cow<'a, str>,
}

#[derive(EmvEncoder, Clone, Debug)]
pub struct PixDinamicoSchema<'a> {
    /// Versão do Payload QRCPS-MPM. Default em "01"
    #[encoder(id = "00")]
    pub format_indicator: Cow<'a, str>,

    /// Está presente para indicar que não deve ser iniciado mais de um pagamento com este mesmo QR Code.
    /// Defaults em "12".
    #[encoder(id = "01")]
    pub point_of_initiation_method: Option<Cow<'a, str>>,

    #[encoder(id = "26")]
    pub merchant_account_information: MerchantAccountInformation<'a>,

    #[encoder(id = "52")]
    pub merchant_category_code: Cow<'a, str>,

    /// Defaults to "968", as BRL.
    #[encoder(id = "53")]
    pub transaction_currency: Cow<'a, str>,

    #[encoder(id = "54")]
    pub transaction_amount: Option<Cow<'a, str>>,

    /// ISO3166-1 alpha 2 Country Code
    /// Defaults to "BR"
    #[encoder(id = "58")]
    pub country_code: Cow<'a, str>,

    /// Recipient's name
    #[encoder(id = "59")]
    pub merchant_name: Cow<'a, str>,

    /// City where transaction occurred
    #[encoder(id = "60")]
    pub merchant_city: Cow<'a, str>,

    #[encoder(id = "61")]
    pub postal_code: Option<Cow<'a, str>>,

    #[encoder(id = "62")]
    pub additional_data: AdditionalData<'a>,
}

impl<'a> PixDinamicoSchema<'a> {
    pub fn standard() -> Self {
        let merchant = MerchantAccountInformation {
            merchant_gui: "br.gov.bcb.pix".into(),
            merchant_location_url: Default::default(),
        };

        let additional_data = AdditionalData { txid: "***".into() };

        Self {
            format_indicator: "01".into(),
            point_of_initiation_method: Some("12".into()),

            merchant_account_information: merchant,
            merchant_category_code: Default::default(),
            transaction_currency: "986".into(),
            transaction_amount: None,
            country_code: "BR".into(),
            merchant_name: Default::default(),
            merchant_city: Default::default(),
            postal_code: None,
            additional_data,
        }
    }
}
