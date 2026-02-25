use crate::error::AppResult;
use leptos::prelude::*;
use solana_transaction::{Message, Transaction};

#[server]
pub async fn build_create_developer_tx(
    developer: String,
    company_name: String,
    collection_uri: String,
) -> AppResult<Transaction> {
    use crate::generated::instructions::CreateDeveloperBuilder;
    use crate::{
        constants::{BLACKLISTED_SEED, DEVELOPER_COLLECTION_SEED, DEVELOPER_SEED, DEVELOPER_TREASURY_SEED},
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
    let (developer_collection_pda, _) = Pubkey::find_program_address(
        &[DEVELOPER_COLLECTION_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );
    let (blacklist_account_pda, _) = Pubkey::find_program_address(
        &[BLACKLISTED_SEED, developer_pubkey.as_ref()],
        &crate::REPLAYER_ID,
    );

    let ix = CreateDeveloperBuilder::new()
        .developer(developer_pubkey)
        .developer_account(developer_account_pda)
        .developer_treasury(developer_treasury_pda)
        .developer_collection(developer_collection_pda)
        .blacklist_account(blacklist_account_pda)
        .company_name(company_name)
        .collection_uri(collection_uri)
        .instruction();

    let blockhash = get_latest_blockhash().await?;
    let message = Message::new_with_blockhash(&[ix], Some(&developer_pubkey), &blockhash);
    let tx = Transaction::new_unsigned(message);

    Ok(tx)
}
