use alloy::{
    network::TransactionBuilder,
    primitives::{
        U256, address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

use crate::ethernaut;
use std::error::Error;

use tokio::runtime::Runtime;

#[allow(dead_code)]
async fn test_1() -> Result<(), Box<dyn Error>> {
    let signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(ethernaut::RPC)
        .await?;

    let alice = address!("0xb309c9C1b79eBE28aF0460D6a9b4F01526e62FC3");
    // let account: Address = ethernaut::ACCOUNT.parse()?;
    let value = U256::from(1000u64);
    let tx = TransactionRequest::default()
        .with_to(alice)
        .with_value(value);

    let pending_tx = provider.send_transaction(tx).await?;
    println!("Pending transaction... {}", pending_tx.tx_hash());

    let receipt = pending_tx.get_receipt().await?;
    println!(
        "Transaction included in block {}",
        receipt.block_number.expect("Failed to get block number")
    );

    println!("Transferred {:.5} ETH to {alice}", format_ether(value));

    Ok(())
}

#[allow(dead_code)]
pub fn test_1_runner() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;
    rt.block_on(test_1())?;

    Ok(())
}

// Generate bindings for the WETH9 contract
sol! {
    #[sol(rpc)]
    contract WETH9 {
        function deposit() public payable;
        function balanceOf(address) public view returns (uint256);
        function withdraw(uint amount) public;
    }
}

#[allow(dead_code)]
async fn test_2() -> Result<(), Box<dyn Error>> {
    // Initialize a signer with a private key and get address from it
    let signer: PrivateKeySigner =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
    let from_address = signer.address();

    // Instantiate a provider with the signer
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(ethernaut::RPC)
        .await?;

    // Setup WETH contract instance
    let weth_address = address!("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    let weth = WETH9::new(weth_address, provider.clone());

    // Read initial balance
    let initial_balance = weth.balanceOf(from_address).call().await?;
    println!(
        "Initial WETH balance: {} WETH",
        format_ether(initial_balance)
    );

    // Write: Deposit ETH to get WETH
    let deposit_amount = Unit::ETHER.wei().saturating_mul(U256::from(10));
    let deposit_tx = weth.deposit().value(deposit_amount).send().await?;
    let deposit_receipt = deposit_tx.get_receipt().await?;
    println!(
        "Deposited ETH in block {}",
        deposit_receipt
            .block_number
            .expect("Failed to get block number")
    );

    // Read: Check updated balance after deposit
    let new_balance = weth.balanceOf(from_address).call().await?;
    println!("New WETH balance: {} WETH", format_ether(new_balance));

    // Write: Withdraw some WETH back to ETH
    let withdraw_amount = Unit::ETHER.wei().saturating_mul(U256::from(5));
    let withdraw_tx = weth.withdraw(withdraw_amount).send().await?;
    let withdraw_receipt = withdraw_tx.get_receipt().await?;
    println!(
        "Withdrew ETH in block {}",
        withdraw_receipt
            .block_number
            .expect("Failed to get block number")
    );

    // Read: Final balance check
    let final_balance = weth.balanceOf(from_address).call().await?;
    println!("Final WETH balance: {} WETH", format_ether(final_balance));

    Ok(())
}
