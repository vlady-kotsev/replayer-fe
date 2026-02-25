use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_allocate_game_account_tx(
    developer: String,
    game_name: String,
    game_uri: String,
    game_price: u64,
    game_hash: Vec<u8>,
    max_supply: u64,
    game_data_length: u64,
) -> AppResult<Transaction> {
    use crate::generated::instructions::AllocateGameAccountBuilder;
    use crate::{
        constants::{BLACKLISTED_SEED, GAME_DATA_SEED, GAME_METADATA_SEED},
        server::get_latest_blockhash,
    };
    use solana_pubkey::Pubkey;

    let developer_pubkey = Pubkey::from_str_const(&developer);

    let (game_data_pda, _) = Pubkey::find_program_address(
        &[
            GAME_DATA_SEED,
            developer_pubkey.as_ref(),
            game_name.as_bytes(),
        ],
        &crate::REPLAYER_ID,
    );
    let (game_metadata_pda, _) = Pubkey::find_program_address(
        &[
            GAME_METADATA_SEED,
            developer_pubkey.as_ref(),
            game_name.as_bytes(),
        ],
        &crate::REPLAYER_ID,
    );
    let (blacklisted_pda, _) = Pubkey::find_program_address(
        &[BLACKLISTED_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    let game_hash_array: [u8; 32] = game_hash
        .try_into()
        .map_err(|_| crate::error::AppError::custom("game_hash must be 32 bytes"))?;

    let ix = AllocateGameAccountBuilder::new()
        .developer(developer_pubkey)
        .game_data(game_data_pda)
        .game_metadata(game_metadata_pda)
        .blacklisted(blacklisted_pda)
        .game_name(game_name)
        .game_uri(game_uri)
        .game_price(game_price)
        .game_hash(game_hash_array)
        .max_supply(max_supply)
        .game_data_length(game_data_length)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&developer_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
