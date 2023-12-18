use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};
use std::io::Error;

use crate::error::AppError;

pub fn now_timestamp() -> u64 {
    Clock::get().unwrap().unix_timestamp as u64
}

pub fn assert_eq_pubkey(account_info: &AccountInfo, account: &Pubkey) -> ProgramResult {
    if account_info.key != account {
        Err(AppError::InvalidEqPubkey.into())
    } else {
        Ok(())
    }
}

pub fn assert_eq_pubkey_0(account_info: &AccountInfo, account: &Pubkey) -> ProgramResult {
    if account_info.key != account {
        Err(AppError::InvalidEqPubkey0.into())
    } else {
        Ok(())
    }
}

pub fn assert_eq_pubkey_1(account_info: &AccountInfo, account: &Pubkey) -> ProgramResult {
    if account_info.key != account {
        Err(AppError::InvalidEqPubkey1.into())
    } else {
        Ok(())
    }
}

pub fn assert_eq_pubkey_2(account_info: &AccountInfo, account: &Pubkey) -> ProgramResult {
    if account_info.key != account {
        Err(AppError::InvalidEqPubkey2.into())
    } else {
        Ok(())
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(AppError::InvalidOwner.into())
    } else {
        Ok(())
    }
}

pub fn assert_derivation(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
) -> Result<u8, ProgramError> {
    let (key, bump) = Pubkey::find_program_address(&path, program_id);
    if key != *account.key {
        return Err(AppError::InvalidDerivedKey.into());
    }
    Ok(bump)
}

pub fn assert_config(program_id: &Pubkey, account: &AccountInfo) -> Result<u8, ProgramError> {
    let path = &[program_id.as_ref(), "config".as_bytes()];
    assert_derivation(&program_id, &account, path)
}

pub fn assert_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

pub fn assert_pda_creator(
    program_id: &Pubkey,
    collection_mint: &AccountInfo,
    pda_creator_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        program_id.as_ref(),
        collection_mint.key.as_ref(),
        "pda_creator".as_bytes(),
    ];
    assert_derivation(&program_id, &pda_creator_info, path)
}

pub fn assert_skc_user_info(
    program_id: &Pubkey,
    skc_info: &AccountInfo,
    signer_info: &AccountInfo,
    user_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        program_id.as_ref(),
        skc_info.key.as_ref(),
        signer_info.key.as_ref(),
    ];
    assert_derivation(&program_id, &user_info, path)
}

pub fn assert_collection(
    program_id: &Pubkey,
    collection_mint: &AccountInfo,
    collection_info: &AccountInfo,
) -> Result<u8, ProgramError> {
    let path = &[
        program_id.as_ref(),
        collection_mint.key.as_ref(),
        "collection".as_bytes(),
    ];
    assert_derivation(&program_id, &collection_info, path)
}

pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer<'a>(
    token_program: AccountInfo<'a>,
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    amount: u64,
    signer_seeds: &[&[u8]],
) -> Result<(), ProgramError> {
    invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
        &[&signer_seeds],
    )
}

#[inline(always)]
pub fn create_or_allocate_account_raw<'a>(
    program_id: Pubkey,
    new_account_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> Result<(), ProgramError> {
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let required_lamports = rent
        .minimum_balance(size)
        .max(1)
        .saturating_sub(new_account_info.lamports());

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        invoke(
            &system_instruction::transfer(&payer_info.key, new_account_info.key, required_lamports),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    msg!("Allocate space for the account");
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        &[new_account_info.clone(), system_program_info.clone()],
        &[&signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, &program_id),
        &[new_account_info.clone(), system_program_info.clone()],
        &[&signer_seeds],
    )?;
    msg!("Completed assignation!");

    Ok(())
}

#[inline(always)]
pub fn spl_token_create_account<'a>(
    token_program: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    mint_info: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    create_account_seeds: &[&[u8]], // when account is not a pda, is null
    initialize_account_seeds: &[&[u8]], // when account is not a pda, is null
    rent_info: &AccountInfo<'a>,
) -> ProgramResult {
    let size = spl_token::state::Account::LEN;
    let rent = &Rent::from_account_info(&rent_info)?;
    let required_lamports = rent.minimum_balance(size);

    msg!("spl_token_create_account create");
    invoke_signed(
        &system_instruction::create_account(
            payer_info.key,
            new_account.key,
            required_lamports,
            size as u64,
            token_program.key,
        ),
        &[payer_info.clone(), new_account.clone()],
        &[create_account_seeds],
    )?;

    msg!("spl_token_create_account initialize");
    invoke_signed(
        &spl_token::instruction::initialize_account(
            token_program.key,
            new_account.key,
            mint_info.key,
            authority.key,
        )?,
        &[
            token_program.clone(),
            new_account.clone(),
            mint_info.clone(),
            authority.clone(),
            rent_info.clone(),
        ],
        &[initialize_account_seeds],
    )?;
    msg!("spl_token_create_account success");

    Ok(())
}

pub fn try_from_slice_unchecked<T: BorshDeserialize>(data: &[u8]) -> Result<T, Error> {
    let mut data_mut = data;
    let result = T::deserialize(&mut data_mut)?;
    Ok(result)
}

pub fn spl_token_transfer_invoke<'a>(
    token_program: AccountInfo<'a>,
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    amount: u64,
) -> Result<(), ProgramError> {
    invoke(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
    )
}

pub fn merkle_proof_verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash = solana_program::keccak::hashv(&[&computed_hash, &proof_element]).0;
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash = solana_program::keccak::hashv(&[&proof_element, &computed_hash]).0;
        }
    }
    // Check if the computed hash (root) is equal to the provided root
    computed_hash == root
}