use crate::{
    accounts::GameMetadata,
    error::{AppError, AppResult},
    utils::{GAME_DATA_SEED, GAME_KEY_ASSET_SEED},
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;
use std::sync::Arc;

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
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig, UiAccountEncoding},
        rpc_filter::{Memcmp, RpcFilterType},
    };
    let solana_client =
        use_context::<Arc<RpcClient>>().ok_or(AppError::custom("Can't get context"))?;

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
            0,
            GAME_METADATA_DISCRIMINATOR.to_vec(),
        ))]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            ..Default::default()
        },
        ..Default::default()
    };

    let metadata_addresses = solana_client
        .get_program_ui_accounts_with_config(&crate::REPLAYER_ID, config)
        .map_err(|e| AppError::custom(format!("Can't fetch metadata addresses: {e}")))?
        .into_iter()
        .map(|(addr, _)| addr)
        .collect::<Vec<_>>();

    let metadatas = fetch_all_game_metadata(&solana_client, &metadata_addresses)?
        .iter()
        .filter_map(|decoded_acc| {
            decoded_acc
                .data
                .is_finalized
                .then_some(FetchedGameMetadata {
                    address: decoded_acc.address,
                    data: decoded_acc.data.clone(),
                })
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

    let (game_nft_pda, _) = Pubkey::find_program_address(
        &[
            GAME_KEY_ASSET_SEED,
            developer_key.as_ref(),
            name.as_bytes(),
            player_key.as_ref(),
        ],
        &crate::REPLAYER_ID,
    );

    Ok(solana_client.get_account(&game_nft_pda).is_ok())
}

#[server]
pub async fn get_owned_games(player: String) -> AppResult<Vec<FetchedGameMetadata>> {
    use solana_client::rpc_client::RpcClient;

    let solana_client =
        use_context::<Arc<RpcClient>>().ok_or(AppError::custom("Can't get context"))?;
    let player_key = Pubkey::from_str_const(&player);

    let all_games = get_all_games().await?;

    let owned = all_games
        .into_iter()
        .filter(|game| {
            let (game_nft_address, _) = Pubkey::find_program_address(
                &[
                    GAME_KEY_ASSET_SEED,
                    game.data.developer.as_ref(),
                    game.data.game_name.as_bytes(),
                    player_key.as_ref(),
                ],
                &crate::REPLAYER_ID,
            );
            solana_client.get_account(&game_nft_address).is_ok()
        })
        .collect();

    Ok(owned)
}

#[server]
pub async fn get_game_data(developer: String, game_name: String) -> AppResult<Vec<u8>> {
    use crate::accounts::fetch_game_data;
    use solana_client::rpc_client::RpcClient;
    let solana_client =
        use_context::<Arc<RpcClient>>().ok_or(AppError::custom("Can't get context"))?;
    let developer_key = Pubkey::from_str_const(&developer);

    let (game_data_pda, _) = Pubkey::find_program_address(
        &[GAME_DATA_SEED, developer_key.as_ref(), game_name.as_bytes()],
        &crate::REPLAYER_ID,
    );

    let decoded_acc = fetch_game_data(&solana_client, &game_data_pda)?;
    let data_len = decoded_acc.data.length as usize;
    let rom_data = decoded_acc.data.data[..data_len].to_vec();

    Ok(rom_data)
}
