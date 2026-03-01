use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_init_global_config_tx(
    initializer: String,
    platform_fee: u64,
) -> AppResult<Transaction> {
    use crate::generated::instructions::InitGlobalConfigBuilder;
    use crate::{
        server::get_latest_blockhash,
        utils::{ADMIN_SEED, GLOBAL_CONFIG_SEED, GLOBAL_TREASURY_SEED},
    };
    use solana_pubkey::Pubkey;

    let initializer_pubkey = Pubkey::from_str_const(&initializer);

    let (global_config_pda, _) =
        Pubkey::find_program_address(&[GLOBAL_CONFIG_SEED], &crate::REPLAYER_ID);
    let (admin_pda, _) = Pubkey::find_program_address(
        &[ADMIN_SEED, initializer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );
    let (global_treasury_pda, _) =
        Pubkey::find_program_address(&[GLOBAL_TREASURY_SEED], &crate::REPLAYER_ID);

    let program_data = Pubkey::find_program_address(
        &[crate::REPLAYER_ID.as_ref()],
        &solana_pubkey::pubkey!("BPFLoaderUpgradeab1e11111111111111111111111"),
    )
    .0;

    let ix = InitGlobalConfigBuilder::new()
        .initializer(initializer_pubkey)
        .global_config(global_config_pda)
        .admin(admin_pda)
        .global_treasury(global_treasury_pda)
        .program_data(program_data)
        .platform_fee(platform_fee)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&initializer_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
