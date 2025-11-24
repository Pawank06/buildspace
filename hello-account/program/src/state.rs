use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

pub const MAX_MESSAGE_LEN: usize = 200;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Greeting {
    pub owner: Pubkey,
    pub count: u64,
    pub message: String,
}

impl Greeting {
    pub const fn space() -> usize {
        32 +
        8 +
        4 +
        MAX_MESSAGE_LEN
    }
}