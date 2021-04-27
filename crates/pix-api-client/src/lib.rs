//! You don't need to wrap the client into an `Arc`, since its inner reqwest client already is wrapped.
//!
//! # Example: Creating and calling an endpoint
//!
//! ```no_run
//! # use std::fs::File;
//! # use std::io::Read;
//!
//! use pix_api_client::cob::CobrancaImediata;
//! use pix_api_client::{Executor, PixClient};
//!
//! # fn main() -> Result<(), anyhow::Error> {
//!
//!
//! let mut cert_buffer = Vec::new();
//! File::open("my_cert.pem")?.read_to_end(&mut cert_buffer)?;
//!
//! let pix_client = PixClient::new("https://my-compliant-endpoint/pix/v2", "client-id", "client-secret", cert_buffer);
//!
//! let response = pix_client
//!     .webhook()
//!     .criar_por_chave(
//!         "minha-chave-pix".to_string(),
//!         "https://pix.example.com/api/webhook/".to_string(),
//!     )
//!     .execute();
//!
//! # Ok(())
//!
//! }
//! ```
//!
//! # Example: Create a new QRCode from a create immediate transaction endpoint
//! ```no_run
//! # use std::fs::File;
//! # use std::io::Read;
//!
//! use pix_api_client::cob::{CobrancaImediata, Devedor};
//! use pix_api_client::{Executor, PixClient};
//! use pix_brcode::qr_dinamico::PixDinamicoSchema;
//! use pix_api_client::extensions::FromResponse;
//!
//! # async fn doc_test() -> Result<(), anyhow::Error> {
//!
//! # let mut cert_buffer = Vec::new();
//! # File::open("my_cert.pem")?.read_to_end(&mut cert_buffer)?;
//!
//! let pix_client = PixClient::new("https://my-compliant-endpoint/pix/v2", "client-id", "client-secret", cert_buffer);
//!
//! let devedor = Devedor::new_pessoa_fisica("00000000000".to_string(), "Fulano de tal".to_string());
//! let payload = CobrancaImediata::new(10.25, "my-key".to_string(), devedor);
//!
//! let response: CobrancaImediata = pix_client
//!     .cob()
//!     .criar_cobranca_imediata(payload)
//!     .execute()
//!     .await;
//!
//! let pix: String = PixDinamicoSchema::from_cobranca_imediata_basic(response, "minha loja", "minha cidade").serialize_with_src();
//!
//! # Ok(())
//!
//! }
//! ```

use std::marker::PhantomData;

use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::{header, Certificate, Client, RequestBuilder};
use serde::de::DeserializeOwned;

pub mod cob;
pub mod errors;
pub mod webhook;

pub mod extensions;

#[derive(Debug)]
struct PixClientBuilder {
    username: String,
    secret: String,
    certificate: Vec<u8>,
}

impl PixClientBuilder {}

/// A strongly typed client for performing requests to a pix-api compliant provider.
///
/// # Example
#[derive(Debug)]
pub struct PixClient {
    client: Client,
    certificate: Vec<u8>,

    base_endpoint: String,
}

impl PixClient {
    /// Creates a new `PixClient` with customized headers.
    ///
    /// This is specially useful, since how the authorization is encoded varies between PSP's.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// # use std::io::{Read, Error};
    ///
    /// use pix_api_client::PixClient;
    /// use reqwest::header;
    ///
    /// # fn teste() -> Result<(), anyhow::Error> {
    ///     let mut cert_buffer = Vec::new();
    ///     File::open("my_cert.pem")?.read_to_end(&mut cert_buffer)?;
    ///
    ///     let username = "my-id";
    ///     let secret = "my-secret";
    ///     let formatted_authorization = format!("{}:{}", username, secret);
    ///     let encoded_auth = base64::encode(formatted_authorization);
    ///
    ///     let pix_client = PixClient::new_with_custom_headers("https://*", |headers| {
    ///         headers.insert(header::AUTHORIZATION, encoded_auth.parse().unwrap()).unwrap();
    ///     }, cert_buffer);
    ///
    /// #   Ok(())
    /// }
    /// ```
    pub fn new_with_custom_headers<F>(endpoint: &str, mut custom_headers: F, certificate: Vec<u8>) -> PixClient
    where
        F: FnMut(&mut HeaderMap),
    {
        let cert = Certificate::from_pem(&certificate).unwrap();
        let mut default_headers = HeaderMap::new();

        custom_headers(&mut default_headers);

        let client = Client::builder()
            .add_root_certificate(cert)
            .default_headers(default_headers)
            .build()
            .unwrap();

        Self {
            client,
            certificate,
            base_endpoint: endpoint.to_string(),
        }
    }
    pub fn new(endpoint: &str, username: &str, secret: &str, certificate: Vec<u8>) -> PixClient {
        let formatted_authorization = format!("{}:{}", username, secret);
        let encoded_auth = base64::encode(formatted_authorization);
        let cert = Certificate::from_pem(&certificate).unwrap();

        let mut default_headers = HeaderMap::new();
        default_headers
            .insert(header::AUTHORIZATION, (&*encoded_auth).parse().unwrap())
            .unwrap();

        let client = Client::builder()
            .add_root_certificate(cert)
            .default_headers(default_headers)
            .build()
            .unwrap();

        Self {
            client,
            certificate,
            base_endpoint: endpoint.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ApiRequest<T> {
    request: RequestBuilder,
    response_type: PhantomData<T>,
}

impl<T> ApiRequest<T> {
    fn new(request: RequestBuilder) -> ApiRequest<T> {
        Self {
            request,
            response_type: Default::default(),
        }
    }
}

#[async_trait]
impl<T> Executor<T> for ApiRequest<T>
where
    T: DeserializeOwned + Send,
{
    async fn execute(self) -> T {
        let result = self.request.send().await.unwrap();
        let deserialized_response = result.json::<T>().await.unwrap();
        deserialized_response
    }
}

#[async_trait]
pub trait Executor<T> {
    async fn execute(self) -> T;
}
