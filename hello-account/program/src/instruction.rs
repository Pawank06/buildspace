use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum HelloInstruction {
    Initialize { owner: Pubkey },
    UpdateMessage { message: String },
    IncrementOnly,
    Close,
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let owner = Pubkey::new_from_array(
                    rest.get(..32)
                        .and_then(|slice| slice.try_into().ok())
                        .ok_or(ProgramError::InvalidInstructionData)?,
                );
                Self::Initialize { owner }
            }

            1 => {
                let message = String::from_utf8(rest.to_vec())
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Self::UpdateMessage { message }
            }

            2 => Self::IncrementOnly,

            3 => Self::Close,

            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}