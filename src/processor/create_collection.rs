use mpl_token_metadata::{
    instructions::{
        ApproveCollectionAuthority, CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs,
        CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs,
    },
    types::DataV2,
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

    assert_eq_pubkey(
        &metadata_program_info,
        &mpl_token_metadata::programs::MPL_TOKEN_METADATA_ID,
    )?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    assert_pda_creator(&program_id, mint_info, pda_creator_info)?;

    let creators = vec![mpl_token_metadata::types::Creator {
        address: *signer_info.key,
        verified: true,
        share: 100,
    }];

    msg!("Create metadata");
    let cmv3 = CreateMetadataAccountV3 {
        metadata: *metadata_info.key,
        mint: *mint_info.key,
        mint_authority: *signer_info.key,
        payer: *signer_info.key,
        update_authority: (*signer_info.key, true),
        system_program: *system_info.key,
        rent: Some(*rent_info.key),
    };
    let data = DataV2 {
        name: args.name,
        symbol: args.symbol,
        uri: args.uri,
        seller_fee_basis_points: args.fee,
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
            mint_info.clone(),
            signer_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
        ],
    )?;

    msg!("Create Master Edition");
    let cmev3 = CreateMasterEditionV3 {
        edition: *edition_info.key,
        mint: *mint_info.key,
        update_authority: *signer_info.key,
        mint_authority: *signer_info.key,
        payer: *signer_info.key,
        metadata: *metadata_info.key,
        token_program: *token_program_info.key,
        system_program: *system_info.key,
        rent: Some(*rent_info.key),
    };
    let cmev3_args = CreateMasterEditionV3InstructionArgs {
        max_supply: Some(0),
    };
    invoke(
        // &create_master_edition_v3(
        //     *metadata_program_info.key,
        //     *edition_info.key,
        //     *mint_info.key,
        //     *signer_info.key,
        //     *signer_info.key,
        //     *metadata_info.key,
        //     *signer_info.key,
        //     Some(0),
        // ),
        &cmev3.instruction(cmev3_args),
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
    let aca = ApproveCollectionAuthority {
        collection_authority_record: *collection_authority_record.key,
        new_collection_authority: *pda_creator_info.key,
        update_authority: *signer_info.key,
        payer: *signer_info.key,
        metadata: *metadata_info.key,
        mint: *mint_info.key,
        system_program: *system_info.key,
        rent: Some(*rent_info.key),
    };
    invoke(&aca.instruction(), &approve_collection_accounts)?;

    Ok(())
}
