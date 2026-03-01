use leptos::prelude::*;

use crate::error::{AppError, AppResult};
use crate::server::{CreateGameRequest, CreateGameResponse};

#[server]
pub async fn create_game(
    name: String,
    developer: String,
    signature: String,
    valid_period: i64,
) -> AppResult<CreateGameResponse> {
    use crate::server::ApiClient;

    let api_client =
        use_context::<ApiClient>().ok_or(AppError::custom("Can't get ApiClient context"))?;

    let request = CreateGameRequest {
        name,
        player: developer.clone(),
        developer,
        signature,
        valid_period,
    };

    api_client.create_game(request).await
}
