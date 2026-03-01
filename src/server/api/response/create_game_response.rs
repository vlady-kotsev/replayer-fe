use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateGameResponse {
    pub encryption_key: String,
    pub nonce: String,
}
