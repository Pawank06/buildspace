use anyhow::Result;
use borsh::BorshDeserialize;
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;

use hello_account::state::{Greeting, MAX_MESSAGE_LEN};

// Replace this with your deployed program ID
const PROGRAM_ID: &str = "9SnsoxmcQRYt2a7jbL9Vb84m2HJLjCfV8WwV7c2Joudh";

#[derive(Parser)]
#[command(name = "hello-account-client")]
#[command(about = "CLI client for Hello Account program", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "http://localhost:8899")]
    rpc_url: String,

    #[arg(short, long, default_value = "~/.config/solana/id.json")]
    keypair_path: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new greeting account
    Init {
        #[arg(short, long, help = "Owner pubkey (defaults to payer)")]
        owner: Option<String>,
    },
    
    /// Update the greeting message
    Update {
        #[arg(short, long, help = "Greeting account address")]
        account: String,
        
        #[arg(short, long, help = "New message")]
        message: String,
    },
    
    /// Increment the counter only
    Increment {
        #[arg(short, long, help = "Greeting account address")]
        account: String,
    },
    
    /// Close the greeting account
    Close {
        #[arg(short, long, help = "Greeting account address")]
        account: String,
    },
    
    /// Get greeting account data
    Get {
        #[arg(short, long, help = "Greeting account address")]
        account: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Setup RPC client
    let rpc_client = RpcClient::new_with_commitment(
        cli.rpc_url.clone(),
        CommitmentConfig::confirmed(),
    );
    
    // Load payer keypair
    let keypair_path = shellexpand::tilde(&cli.keypair_path).to_string();
    let payer = solana_sdk::signature::read_keypair_file(&keypair_path)
        .expect("Failed to read keypair file");
    
    println!("Using payer: {}", payer.pubkey());
    println!("RPC URL: {}", cli.rpc_url);
    
    match cli.command {
        Commands::Init { owner } => {
            let owner_pubkey = if let Some(owner_str) = owner {
                Pubkey::from_str(&owner_str)?
            } else {
                payer.pubkey()
            };
            
            initialize_greeting(&rpc_client, &payer, owner_pubkey)?;
        }
        
        Commands::Update { account, message } => {
            let account_pubkey = Pubkey::from_str(&account)?;
            update_message(&rpc_client, &payer, account_pubkey, message)?;
        }
        
        Commands::Increment { account } => {
            let account_pubkey = Pubkey::from_str(&account)?;
            increment_only(&rpc_client, &payer, account_pubkey)?;
        }
        
        Commands::Close { account } => {
            let account_pubkey = Pubkey::from_str(&account)?;
            close_greeting(&rpc_client, &payer, account_pubkey)?;
        }
        
        Commands::Get { account } => {
            let account_pubkey = Pubkey::from_str(&account)?;
            get_greeting(&rpc_client, account_pubkey)?;
        }
    }
    
    Ok(())
}

fn initialize_greeting(
    client: &RpcClient,
    payer: &Keypair,
    owner: Pubkey,
) -> Result<()> {
    println!("\nInitializing greeting account...");
    
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let greeting_keypair = Keypair::new();
    
    println!("New greeting account: {}", greeting_keypair.pubkey());
    println!("Owner: {}", owner);
    
    // Build instruction data: [variant(1 byte), owner(32 bytes)]
    let mut instruction_data = vec![0u8]; // variant 0 = Initialize
    instruction_data.extend_from_slice(&owner.to_bytes());
    
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_keypair.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: instruction_data,
    };
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer, &greeting_keypair],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&transaction)?;
    
    println!("Success!");
    println!("Signature: {}", signature);
    println!("Greeting Account: {}", greeting_keypair.pubkey());
    
    Ok(())
}

fn update_message(
    client: &RpcClient,
    payer: &Keypair,
    greeting_account: Pubkey,
    message: String,
) -> Result<()> {
    println!("\nUpdating message...");
    
    if message.len() > MAX_MESSAGE_LEN {
        anyhow::bail!("Message too long! Max length: {}", MAX_MESSAGE_LEN);
    }
    
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    
    // Build instruction data: [variant(1 byte), message(variable)]
    let mut instruction_data = vec![1u8]; // variant 1 = UpdateMessage
    instruction_data.extend_from_slice(message.as_bytes());
    
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_account, false),
        ],
        data: instruction_data,
    };
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&transaction)?;
    
    println!("Success!");
    println!("Signature: {}", signature);
    
    Ok(())
}

fn increment_only(
    client: &RpcClient,
    payer: &Keypair,
    greeting_account: Pubkey,
) -> Result<()> {
    println!("\nIncrementing counter...");
    
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    
    // Build instruction data: [variant(1 byte)]
    let instruction_data = vec![2u8]; // variant 2 = IncrementOnly
    
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_account, false),
        ],
        data: instruction_data,
    };
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&transaction)?;
    
    println!("Success!");
    println!("Signature: {}", signature);
    
    Ok(())
}

fn close_greeting(
    client: &RpcClient,
    payer: &Keypair,
    greeting_account: Pubkey,
) -> Result<()> {
    println!("\nClosing greeting account...");
    
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    
    // Build instruction data: [variant(1 byte)]
    let instruction_data = vec![3u8]; // variant 3 = Close
    
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_account, false),
            AccountMeta::new(payer.pubkey(), false), // destination for lamports
        ],
        data: instruction_data,
    };
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&transaction)?;
    
    println!("Success!");
    println!("Signature: {}", signature);
    println!("Account closed and lamports returned");
    
    Ok(())
}

fn get_greeting(client: &RpcClient, greeting_account: Pubkey) -> Result<()> {
    println!("\nFetching greeting account...");
    
    let account_data = client.get_account_data(&greeting_account)?;
    let greeting = Greeting::try_from_slice(&account_data)?;
    
    println!("\nGreeting Account Data:");
    println!("  Owner: {}", greeting.owner);
    println!("  Count: {}", greeting.count);
    println!("  Message: \"{}\"", greeting.message);
    
    Ok(())
}