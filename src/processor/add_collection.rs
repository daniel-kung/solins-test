use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

use crate::{ferror, state::*, utils::*};

pub fn process_add_collection(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddCollectionArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let collection_mint = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?;
    let collection_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_pda_creator(&program_id, collection_mint, pda_creator_info)?;
    assert_collection(&program_id, collection_mint, collection_info)?;

    //check authority
    let config_data = ConfigureData::from_account_info(config_info)?;
    if config_data.authority != *signer_info.key {
        return ferror!("invalid authority");
    }
    assert_owned_by(config_info, &program_id)?;

    let bump = assert_collection(&program_id, &collection_mint, &collection_info)?;

    if collection_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            collection_info,
            rent_info,
            system_info,
            signer_info,
            CollectionData::LEN,
            &[program_id.as_ref(), collection_mint.key.as_ref(), "collection".as_bytes(), &[bump]],
        )?;
    }

    let mut collection_data = CollectionData::from_account_info(collection_info)?;
    collection_data.collection_mint = collection_mint.key.clone();
    collection_data.admin = args.admin;
    collection_data.pda_creator = pda_creator_info.key.clone();
    collection_data.creators = args.creators;
    collection_data.fee = args.fee;
    collection_data.name = args.name;
    collection_data.symbol = args.symbol;
    collection_data.uri = args.uri;

    collection_data.serialize(&mut &mut collection_info.data.borrow_mut()[..])?;

    Ok(())
}
