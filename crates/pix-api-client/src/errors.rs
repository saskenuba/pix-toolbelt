use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GenericErrorMessage {
    nome: String,
    mensagem: String,
}
