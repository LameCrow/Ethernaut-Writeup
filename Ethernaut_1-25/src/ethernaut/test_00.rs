#[allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        Address, B256, Bytes, U256, address,
        keccak256,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol!{
    #[sol(rpc)]
    contract Token {
        function transfer(address _to, uint256 _value) public returns (bool);
        function balanceOf(address _owner) public view returns (uint256 balance);
    }
}

use crate::ethernaut;
use crate::macros::*;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn test00_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(test00())?;
    Ok(())
}


async fn test00() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let token_address = "0x9E4EdAbaF9BA7E84386a2c4F0026aC021B0A54b0".parse()?;
    let token = Token::new(token_address, provider.clone());
    let alice = ethernaut::ALICE.parse()?;
    let bob = ethernaut::BOB.parse()?;

    println!("alice's balance = {}", token.balanceOf(alice).call().await?);
    println!("bob's balance   = {}\n", token.balanceOf(bob).call().await?);
    
    let transfer_tx = token.transfer(bob, U256::from(100))
        .send()
        .await?;
    let _ = wait_tx!(transfer_tx, "Token.transfer()");

    println!("alice's balance = {}", token.balanceOf(alice).call().await?);
    println!("bob's balance   = {}\n", token.balanceOf(bob).call().await?);
    
    Ok(())
}

