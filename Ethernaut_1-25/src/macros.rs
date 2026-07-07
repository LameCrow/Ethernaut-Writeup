macro_rules! wait_tx {
    ($tx:expr, $name:expr) => {{
        println!("[{}]    Pending tx... {}", $name, $tx.tx_hash());

        let receipt = $tx.get_receipt().await?;

        println!(
            "[{}]    Transaction included in block {}\n",
            $name,
            receipt.block_number.expect("Failed to get block number")
        );

        receipt
    }};
}

pub(crate) use wait_tx;
