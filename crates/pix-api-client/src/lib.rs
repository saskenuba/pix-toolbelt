//! You don't need to wrap `PixClient` into an `Arc`, since its inner client is already wrapped.
//!
//! ## Note
//!
//! You must take care of manually renewing your oauth token. This is accomplished easily
//! with helper functions provided by the `PixClient`.
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
pub use pix_brcode::qr_dinamico::PixDinamicoSchema;
pub use reqwest::header;

use std::marker::PhantomData;
use std::sync::Arc;

use crate::errors::{ApiResult, PixError};
use arc_swap::ArcSwap;
use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest::{Certificate, Client, Identity, Method, Request, RequestBuilder, StatusCode};
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
    inner_client: Client,
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
        let identity = Identity::from_pkcs12_der(&*certificate, "").expect("Invalid certificate");
        let mut default_headers = HeaderMap::new();

        custom_headers(&mut default_headers);

        let client = Client::builder().identity(identity).https_only(true).build().unwrap();

        Self {
            inner_client: client,
            headers: ArcSwap::from_pointee(default_headers),
            certificate,
            base_endpoint: endpoint.to_string(),
        }
    }

    /// Call this method in order to change the value of your `Authorization` header.
    ///
    /// For Bearer: `format!("Bearer {}", token)`
    ///
    /// For Basic: `format!("Basic {}:{}", id, secret)`
    ///
    /// This is usually done after you fetch the oauth token.
    pub fn swap_authorization_token(&self, authorization_header_value: String) {
        let mut stored_header = HeaderMap::new();
        stored_header.insert(header::AUTHORIZATION, authorization_header_value.parse().unwrap());

        self.headers.store(Arc::new(stored_header));
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
        let request = self
            .inner_client
            .request(method, endpoint)
            .headers(inner_headers.clone())
            .json(&payload)
            .build()
            .unwrap();

        ApiRequest::new(&self, request)
    }
}

#[derive(Debug)]
pub struct ApiRequest<'a, Response> {
    client: &'a PixClient,
    request: Request,
    response_type: PhantomData<Response>,
}

impl<'a, T> ApiRequest<'a, T> {
    fn new(client: &'a PixClient, request: Request) -> ApiRequest<T> {
        Self {
            client,
            request,
            response_type: Default::default(),
        }
    }
}

#[async_trait]
impl<ResponseType> Executor<ResponseType> for ApiRequest<'_, ResponseType>
where
    ResponseType: DeserializeOwned + Send,
{
    async fn execute(self) -> ApiResult<ResponseType> {
        let body = self
            .request
            .body()
            .map(|x| x.as_bytes().map(|x| String::from_utf8(Vec::from(x)).unwrap()))
            .flatten();
        println!("{:?}", body);

        let result = self.client.inner_client.execute(self.request).await?;
        let status_code = result.status();

        let text = result.text().await?;
        log::info!("{}", text);

        if !status_code.is_success() {
            return match status_code {
                StatusCode::UNAUTHORIZED => Err(PixError::InvalidCredentials),
                StatusCode::BAD_REQUEST => Err(PixError::PayloadError),
                _ => Err(PixError::Other(text)),
            };
        }

        serde_json::from_str::<ResponseType>(&*text).map_err(|e| e.into())
    }
}

#[async_trait]
pub trait Executor<T> {
    async fn execute(self) -> ApiResult<T>;
}
