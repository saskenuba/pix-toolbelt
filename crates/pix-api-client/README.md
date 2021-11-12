# Pix API Client

![Crate version on crates.io](https://img.shields.io/crates/v/pix-api-client)
![Crate documentation on docs.rs](https://img.shields.io/docsrs/pix-api-client)
![Crate license](https://img.shields.io/crates/l/pix-api-client)

A Rust PIX API client, [Bacen API Pix](https://github.com/bacen/pix-api)
compliant.

Before usage, check if your PSP is also compliant or this won't work.

## Usage

Add the following to your `Cargo.toml`:
```toml
[dependencies]
pix-api-client = "^0.2"
```

See the [documentation](https://docs.rs/pix-api-client) for detailed usage information.

### Notes

The inner client is wrapped with `Arc` ready for reuse.

You need to manually renew your OAuth token. This is accomplished easily with
helper functions provided by the `PixClient`.


## Examples

### Setup a ready to use client

```rust
use pix_api_client::cob::CobrancaImediata;
use pix_api_client::{Executor, PixClient};
use pix_api_client::header::AUTHORIZATION;

let mut cert_buffer = Vec::new();
File::open("my_cert.pem")?.read_to_end(&mut cert_buffer)?;

// format your headers the way your PSP expects it
// this is just an example
let pix_client = PixClient::new_with_custom_headers("https://my-pix-h", |headers| {
    let username = "my-id";
    let secret = "my-secret";
    let formatted_authorization = format!("{}:{}", username, secret);
    let encoded_auth = base64::encode(formatted_authorization);

    // and then insert it
    headers.insert(header::AUTHORIZATION, encoded_auth.parse().unwrap()).unwrap();
}, cert_buffer);

let oauth_response = pix_client
    .oauth()
    .autenticar()
    .execute()
    .await?;

// retrieve your new access token, and store it as your new authorization header
let token = oauth_response.access_token;
pix_client.swap_authorization_token(token.to_string());

// That's it! Your client is ready for any further api calls.

```

### Create a new QRCode from `criar_cobranca_imediata` endpoint

```rust
use pix_api_client::cob::{CobrancaImediata, Devedor};
use pix_api_client::{Executor, PixClient, PixDinamicoSchema};
use pix_api_client::extensions::FromResponse;


let devedor = Devedor::new_pessoa_fisica("00000000000".to_string(), "Fulano de tal".to_string());
let payload = CobrancaImediata::new(10.25, "my-key".to_string(), devedor);

let response: CobrancaImediata = pix_client
    .cob()
    .criar_cobranca_imediata(payload)
    .execute()
    .await?;

let pix: String = PixDinamicoSchema::from_cobranca_imediata_basic(response, "minha loja", "minha cidade").serialize_with_src();
assert_eq!(pix, "00020104141234567890123426580014BR.GOV.BCB.PIX0136123e4567-e12b-12d1-a456-42665544000027300012BR.COM. OUTRO011001234567895204000053039865406123.455802BR5917NOME DO RECEBEDOR6008BRASILIA61087007490062190515RP12345678-201980390012BR.COM.OUTRO01190123.ABCD.3456.WXYZ6304AD38");
```

License: MIT
