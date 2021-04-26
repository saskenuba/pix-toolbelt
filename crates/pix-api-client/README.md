# pix-api-client


You don't need to wrap the client into an `Arc`, since its inner reqwest client already is wrapped.

## Example
```rust
use std::fs::File;
use std::io::Read;

use pix_api_client::cob::CobPayload;
use pix_api_client::{Executor, PixClient};

let mut cert_buffer = Vec::new();
File::open("my_cert.pem")?.read_to_end(&mut cert_buffer)?;

let pix_client = PixClient::new("https://*", "client-id", "client-secret", cert_buffer);

let payload = CobPayload::default();
let response = pix_client
    .webhook()
    .criar_por_chave(
        "minha-chave-pix".to_string(),
        "https://pix.example.com/api/webhook/".to_string(),
    )
    .execute();
```

License: MIT
