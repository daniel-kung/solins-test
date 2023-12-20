use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::*;

pub mod configure;
pub use configure::*;

pub mod mint;
pub use mint::*;

pub mod create_collection;
pub use create_collection::*;

pub mod approve_collection;
pub use approve_collection::*;

pub mod add_collection;
pub use add_collection::*;

pub mod add_promotion;
pub use add_promotion::*;

pub mod create_token;
pub use create_token::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = AppInstruction::try_from_slice(input)?;
    match instruction {
        AppInstruction::Configure(args) => {
            msg!("Instruction: Configure");
            process_configure(program_id, accounts, args)
        }
        AppInstruction::Mint => {
            msg!("Instruction: Mint");
            process_mint(program_id, accounts)
        }
        AppInstruction::CreateAndApproveCollection(args) => {
            msg!("Instruction: CreateAndApproveCollection");
            process_create_collection(program_id, accounts, args)
        }
        AppInstruction::ApproveCollection => {
            msg!("Instruction: ApproveCollection");
            process_approve_collection(program_id, accounts)
        }
        AppInstruction::AddCollection(args) => {
            msg!("Instruction: AddCollection");
            process_add_collection(program_id, accounts, args)
        }
        AppInstruction::AddPromotion(args) => {
            msg!("Instruction: AddPromotion");
            process_add_promotion(program_id, accounts, args)
        }
        AppInstruction::CreateToken(args) => {
            msg!("Instruction: CreateToken");
            process_create_token(program_id, accounts, args)
        }
    }
}
