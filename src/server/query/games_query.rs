use crate::{
    accounts::GameMetadata,
    error::{AppError, AppResult},
    utils::GAME_KEY_ASSET_SEED,
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;
use std::{str::FromStr, sync::Arc};

#[derive(Serialize, Deserialize, Clone)]
pub struct FetchedGameMetadata {
    pub address: Pubkey,
    pub data: GameMetadata,
}

#[server]
pub async fn get_all_games() -> AppResult<Vec<FetchedGameMetadata>> {
    use crate::{accounts::fetch_all_game_metadata, utils::GAME_METADATA_DISCRIMINATOR};
    use solana_client::{
        rpc_client::RpcClient,
        rpc_config::RpcProgramAccountsConfig,
        rpc_filter::{Memcmp, RpcFilterType},
    };
    let solana_client =
        use_context::<Arc<RpcClient>>().ok_or(AppError::custom("Can't get context"))?;

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
            0,
            GAME_METADATA_DISCRIMINATOR.to_vec(),
        ))]),
        ..Default::default()
    };

    let metadata_addresses = solana_client
        .get_program_ui_accounts_with_config(&crate::REPLAYER_ID, config)
        .map_err(|_| AppError::custom("Can't fetch metadata addresses"))?
        .into_iter()
        .map(|(addr, _)| addr)
        .collect::<Vec<_>>();

    let metadatas = fetch_all_game_metadata(&solana_client, &metadata_addresses)?
        .iter()
        .map(|decoded_acc| FetchedGameMetadata {
            address: decoded_acc.address,
            data: decoded_acc.data.clone(),
        })
        .collect::<Vec<_>>();

    Ok(metadatas)
}

#[server]
pub async fn check_game_is_owned(
    player: String,
    developer: String,
    name: String,
) -> AppResult<bool> {
    use solana_client::rpc_client::RpcClient;

    let solana_client =
        use_context::<Arc<RpcClient>>().ok_or(AppError::custom("Can't get context"))?;
    let developer_key = Pubkey::from_str_const(&developer);
    let player_key = Pubkey::from_str_const(&player);

    let (game_nft_address, _) = Pubkey::find_program_address(
        &[
            GAME_KEY_ASSET_SEED,
            developer_key.as_ref(),
            name.as_bytes(),
            player_key.as_ref(),
        ],
        &crate::REPLAYER_ID,
    );

    Ok(solana_client.get_account(&game_nft_address).is_ok())
}
