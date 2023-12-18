use mpl_token_metadata::instruction::{
    approve_collection_authority, create_master_edition_v3, create_metadata_accounts_v2,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    sysvar,
};

use crate::{state::*, utils::*};

pub fn process_create_collection(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateCollectionArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?;
    let collection_authority_record = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&metadata_program_info, &mpl_token_metadata::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_pda_creator(&program_id, mint_info, pda_creator_info)?;

    let creators = vec![mpl_token_metadata::state::Creator {
        address: *signer_info.key,
        verified: true,
        share: 100,
    }];

    msg!("Create metadata");
    invoke(
        &create_metadata_accounts_v2(
            *metadata_program_info.key,
            *metadata_info.key,
            *mint_info.key,
            *signer_info.key,
            *signer_info.key,
            *signer_info.key,
            args.name,
            args.symbol,
            args.uri,
            Some(creators),
            args.fee,
            true,
            true,
            None,
            None,
        ),
        &[
            metadata_info.clone(),
            mint_info.clone(),
            signer_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
        ],
    )?;

    msg!("Create Master Edition");
    invoke(
        &create_master_edition_v3(
            *metadata_program_info.key,
            *edition_info.key,
            *mint_info.key,
            *signer_info.key,
            *signer_info.key,
            *metadata_info.key,
            *signer_info.key,
            Some(0),
        ),
        &[
            edition_info.clone(),
            mint_info.clone(),
            signer_info.clone(),
            metadata_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
        ],
    )?;

    let approve_collection_accounts = vec![
        collection_authority_record.clone(),
        pda_creator_info.clone(),
        signer_info.clone(),
        metadata_info.clone(),
        metadata_program_info.clone(),
        mint_info.clone(),
        rent_info.clone(),
        system_info.clone(),
    ];

    msg!("approve collection");
    invoke(
        &approve_collection_authority(
            *metadata_program_info.key,
            *collection_authority_record.key,
            *pda_creator_info.key,
            *signer_info.key,
            *signer_info.key,
            *metadata_info.key,
            *mint_info.key,
        ),
        &approve_collection_accounts,
    )?;

    Ok(())
}
