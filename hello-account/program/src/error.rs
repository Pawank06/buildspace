use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum HelloError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Not Rent Exempt")]
    NotRentExempt,

    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,

    #[error("Amount Overflow")]
    AmountOverflow,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Message Too Long")]
    MessageTooLong,

}

impl From<HelloError> for ProgramError {
    fn from(e: HelloError) -> Self {
        ProgramError::Custom(e as u32)
    }
}