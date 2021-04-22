use emv_qrcps::EmvEncoder;
use std::borrow::Cow;

#[derive(EmvEncoder, Clone, Debug)]
struct MerchantAccountInformation<'a> {
    #[encoder(id = "00")]
    /// Versão do Payload QRCPS-MPM. Default em "01"
    pub gui: Cow<'a, str>,

    #[encoder(id = "21")]
    /// Instituição do usuário recebedor
    pub instituicao: Cow<'a, str>,

    #[encoder(id = "22")]
    /// Tipo de conta do usuário recebedor
    pub tipo_conta: Cow<'a, str>,

    #[encoder(id = "23")]
    /// Tipo de conta do usuário recebedor
    pub agencia: Cow<'a, str>,

    #[encoder(id = "24")]
    /// Numero da conta do usuário recebedor
    pub conta: Cow<'a, str>,
}

#[derive(EmvEncoder, Clone, Debug)]
struct PixDinamicoSchema<'a> {
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
