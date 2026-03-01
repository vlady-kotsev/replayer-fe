use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_finalize_game_upload_tx(
    developer: String,
    game_name: String,
) -> AppResult<Transaction> {
    use crate::generated::instructions::FinalizeGameUploadBuilder;
    use crate::{
        server::get_latest_blockhash,
        utils::{DEVELOPER_SEED, GAME_DATA_SEED, GAME_METADATA_SEED},
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
    let (developer_account_pda, _) = Pubkey::find_program_address(
        &[DEVELOPER_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    let ix = FinalizeGameUploadBuilder::new()
        .developer(developer_pubkey)
        .game_data(game_data_pda)
        .game_metadata(game_metadata_pda)
        .developer_account(developer_account_pda)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&developer_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
