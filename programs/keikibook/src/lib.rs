mod instructions;
mod processor;
mod error;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    log::sol_log_compute_units,
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub txt: String,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("program entrypoint");

    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Start instruction decode");
    let message = GreetingAccount::try_from_slice(instruction_data).map_err(|err| {
        msg!("Receiving message as string utf8 failed, {:?}", err);
        ProgramError::InvalidInstructionData
    })?;
    msg!("Greeting passed to program is {:?}", message);

    let data = &mut &mut account.data.borrow_mut();
    msg!("Start save instruction into data");
    data[..instruction_data.len()].copy_from_slice(&instruction_data);

    sol_log_compute_units();
    msg!("Was sent message {}!", message.txt);

    Ok(())
}