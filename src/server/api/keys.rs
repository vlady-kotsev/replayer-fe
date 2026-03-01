use leptos::prelude::*;

use crate::error::{AppError, AppResult};
use crate::server::{GetKeyRequest, KeyResponse};

#[server]
pub async fn get_key(
    name: String,
    developer: String,
    player: String,
    signature: String,
    valid_period: i64,
) -> AppResult<KeyResponse> {
    use crate::config::Config;
    use crate::server::ApiClient;
    use solana_keypair::Signature;
    use solana_pubkey::Pubkey;

    let api_client =
        use_context::<ApiClient>().ok_or(AppError::custom("Can't get ApiClient context"))?;
    let config = use_context::<Config>().ok_or(AppError::custom("Can't get Config context"))?;

    let request = GetKeyRequest {
        name: name.clone(),
        developer: developer.clone(),
        player: player.clone(),
        signature,
        valid_period,
    };

    let response = api_client.get_key(request).await?;

    let developer_key = Pubkey::from_str_const(&developer);
    let player_key = Pubkey::from_str_const(&player);

    let payload = [
        &response.valid_period.to_le_bytes()[..],
        name.as_bytes(),
        developer_key.as_ref(),
        player_key.as_ref(),
    ]
    .concat();

    let sig_bytes: [u8; 64] = bs58::decode(&response.signature)
        .into_vec()
        .map_err(|e| AppError::custom(format!("Invalid signature encoding: {e}")))?
        .try_into()
        .map_err(|_| AppError::custom("Invalid signature length"))?;

    let signature = Signature::from(sig_bytes);
    if !signature.verify(config.app.backend_signer.as_ref(), &payload) {
        return Err(AppError::custom("Invalid backend signature"));
    }

    Ok(response)
}
