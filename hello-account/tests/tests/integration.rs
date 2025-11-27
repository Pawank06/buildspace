use borsh::{BorshDeserialize, BorshSerialize};
use hello_account::{
    processor::Processor,
    state::Greeting
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
}
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn test_initialize_greeting() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "hello_account",
        program_id,
        processor!(Processor::process),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let greeting_account = Keypair::new();
    instruction_data.extend_from_slice(owner.as_ref());

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: instruction_data,
    };

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &greeting_account], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    let account = banks_client
        .get_account(greeting_account.pubkey())
        .await
        .unwrap()
        .unwrap();

    let greeting = Greeting::try_from_slice(&account.data).unwrap();
    assert_eq!(greeting.owner, owner);
    assert_eq!(greeting.count, 0);
    assert_eq!(greeting.message, "Hello, Solana!");

    println!("Test passed: Instialize greeting");
}

#[tokio::test]
async fn test_update_message() {
    let program_id::Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "hello_account_program",
        program_id,
        processor!(Processor::process),
    )

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let greeting_account = Keypair::new();
    let owner = payer.pubkey();

    let mut init_data = vec![0u8];
    init_data.extend_from_slice(owner.as_ref);

    let init_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: init_data,
    };

     let mut transaction = Transaction::new_with_payer(
        &[init_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &greeting_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let new_message = "Updated message!";
    let mut update_data = vec![1u8];
    update_data.extend_from_slice(new_message.as_bytes());
    
    let update_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(payer.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), false),
        ],
        data: update_data,
    };
    
    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[update_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let account = banks_client
        .get_account(greeting_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    
    let greeting = Greeting::try_from_slice(&account.data).unwrap();
    assert_eq!(greeting.message, new_message);
    assert_eq!(greeting.count, 1);

    println!("Test passed: Update message");
}

#[tokio::test]
async fn test_increment_only() {

    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "hello_account_program",
        program_id,
        processor!(Processor::process),
    );
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let greeting_account = Keypair::new();
    let owner = payer.pubkey();
    
    let mut init_data = vec![0u8];
    init_data.extend_from_slice(owner.as_ref());
    
    let init_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: init_data,
    };
    
    let mut transaction = Transaction::new_with_payer(
        &[init_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &greeting_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    
    let increment_data = vec![2u8];
    
    let increment_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(payer.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), false),
        ],
        data: increment_data,
    };
    
    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[increment_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    
    let account = banks_client
        .get_account(greeting_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    
    let greeting = Greeting::try_from_slice(&account.data).unwrap();
    assert_eq!(greeting.count, 1);
    assert_eq!(greeting.message, "Hello, Solana!");
    
    println!("Test passed: Increment only");
}

#[tokio::test]
async fn test_unauthorized_update() {

    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "hello_account_program",
        program_id,
        processor!(Processor::process),
    );
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let greeting_account = Keypair::new();
    let owner = payer.pubkey();
    
    let mut init_data = vec![0u8];
    init_data.extend_from_slice(owner.as_ref());
    
    let init_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: init_data,
    };
    
    let mut transaction = Transaction::new_with_payer(
        &[init_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &greeting_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let unauthorized_user = Keypair::new();
    
    let mut update_data = vec![1u8];
    update_data.extend_from_slice(b"Hacked!");
    
    let update_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(unauthorized_user.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), false),
        ],
        data: update_data,
    };
    
    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[update_instruction],
        Some(&unauthorized_user.pubkey()),
    );
    transaction.sign(&[&unauthorized_user], recent_blockhash);

    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_err());
    
    println!("Test passed: Unauthorized update rejected");
}

#[tokio::test]
async fn test_close_account() {

    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "hello_account_program",
        program_id,
        processor!(Processor::process),
    );
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let greeting_account = Keypair::new();
    let owner = payer.pubkey();
    
    let mut init_data = vec![0u8];
    init_data.extend_from_slice(owner.as_ref());
    
    let init_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: init_data,
    };
    
    let mut transaction = Transaction::new_with_payer(
        &[init_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &greeting_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let close_data = vec![3u8];
    
    let close_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(payer.pubkey(), true),
            AccountMeta::new(greeting_account.pubkey(), false),
            AccountMeta::new(payer.pubkey(), false),
        ],
        data: close_data,
    };
    
    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[close_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let account = banks_client
        .get_account(greeting_account.pubkey())
        .await
        .unwrap();

    if let Some(acc) = account {
        assert_eq!(acc.lamports, 0);
    }
    
    println!("Test passed: Close account");
}
