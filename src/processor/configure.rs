use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    sysvar,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{ferror, state::*, utils::*};

pub fn process_configure(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ConfigureArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    let bump = assert_config(&program_id, &config_info)?;

    let mut is_created = true;
    if config_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            config_info,
            rent_info,
            system_info,
            signer_info,
            ConfigureData::LEN,
            &[
                program_id.as_ref(),
                "config".as_bytes(),
                &[bump],
            ],
        )?;
        is_created = false;
    }

    let mut config_data = ConfigureData::from_account_info(config_info)?;

    if is_created {
        if config_data.authority != *signer_info.key {
            return ferror!("invalid authority");
        }
        assert_owned_by(config_info, &program_id)?;
    }


    config_data.authority = args.authority;
    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;

    Ok(())
}
