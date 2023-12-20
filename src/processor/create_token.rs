use borsh::BorshSerialize;
use mpl_token_metadata::{
    instructions::{
        CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs,
    },
    types::DataV2,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

use crate::{ferror, state::*, utils::*};

pub fn process_create_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateTokenArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let token_info = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let mint_vault = next_account_info(account_info_iter)?;
    let mint_auth = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;

    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(
        &metadata_program_info,
        &mpl_token_metadata::programs::MPL_TOKEN_METADATA_ID,
    )?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_token_info(&program_id, &mint.key, token_info)?;

    let bump = assert_token_info(program_id, &mint.key, token_info)?;
    let mint_vault_bump = assert_mint_vault(program_id, mint, mint_vault)?;
    let auth_bump = assert_mint_authority(program_id, mint, mint_auth)?;
    let authority_seed = [
        program_id.as_ref(),
        mint.key.as_ref(),
        "mint_auth".as_bytes(),
        &[auth_bump],
    ];
    //create mint
    let mut is_created = true;
    if token_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            token_info,
            rent_info,
            system_info,
            signer_info,
            TokenData::LEN,
            &[program_id.as_ref(), "config".as_bytes(), &[bump]],
        )?;
        msg!("spl token create mint");
        spl_token_create_mint(
            token_program_info,
            signer_info,
            mint,
            &mint_auth,
            &[],
            &[],
            rent_info,
            args.decimals,
        )?;
        //creat mint vault
        msg!("create mint vault");
        spl_token_create_account(
            &token_program_info,
            &signer_info,
            &mint,
            &mint_vault,
            &mint_auth,
            &[
                program_id.as_ref(),
                mint.key.as_ref(),
                "mint_vault".as_bytes(),
                &[mint_vault_bump],
            ],
            &authority_seed,
            &rent_info,
        )?;

        //create token metadata
        msg!("create metadata");
        let creators = vec![mpl_token_metadata::types::Creator {
            address: *signer_info.key,
            verified: true,
            share: 100,
        }];
        let cmv3 = CreateMetadataAccountV3 {
            metadata: *metadata_info.key,
            mint: *mint.key,
            mint_authority: *signer_info.key,
            payer: *signer_info.key,
            update_authority: (*signer_info.key, true),
            system_program: *system_info.key,
            rent: Some(*rent_info.key),
        };
        let data = DataV2 {
            name: args.name.clone(),
            symbol: args.symbol.clone(),
            uri: args.uri,
            seller_fee_basis_points: 0,
            creators: Some(creators),
            collection: None,
            uses: None,
        };
        let cmv3_args = CreateMetadataAccountV3InstructionArgs {
            data: data,
            is_mutable: true,
            collection_details: None,
        };
        invoke(
            &cmv3.instruction(cmv3_args),
            &[
                metadata_info.clone(),
                mint.clone(),
                signer_info.clone(),
                metadata_program_info.clone(),
                token_program_info.clone(),
                system_info.clone(),
                rent_info.clone(),
            ],
        )?;
        is_created = false;
    }

    let mut token_data = TokenData::from_account_info(token_info)?;
    if is_created {
        assert_eq_pubkey(signer_info, &token_data.creator)?;
        //update metadata todo
    }

    token_data.creator = signer_info.key.clone();
    token_data.decimals = args.decimals;
    token_data.name = args.name;
    token_data.symbol = args.symbol;
    token_data.mint = mint.key.clone();
    token_data.serialize(&mut &mut token_info.data.borrow_mut()[..])?;

    Ok(())
}
