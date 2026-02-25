use leptos::prelude::*;
use solana_hash::Hash;
use std::sync::Arc;

use crate::error::{AppError, AppResult};

#[server]
pub async fn get_latest_blockhash() -> AppResult<Hash> {
    use solana_client::nonblocking::rpc_client::RpcClient;

    let solana_client =
        use_context::<Arc<RpcClient>>().ok_or(AppError::custom("Can't get context"))?;
    let blockhash = solana_client
        .get_latest_blockhash()
        .await
        .map_err(|e| AppError::custom(e.to_string()))?;

    Ok(blockhash)
}
