use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar::rent,
};

use crate::state::*;

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AppInstruction {
    Configure(ConfigureArgs),
    CreateAndApproveCollection(CreateCollectionArgs),
    ApproveCollection,
    AddCollection(AddCollectionArgs),
    Mint,
    AddPromotion(AddPromotionArgs),
    CreateToken(CreateTokenArgs)
}

pub fn configure(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    args: ConfigureArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Configure(args).try_to_vec().unwrap(),
    })
}

pub fn create_collection(
    program_id: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    pda_creator_info: &Pubkey,
    collection_authority_record: &Pubkey,
    metadata_info: &Pubkey,
    edition_info: &Pubkey,
    metadata_program_info: &Pubkey,
    token_program: &Pubkey,
    args: CreateCollectionArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new(*pda_creator_info, false),
        AccountMeta::new(*collection_authority_record, false),
        AccountMeta::new(*metadata_info, false),
        AccountMeta::new(*edition_info, false),
        AccountMeta::new_readonly(*metadata_program_info, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::CreateAndApproveCollection(args).try_to_vec().unwrap(),
    })
}

pub fn approve_collection(
    program_id: &Pubkey,
    signer: &Pubkey,
    mint: &Pubkey,
    pda_creator_info: &Pubkey,
    collection_authority_record: &Pubkey,
    metadata_info: &Pubkey,
    metadata_program_info: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*mint, true),
        AccountMeta::new(*pda_creator_info, false),
        AccountMeta::new(*collection_authority_record, false),
        AccountMeta::new(*metadata_info, false),
        AccountMeta::new_readonly(*metadata_program_info, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::ApproveCollection.try_to_vec().unwrap(),
    })
}

pub fn add_collection(
    program_id: &Pubkey,
    signer: &Pubkey,
    config: &Pubkey,
    collection_mint: &Pubkey,
    pda_creator_info: &Pubkey,
    collection_info: &Pubkey,
    args: AddCollectionArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*config, false),
        AccountMeta::new(*collection_mint, false),
        AccountMeta::new(*pda_creator_info, false),
        AccountMeta::new(*collection_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::AddCollection(args).try_to_vec().unwrap(),
    })
}

pub fn add_promotion(
    program_id: &Pubkey,
    signer: &Pubkey,
    config: &Pubkey,
    collection_mint: &Pubkey,
    collection_info: &Pubkey,
    promotion_info: &Pubkey,
    args: AddPromotionArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*signer, true),
        AccountMeta::new(*config, false),
        AccountMeta::new(*collection_mint, false),
        AccountMeta::new(*collection_info, false),
        AccountMeta::new(*promotion_info, false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::AddPromotion(args).try_to_vec().unwrap(),
    })
}

pub fn mint(
    program_id: &Pubkey,
    siger: &Pubkey,
    pda_creator_info: &Pubkey,
    mint_info: &Pubkey,
    token_account: &Pubkey,
    metadata_info: &Pubkey,
    edition_info: &Pubkey,
    collection_mint: &Pubkey,
    collection_metadata: &Pubkey,
    collection_master_edition_account: &Pubkey,
    collection_authority_record: &Pubkey,
    promotion_info: &Pubkey,    
    collection_info: &Pubkey,
    charge_info: &Pubkey,
    user_info: &Pubkey,
    metadata_program_info: &Pubkey,
    token_program_info: &Pubkey,   
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*pda_creator_info, false),
        AccountMeta::new(*mint_info, true),
        AccountMeta::new(*token_account, false),
        AccountMeta::new(*metadata_info, false),
        AccountMeta::new(*edition_info, false),
        AccountMeta::new(*collection_mint, false),
        AccountMeta::new(*collection_metadata, false),
        AccountMeta::new(*collection_master_edition_account, false),
        AccountMeta::new(*collection_authority_record, false),
        AccountMeta::new(*promotion_info, false),        
        AccountMeta::new(*collection_info, false),
        AccountMeta::new(*charge_info, false),
        AccountMeta::new(*user_info, false),
        AccountMeta::new_readonly(*metadata_program_info, false),
        AccountMeta::new_readonly(*token_program_info, false),    
        AccountMeta::new_readonly(rent::id(), false),  
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Mint.try_to_vec().unwrap(),
    })
}

pub fn create_token(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    mint: &Pubkey,
    mint_vault: &Pubkey,
    mint_auth:  &Pubkey,
    metadata_key: &Pubkey,
    metadata_program: &Pubkey,
    args: CreateTokenArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*mint, false),
        AccountMeta::new(*mint_vault, false),
        AccountMeta::new(*mint_auth, false),
        AccountMeta::new(*metadata_key, false),
        AccountMeta::new_readonly(*metadata_program, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::CreateToken(args).try_to_vec().unwrap(),
    })
}