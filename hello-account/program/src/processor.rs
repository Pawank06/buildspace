use borsh::{BorshSerialize, BorshDeserialize}
use solana_program:: {
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
}

use crate::{
    error::HelloError,
    instruction::HelloInstruction,
    state::{Greeting, MAX_MESSAGE_LEN},
}

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = HelloInstruction::unpack(instruction_data)?;

        match instruction {
            HelloInstruction::Initialize {owner} => {
                msg!("Instruction: Initialize");
                Self::process_initialize(program_id, accounts, owner)
            }

            HelloInstruction::UpdateMessage {message} => {
                msg!("Instruction: UpdateMessage");
                Self::process_update_message(accounts, message)
            }

            HelloInstruction::IncrementOnly => {
                msg!("Instruction: IncrementOnly");
                Self::process_initialize_only(accounts)
            }

            HelloInstruction::Close => {
                msg!("Instruction: Close");
                Self::process_close(accounts)
            }
        }
    }

    fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        owner: Pubkey,
    ) -> ProgramResult{
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let greeting_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        if !payer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let space = Greeting::space();
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(space);

        invoke(
            &system_instruction::crate_account(
                payer.key,
                greeting_account.key,
                lamports,
                space as u64,
                program_id,
            ),
            &[payer.clone(), greeting_account.clone(), system_program.clone()],
        )?;

        let greeting = Greeting {
            owner,
            count: 0,
            message: String::from("Hello, Solana!"),
        };

        greeting.serialize(&mut &mut greeting_account.data.borrow_mut()[...])?;

        msg!("Greeting account created for owner: {}", owner);
        Ok(())
    }

    fn process_update_message(accounts: &[AccountInfo], message: String) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let greeting_account = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut greeting = Greeting::try_from_slice(&greeting_account.data.borrow())?;

        if greeting.owner != *owner.key {
            return Err(HelloError::Unauthorized.into());
        }
        
        greeting.message = message;
        greeting.count = greeting
            .count
            .checked_add(1)
            .ok_or(HelloError::AmountOverflow)?;

        greeting.serialize(&mut &mut greeting_account.data.borrow_mut()[..])?;

        msg!("Message updated. Count: {}", greeting.count);
        Ok(())
    }
}