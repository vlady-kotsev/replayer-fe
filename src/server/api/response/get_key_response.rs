use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyResponse {
    pub encryption_key: String,
    pub nonce: String,
    pub signature: String,
    pub valid_period: i64,
}
