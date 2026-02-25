use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_withdraw_platform_fee_tx(
    withdrawer: String,
    receiver: String,
    amount: u64,
) -> AppResult<Transaction> {
    use crate::generated::instructions::WithdrawPlatformFeeBuilder;
    use crate::{
        constants::{ADMIN_SEED, GLOBAL_CONFIG_SEED, GLOBAL_TREASURY_SEED},
        server::get_latest_blockhash,
    };
    use solana_pubkey::Pubkey;

    let withdrawer_pubkey = Pubkey::from_str_const(&withdrawer);
    let receiver_pubkey = Pubkey::from_str_const(&receiver);

    let (admin_account_pda, _) = Pubkey::find_program_address(
        &[ADMIN_SEED, withdrawer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );
    let (global_config_pda, _) =
        Pubkey::find_program_address(&[GLOBAL_CONFIG_SEED], &crate::REPLAYER_ID);
    let (global_treasury_pda, _) =
        Pubkey::find_program_address(&[GLOBAL_TREASURY_SEED], &crate::REPLAYER_ID);

    let ix = WithdrawPlatformFeeBuilder::new()
        .withdrawer(withdrawer_pubkey)
        .admin_account(admin_account_pda)
        .global_config(global_config_pda)
        .global_treasury(global_treasury_pda)
        .receiver(receiver_pubkey)
        .amount(amount)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&withdrawer_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
