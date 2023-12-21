use crate::{ferror, state::*, utils::*};
use borsh::BorshSerialize;
use mpl_token_metadata::instructions::{
    CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3,
    CreateMetadataAccountV3InstructionArgs, VerifyCollection,
};
use mpl_token_metadata::types::{Collection, Creator, DataV2};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, sysvar,
};

pub fn process_mint(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    let mint_info = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let metadata_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let collection_mint = next_account_info(account_info_iter)?;
    let collection_metadata = next_account_info(account_info_iter)?;
    let collection_master_edition_account = next_account_info(account_info_iter)?;
    let collection_authority_record = next_account_info(account_info_iter)?;
    let promotion_info = next_account_info(account_info_iter)?;
    let collection_info = next_account_info(account_info_iter)?;
    let charge_info = next_account_info(account_info_iter)?;
    let user_info = next_account_info(account_info_iter)?;
    let metadata_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_signer(&signer_info)?;
    assert_eq_pubkey(&token_program_info, &spl_token::id())?;
    assert_eq_pubkey_0(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey_1(&system_info, &solana_program::system_program::id())?;

    let pro_data = PromotionData::from_account_info(promotion_info)?;
    let mut collection_data = CollectionData::from_account_info(collection_info)?;
    assert_eq_pubkey_2(&charge_info, &pro_data.char_addr)?;

    let user_bump = assert_user_info(program_id, &signer_info.key, user_info)?;
    let user_seeds = [
        program_id.as_ref(),
        signer_info.key.as_ref(),
        "user_info".as_bytes(),
        &[user_bump],
    ];

    if user_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            user_info,
            rent_info,
            system_info,
            signer_info,
            UserData::LEN,
            &user_seeds
        )?;
    }

    let name = collection_data.name.clone();
    let symbol = collection_data.symbol.clone();
    let uri = collection_data.uri.clone();
    let fee = collection_data.fee.clone();
    let now_ts = now_timestamp();
    //check sale state
    if pro_data.public_start_ts > now_ts {
        return ferror!("sale not open");
    }

    if collection_data.max_supply == 10000 {
        return ferror!("sold out");
    }

    let pda_bump = assert_pda_creator(&program_id, collection_mint, pda_creator_info)?;
    let pda_seed = [
        program_id.as_ref(),
        collection_mint.key.as_ref(),
        "pda_creator".as_bytes(),
        &[pda_bump],
    ];

    let price = pro_data.sale_price;
    invoke(
        &system_instruction::transfer(&signer_info.key, &pro_data.char_addr, price),
        &[
            signer_info.clone(),
            charge_info.clone(),
            system_info.clone(),
        ],
    )?;

    //deal creators
    let mut creators = vec![Creator {
        address: *pda_creator_info.key,
        verified: true,
        share: 0,
    }];
    for creator in collection_data.creators.iter() {
        creators.push(creator.clone());
    }

    //create metadata
    let cmv3 = CreateMetadataAccountV3 {
        metadata: *metadata_info.key,
        mint: *mint_info.key,
        mint_authority: *signer_info.key,
        payer: *signer_info.key,
        update_authority: (*pda_creator_info.key, true),
        system_program: *system_info.key,
        rent: Some(*rent_info.key),
    };
    let data = DataV2 {
        name: name,
        symbol: symbol,
        uri: uri,
        seller_fee_basis_points: fee,
        creators: Some(creators),
        collection: Some(Collection {
            verified: false,
            key: *collection_mint.key,
        }),
        uses: None,
    };
    let cmv3_args = CreateMetadataAccountV3InstructionArgs {
        data: data,
        is_mutable: true,
        collection_details: None,
    };
    invoke_signed(
        &cmv3.instruction(cmv3_args),
        // &create_metadata_accounts_v2(
        //     *metadata_program_info.key,
        //     *metadata_info.key,
        //     *mint_info.key,
        //     *signer_info.key,
        //     *signer_info.key,
        //     *pda_creator_info.key,
        //     name,
        //     collection_data.symbol.clone(),
        //     uri,
        //     Some(creators),
        //     collection_data.fee,
        //     true,
        //     true,
        //     Some(Collection {
        //         verified: false,
        //         key: *collection_mint.key,
        //     }),
        //     None,
        // ),
        &[
            metadata_info.clone(),
            mint_info.clone(),
            signer_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
            pda_creator_info.clone(),
            collection_mint.clone(),
        ],
        &[&pda_seed],
    )?;

    //create edition
    msg!("Create Master Edition");
    let cmev3 = CreateMasterEditionV3 {
        edition: *edition_info.key,
        mint: *mint_info.key,
        update_authority: *pda_creator_info.key,
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

    invoke_signed(
        &cmev3.instruction(cmev3_args),
        // &create_master_edition_v3(
        //     *metadata_program_info.key,
        //     *edition_info.key,
        //     *mint_info.key,
        //     *pda_creator_info.key,
        //     *signer_info.key,
        //     *metadata_info.key,
        //     *signer_info.key,
        //     Some(0),
        // ),
        &[
            edition_info.clone(),
            mint_info.clone(),
            signer_info.clone(),
            metadata_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
            pda_creator_info.clone(),
        ],
        &[&pda_seed],
    )?;

    msg!("verify collection");
    let vc = VerifyCollection {
        metadata: *metadata_info.key,
        collection_authority: *pda_creator_info.key,
        payer: *signer_info.key,
        collection_mint: *collection_mint.key,
        collection: *collection_metadata.key,
        collection_master_edition_account: *collection_master_edition_account.key,
        collection_authority_record: Some(*collection_authority_record.key),
    };
    invoke_signed(
        &vc.instruction(),
        // &verify_collection(
        //     *metadata_program_info.key,
        //     *metadata_info.key,
        //     *pda_creator_info.key,
        //     *signer_info.key,
        //     *collection_mint.key,
        //     *collection_metadata.key,
        //     *collection_master_edition_account.key,
        //     Some(*collection_authority_record.key),
        // ),
        &[
            collection_mint.clone(),
            signer_info.clone(),
            metadata_info.clone(),
            metadata_program_info.clone(),
            token_program_info.clone(),
            system_info.clone(),
            rent_info.clone(),
            collection_metadata.clone(),
            collection_master_edition_account.clone(),
            collection_authority_record.clone(),
            pda_creator_info.clone(),
        ],
        &[&pda_seed],
    )?;
    let mut user_data = UserData::from_account_info(user_info)?;
    user_data.shots += 1;
    // if now_ts > collection_data.ts {
    //     collection_data.max_supply += 1;
    //     collection_data.ts = now_ts;
    //     user_data.minted += 1;
    // } else {
        spl_token_burn(
            token_program_info,
            mint_info,
            token_account,
            signer_info,
            &[],
            rent_info,
            1,
        )?;
    // }

    user_data.serialize(&mut *user_info.try_borrow_mut_data()?)?;
    collection_data.serialize(&mut *collection_info.try_borrow_mut_data()?)?;
    Ok(())
}
