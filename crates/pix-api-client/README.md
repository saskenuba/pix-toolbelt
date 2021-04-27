# pix-api-client

You don't need to wrap the client into an `Arc`, since its inner reqwest client already is wrapped.

### Note

You need to take care munually of renewing your oauth token. This is accomplished very easily
with the helper functions provided by the `PixClient`.

## Example: Create a new client and fetch the oauth token

```rust
use pix_api_client::cob::CobrancaImediata;
use pix_api_client::{Executor, PixClient};
use reqwest::header;


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
    .await;

// retrieve your new access token, and store it as your new authorization header
let token = oauth_response.access_token;
pix_client.swap_authorization_token(token.to_string());

// Your client is ready for any further calls.


}
```

## Example: Create a new QRCode from a create immediate transaction endpoint
```rust
use pix_api_client::cob::{CobrancaImediata, Devedor};
use pix_api_client::{Executor, PixClient};
use pix_brcode::qr_dinamico::PixDinamicoSchema;
use pix_api_client::extensions::FromResponse;


let devedor = Devedor::new_pessoa_fisica("00000000000".to_string(), "Fulano de tal".to_string());
let payload = CobrancaImediata::new(10.25, "my-key".to_string(), devedor);

let response: CobrancaImediata = pix_client
    .cob()
    .criar_cobranca_imediata(payload)
    .execute()
    .await;

let pix: String = PixDinamicoSchema::from_cobranca_imediata_basic(response, "minha loja", "minha cidade").serialize_with_src();


}
```


License: MIT
