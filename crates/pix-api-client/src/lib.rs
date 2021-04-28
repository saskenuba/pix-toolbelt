//! You don't need to wrap the client into an `Arc`, since its inner reqwest client already is wrapped.
//!
//! ## Note
//!
//! You need to take care munually of renewing your oauth token. This is accomplished very easily
//! with the helper functions provided by the `PixClient`.
//!
//! # Example: Create a new client and fetch the oauth token
//!
//! ```no_run
//! # use std::fs::File;
//! # use std::io::Read;
//! use pix_api_client::cob::CobrancaImediata;
//! use pix_api_client::{Executor, PixClient};
//! use reqwest::header;
//!
//! # async fn teste() -> Result<(), anyhow::Error> {
//!
//! let mut cert_buffer = Vec::new();
//! File::open("my_cert.pem")?.read_to_end(&mut cert_buffer)?;
//!     
//! // format your headers the way your PSP expects it
//! // this is just an example
//! let pix_client = PixClient::new_with_custom_headers("https://my-pix-h", |headers| {
//!     let username = "my-id";
//!     let secret = "my-secret";
//!     let formatted_authorization = format!("{}:{}", username, secret);
//!     let encoded_auth = base64::encode(formatted_authorization);
//!
//!     // and then insert it
//!     headers.insert(header::AUTHORIZATION, encoded_auth.parse().unwrap()).unwrap();
//! }, cert_buffer);
//!
//! let oauth_response = pix_client
//!     .oauth()
//!     .autenticar()
//!     .execute()
//!     .await?;
//!
//! // retrieve your new access token, and store it as your new authorization header
//! let token = oauth_response.access_token;
//! pix_client.swap_authorization_token(token.to_string());
//!
//! // Your client is ready for any further api calls.
//!
//! # Ok(())
//! # }
//! ```
//!
//! # Example: Create a new QRCode from a create immediate transaction endpoint
//! ```no_run
//! # use std::fs::File;
//! # use std::io::Read;
//! use pix_api_client::cob::{CobrancaImediata, Devedor};
//! use pix_api_client::{Executor, PixClient};
//! use pix_brcode::qr_dinamico::PixDinamicoSchema;
//! use pix_api_client::extensions::FromResponse;
//!
//! # async fn doc_test() -> Result<(), anyhow::Error> {
//! # let mut cert_buffer = Vec::new();
//! # File::open("my_cert.pem")?.read_to_end(&mut cert_buffer)?;
//! # let pix_client = PixClient::new("https://my-compliant-endpoint/pix/v2", "client-id", "client-secret", cert_buffer);
//!
//! let devedor = Devedor::new_pessoa_fisica("00000000000".to_string(), "Fulano de tal".to_string());
//! let payload = CobrancaImediata::new(10.25, "my-key".to_string(), devedor);
//!
//! let response: CobrancaImediata = pix_client
//!     .cob()
//!     .criar_cobranca_imediata(payload)
//!     .execute()
//!     .await?;
//!
//! let pix: String = PixDinamicoSchema::from_cobranca_imediata_basic(response, "minha loja", "minha cidade").serialize_with_src();
//!
//! # Ok(())
//! # }
//! ```
//!
pub use reqwest::header;

use std::marker::PhantomData;
use std::sync::Arc;

use crate::errors::{ApiResult, PixError};
use arc_swap::ArcSwap;
use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::{Certificate, Client, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod cob;
pub mod errors;
pub mod webhook;

pub mod extensions;

pub mod oauth;

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
    /// Headers used for every request
    headers: ArcSwap<HeaderMap>,
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
    /// # }
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
            .default_headers(default_headers.clone())
            .build()
            .unwrap();

        Self {
            client,
            headers: ArcSwap::from_pointee(default_headers),
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
            .default_headers(default_headers.clone())
            .build()
            .unwrap();

        Self {
            client,
            headers: ArcSwap::from_pointee(default_headers),
            certificate,
            base_endpoint: endpoint.to_string(),
        }
    }

    /// Call this method in order to change the value of your `Authorization` header.
    ///
    /// This is usually done after you fetch the oauth token.
    pub fn swap_authorization_token(&self, authorization_header_value: String) {
        let mut default_headers = HeaderMap::new();
        default_headers
            .insert(header::AUTHORIZATION, authorization_header_value.parse().unwrap())
            .unwrap();

        self.headers.store(Arc::new(default_headers));
    }

    fn request_with_headers<Payload, Response>(
        &self,
        method: Method,
        endpoint: &str,
        payload: Payload,
    ) -> ApiRequest<Response>
    where
        Payload: Serialize,
        Response: DeserializeOwned,
    {
        let inner_headers = &**self.headers.load();
        let request = self.client.request(method, endpoint).headers(inner_headers.clone());

        ApiRequest::new(request.json(&payload))
    }
}

#[derive(Debug)]
pub struct ApiRequest<Response> {
    request: RequestBuilder,
    response_type: PhantomData<Response>,
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
impl<ResponseType> Executor<ResponseType> for ApiRequest<ResponseType>
where
    ResponseType: DeserializeOwned + Send,
{
    async fn execute(self) -> ApiResult<ResponseType> {
        let result = self.request.send().await?;
        let status_code = result.status();

        if !status_code.is_success() {
            match status_code {
                StatusCode::UNAUTHORIZED => return Err(PixError::InvalidCredentials),
                StatusCode::BAD_REQUEST => return Err(PixError::PayloadError),
                _ => {}
            }
        }

        let deserialized_response = result
            .json::<ResponseType>()
            .await
            .map_err(|_| PixError::NonCompliantResponse)?;

        Ok(deserialized_response)
    }
}

#[async_trait]
pub trait Executor<T> {
    async fn execute(self) -> ApiResult<T>;
}
