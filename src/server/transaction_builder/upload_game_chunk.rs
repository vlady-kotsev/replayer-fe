use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_upload_game_chunk_tx(
    developer: String,
    game_name: String,
    data_chunk: Vec<u8>,
) -> AppResult<Transaction> {
    use crate::generated::instructions::UploadGameChunkBuilder;
    use crate::{
        server::get_latest_blockhash,
        utils::{GAME_DATA_SEED, GAME_METADATA_SEED},
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

    let ix = UploadGameChunkBuilder::new()
        .developer(developer_pubkey)
        .game_data(game_data_pda)
        .game_metadata(game_metadata_pda)
        .data_chunk(data_chunk)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&developer_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
