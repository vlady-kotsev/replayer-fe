use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_buy_game_tx(
    player: String,
    developer_arg: String,
    game_name: String,
) -> AppResult<Transaction> {
    use crate::generated::instructions::BuyGameBuilder;
    use crate::{
        utils::{
            DEVELOPER_COLLECTION_SEED, DEVELOPER_SEED, DEVELOPER_TREASURY_SEED,
            GAME_DATA_SEED, GAME_KEY_ASSET_SEED, GAME_METADATA_SEED, GLOBAL_CONFIG_SEED,
            GLOBAL_TREASURY_SEED,
        },
        server::get_latest_blockhash,
    };
    use solana_pubkey::Pubkey;

    let player_pubkey = Pubkey::from_str_const(&player);
    let developer_pubkey = Pubkey::from_str_const(&developer_arg);

    let (game_metadata_pda, _) = Pubkey::find_program_address(
        &[GAME_METADATA_SEED, developer_pubkey.as_ref(), game_name.as_bytes()],
        &crate::REPLAYER_ID,
    );
    let (game_data_pda, _) = Pubkey::find_program_address(
        &[GAME_DATA_SEED, developer_pubkey.as_ref(), game_name.as_bytes()],
        &crate::REPLAYER_ID,
    );
    let (developer_account_pda, _) = Pubkey::find_program_address(
        &[DEVELOPER_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );
    let (collection_pda, _) = Pubkey::find_program_address(
        &[DEVELOPER_COLLECTION_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );
    let (asset_pda, _) = Pubkey::find_program_address(
        &[
            GAME_KEY_ASSET_SEED,
            developer_pubkey.as_ref(),
            game_name.as_bytes(),
            player_pubkey.as_ref(),
        ],
        &crate::REPLAYER_ID,
    );
    let (global_config_pda, _) =
        Pubkey::find_program_address(&[GLOBAL_CONFIG_SEED], &crate::REPLAYER_ID);
    let (global_treasury_pda, _) =
        Pubkey::find_program_address(&[GLOBAL_TREASURY_SEED], &crate::REPLAYER_ID);
    let (developer_treasury_pda, _) = Pubkey::find_program_address(
        &[DEVELOPER_TREASURY_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    let ix = BuyGameBuilder::new()
        .player(player_pubkey)
        .game_metadata(game_metadata_pda)
        .game_data(game_data_pda)
        .developer_account(developer_account_pda)
        .collection(collection_pda)
        .asset(asset_pda)
        .global_config(global_config_pda)
        .global_treasury(global_treasury_pda)
        .developer_treasury(developer_treasury_pda)
        .game_name(game_name)
        .developer(developer_pubkey)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&player_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
