use crate::{accounts::Developer, error::AppResult};
use leptos::prelude::*;

#[server]
pub async fn get_developer(developer: String) -> AppResult<Option<Developer>> {
    use std::sync::Arc;

    use crate::accounts::fetch_developer;
    use crate::constants::DEVELOPER_SEED;
    use crate::error::AppError;
    use solana_client::rpc_client::RpcClient;
    use solana_pubkey::Pubkey;

    let solana_client =
        use_context::<Arc<RpcClient>>().ok_or(AppError::custom("Can't get context"))?;

    let developer_pubkey = Pubkey::from_str_const(&developer);
    let (developer_account_pda, _) = Pubkey::find_program_address(
        &[DEVELOPER_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    match fetch_developer(&solana_client, &developer_account_pda) {
        Ok(decoded) => Ok(Some(decoded.data)),
        Err(_) => Ok(None),
    }
}

#[server]
pub async fn developer_exists(developer: String) -> AppResult<bool> {
    let developer = get_developer(developer).await?;

    match developer {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
