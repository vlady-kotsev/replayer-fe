use serde::Serialize;

#[derive(Serialize)]
pub struct GetKeyRequest {
    pub name: String,
    pub developer: String,
    pub player: String,
    pub signature: String,
    pub valid_period: i64,
}
