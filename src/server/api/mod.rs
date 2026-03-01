mod encrypt;
mod game;
mod keys;
mod request;
mod response;

pub use encrypt::*;
pub use game::*;
pub use keys::*;
pub use request::*;
pub use response::*;

use crate::error::{AppError, AppResult};

#[cfg(feature = "ssr")]
use reqwest::Client;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    backend_addr: String,
}

#[cfg(feature = "ssr")]
impl ApiClient {
    pub fn new(client: Client, backend_addr: String) -> ApiClient {
        Self {
            client,
            backend_addr,
        }
    }

    pub async fn create_game(&self, request: CreateGameRequest) -> AppResult<CreateGameResponse> {
        let url = format!("{}/games", self.backend_addr);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::custom(format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::custom(format!("{status}: {body}")));
        }

        response
            .json::<CreateGameResponse>()
            .await
            .map_err(|e| AppError::custom(format!("Parse error: {e}")))
    }

    pub async fn get_key(&self, request: GetKeyRequest) -> AppResult<KeyResponse> {
        let url = format!("{}/keys", self.backend_addr);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::custom(format!("Request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::custom(format!("{status}: {body}")));
        }

        response
            .json::<KeyResponse>()
            .await
            .map_err(|e| AppError::custom(format!("Parse error: {e}")))
    }
}
