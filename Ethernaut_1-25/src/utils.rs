#[allow(unused_imports)]
use alloy::{
    network::TransactionBuilder,
    primitives::{
        Address, U256, address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

use std::error::Error;
// use tokio::runtime::Runtime;

use crate::ethernaut;

pub async fn balance(account: Address) -> Result<U256, Box<dyn Error>> {
    let signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(ethernaut::RPC)
        .await?;

    let balance = provider.get_balance(account).await?;

    Ok(balance)
}
