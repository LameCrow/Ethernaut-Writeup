#[allow(unused_imports)]
use alloy::{
    sol_types::SolCall,
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
    contract Exp {
        function exploit() external payable;
    }
}

use crate::ethernaut;
#[allow(unused_imports)]
use crate::utils;
use crate::macros::wait_tx;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level())?;

    Ok(())
}

async fn level() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let exp_address = "000".parse()?;
    let exp = Exp::new(exp_address, provider.clone());


    let tx = exp.exploit()
        .send()
        .await?;
    let _ = wait_tx!(tx, "xxx.xxx()");
    
    Ok(())
}
