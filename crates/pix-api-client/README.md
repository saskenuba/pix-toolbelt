# pix-api-client

You don't need to wrap the client into an `Arc`, since its inner reqwest client already is wrapped.

## Example: Creating and calling an endpoint

```rust

use pix_api_client::cob::CobrancaImediata;
use pix_api_client::{Executor, PixClient};



let mut cert_buffer = Vec::new();
File::open("my_cert.pem")?.read_to_end(&mut cert_buffer)?;

let pix_client = PixClient::new("https://my-compliant-endpoint/pix/v2", "client-id", "client-secret", cert_buffer);

let payload = CobrancaImediata::default();
let response = pix_client
    .webhook()
    .criar_por_chave(
        "minha-chave-pix".to_string(),
        "https://pix.example.com/api/webhook/".to_string(),
    )
    .execute();


}
```

## Example: Create a new QRCode from a create immediate transaction endpoint
```rust

use pix_api_client::cob::{CobrancaImediata, Devedor};
use pix_api_client::{Executor, PixClient};
use pix_brcode::qr_dinamico::PixDinamicoSchema;



let pix_client = PixClient::new("https://my-compliant-endpoint/pix/v2", "client-id", "client-secret", cert_buffer);

let devedor = Devedor::new_pessoa_fisica("00000000000".to_string(), "Fulano de tal".to_string());
let payload = CobrancaImediata::new(10.25, "my-key".to_string(), devedor);

let response = pix_client
    .cob()
    .criar_cobranca_imediata(payload)
    .execute()
    .await;

let pix: String = PixDinamicoSchema::from(response).serialize_with_src();


}
```

License: MIT
