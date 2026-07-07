#[allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        U256, address, Address, Bytes,
        utils::{Unit, format_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};

sol!{
    #[sol(rpc)]
    contract Solver {
        function whatIsTheMeaningOfLife() external pure returns (uint256);
    }

    #[sol(rpc)]
    contract MagicNum {
        function setSolver(address _solver) public;
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level18_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level18())?;

    Ok(())
}

async fn level18() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let calldata: Bytes = "600a600c600039600a6000f3602a60a052602060a0f3".parse()?;
    let request = TransactionRequest::default()
        .with_deploy_code(calldata);
    let creation_tx = provider.send_transaction(request).await?;
    println!(
        "[Creation]    Panding transaction... {}",
        creation_tx.tx_hash()
    );

    let creation_receipt = creation_tx.get_receipt().await?;
    println!(
        "[Creation]    transaction included in block {}\n",
        creation_receipt.block_number.expect("Failed to get block number")
    );

    let solver_address = creation_receipt
        .contract_address
        .expect("Failed to get contract address");
    let solver = Solver::new(solver_address, provider.clone());
    println!("[TEST]    solver = {solver_address}");

    let test_num = solver.whatIsTheMeaningOfLife()
        .call()
        .await?;

    println!("[TEST]    test_num = {test_num}");

    let magic_address = "0xecdED86392B4d8101d5722E3e2c3c3CB44dD65B8".parse()?;
    let magic = MagicNum::new(magic_address, provider.clone());

    let solver_tx = magic.setSolver(solver_address)
        .send()
        .await?;
    println!("[MagicNum.setSolver()]    Panding trasaction... {}",
        solver_tx.tx_hash()
    );

    let solver_receipt = solver_tx.get_receipt().await?;
    println!(
        "[MagicNum.setSolver()]    transaction included in block {}",
        solver_receipt.block_number.expect("Failed to get block number")
    );
        
    Ok(())
}
