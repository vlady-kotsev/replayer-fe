use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_withdraw_developer_fee_tx(
    developer: String,
    amount: u64,
) -> AppResult<Transaction> {
    use crate::generated::instructions::WithdrawDeveloperFeeBuilder;
    use crate::{
        constants::{DEVELOPER_SEED, DEVELOPER_TREASURY_SEED},
        server::get_latest_blockhash,
    };
    use solana_pubkey::Pubkey;

    let developer_pubkey = Pubkey::from_str_const(&developer);

    let (developer_account_pda, _) = Pubkey::find_program_address(
        &[DEVELOPER_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );
    let (developer_treasury_pda, _) = Pubkey::find_program_address(
        &[DEVELOPER_TREASURY_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    let ix = WithdrawDeveloperFeeBuilder::new()
        .developer(developer_pubkey)
        .developer_account(developer_account_pda)
        .developer_treasury(developer_treasury_pda)
        .amount(amount)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&developer_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
