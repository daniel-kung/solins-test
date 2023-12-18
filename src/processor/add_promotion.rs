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

pub fn process_add_promotion(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddPromotionArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let collection_mint = next_account_info(account_info_iter)?;
    let collection_info = next_account_info(account_info_iter)?;
    let promotion_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_collection(&program_id, &collection_mint, &collection_info)?;

    //check authority
    let config_data = ConfigureData::from_account_info(config_info)?;
    if config_data.authority != *signer_info.key {
        return ferror!("invalid authority");
    }
    assert_owned_by(config_info, &program_id)?;

    let collection_data = CollectionData::from_account_info(collection_info)?;
    let path = &[
        program_id.as_ref(),
        collection_info.key.as_ref(),
    ];
    let bump = assert_derivation(&program_id, &promotion_info, path)?;
    let bump_seed = &[
        program_id.as_ref(),
        collection_info.key.as_ref(),
        &[bump],
    ];
    if promotion_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            promotion_info,
            rent_info,
            system_info,
            signer_info,
            PromotionData::LEN,
            bump_seed,
        )?;
    }

    let mut promotion_data = PromotionData::from_account_info(promotion_info)?;


    promotion_data.sale_price = args.sale_price;
    promotion_data.public_start_ts = args.public_start_ts;
    promotion_data.char_addr = args.char_addr;
    promotion_data.collection = collection_data.collection_mint;
    
    promotion_data.serialize(&mut &mut promotion_info.data.borrow_mut()[..])?;

    Ok(())
}
