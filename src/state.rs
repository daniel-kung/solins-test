use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use mpl_token_metadata::state::Creator;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigureArgs {
    /// Contract admin
    pub authority: Pubkey,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigureData {
    /// Contract admin
    pub authority: Pubkey,
}

impl ConfigureData {
    pub const LEN: usize = 32;

    pub fn from_account_info(a: &AccountInfo) -> Result<ConfigureData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct CreateCollectionArgs {
    ///seller fee
    pub fee: u16,
    /// nft name
    pub name: String,
    /// nft symbol
    pub symbol: String,
    /// default uri
    pub uri: String,
}


pub const MAX_CREATOR_LEN: usize = 32 + 1 + 1;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct AddCollectionArgs {
    pub max_supply : u64,
    pub collection_mint: Pubkey,
    pub admin: Pubkey,
    pub pda_creator: Pubkey,
    /// creators
    pub creators: Vec<Creator>,
    /// seller fee
    pub fee: u16,
    /// nft name
    pub name: String,
    /// nft symbol
    pub symbol: String,
    /// default uri
    pub uri: String,

}

pub type CollectionData = AddCollectionArgs;

impl CollectionData {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 34 * 4 + 1 + 4 + 32 + 10 + 200;

    pub fn from_account_info(a: &AccountInfo) -> Result<CollectionData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }

}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct AddPromotionArgs {
    pub sale_price: u64,
    pub public_start_ts: u64,
    pub collection: Pubkey,
    pub char_addr: Pubkey,
}

pub type PromotionData = AddPromotionArgs;

impl PromotionData {
    // pub const LEN: usize = 8 * 9 + 4 + 32 * 3 + 32 * 100 + 4;
    pub const LEN: usize = 8 + 8 + 32 * 2;

    pub fn from_account_info(a: &AccountInfo) -> Result<PromotionData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }

}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct UserData {
    pub minted: u16,
}

impl UserData {
    pub const LEN: usize = 2;

    pub fn from_account_info(a: &AccountInfo) -> Result<UserData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}