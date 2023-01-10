use std::{env, fs};
use near_units::parse_near;
use serde_json::json;
use workspaces::{Account, Contract, AccountId};
use near_sdk::json_types::{*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;
       
    // Deploy contract
    // Init the worker and start a Sandbox server
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // Create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount( "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;
    let bob = account
        .create_subaccount( "bob")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;
    let carol = account
        .create_subaccount( "carol")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_add_two_donations(&alice, &bob, &contract).await?; // parse_near!("0.008 N")
    test_join_the_lottery(&alice, &contract).await?;
    test_join_the_lottery(&carol, &contract).await?;
    test_pick_lottery_winner_and_withdraw(&account, &contract).await?;
    Ok(())
}

async fn test_add_two_donations(
    user1: &Account,
    user2: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    user1.call( contract.id(), "add_donation")        
        .args_json(json!({"text_message": "first donation"}))
        .deposit(parse_near!("10 N")) 
        .transact()
        .await?
        .into_result()?;

    user2.call( contract.id(), "add_donation")        
        .args_json(json!({"text_message": "second donation"}))
        .deposit(parse_near!("20 N")) 
        .transact()
        .await?
        .into_result()?;

    let total_global_amount_donated: U128 = user1.call(contract.id(), "get_total_global_amount_donated")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(total_global_amount_donated.0, parse_near!("30 N"));
    println!("      Passed ✅ test_add_two_donations");
    Ok(())
}

async fn test_join_the_lottery(
    user: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    user.call(contract.id(), "join_the_lottery")
        .args_json(json!({"account_to_register": user.id()}))
        .transact()
        .await?
        .into_result()?;

    let registered_lottery_accounts: Vec<AccountId> = user.call(contract.id(), "get_registered_lottery_accounts")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    println!("registered_lottery_accounts: {:?}", registered_lottery_accounts);

    let is_user_account_registered: bool = registered_lottery_accounts.contains(user.id());
    assert_eq!(is_user_account_registered, true);
    println!("      Passed ✅ test_join_the_lottery");
    Ok(())
}

async fn test_pick_lottery_winner_and_withdraw(
    user: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let winner_id: AccountId = user.call(contract.id(), "pick_lottery_winner_and_withdraw")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    let registered_lottery_accounts: Vec<AccountId> = user.call(contract.id(), "get_registered_lottery_accounts")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;
    
    // Check if the registered_lottery_accounts vector is empty
    assert_eq!(registered_lottery_accounts.len(), 0);

    let total_global_amount_donated: U128 = user.call(contract.id(), "get_total_global_amount_donated")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    println!("total_global_amount_donated: {}", total_global_amount_donated.0.to_string());

    // Check the remaining 1% of the prize pool
    assert!(total_global_amount_donated.0 <= ((parse_near!("30 N")) as f32 * 0.01) as u128);
    println!("      Passed ✅ test_pick_lottery_winner_and_withdraw");
    Ok(())
}