use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
}
use std::str::FromStr;

use hello_account_program::state::{Greeting, MAX_MESSAGE_LEN};

const PROGRAM_ID: &str = "9SnsoxmcQRYt2a7jbL9Vb84m2HJLjCfV8WwV7c2Joudh";

#[derive(Parser)]
#[command(name = "hello-account-client")]
#[command(about = "CLI client for Hello Account program", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
    }
}