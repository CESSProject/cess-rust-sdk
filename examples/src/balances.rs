use cess_rust_sdk::{
    chain::balances::transaction::StorageTransaction, core::Error,
    polkadot::balances::events::Transfer,
};

const MNEMONIC: &str =
    "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

pub async fn transfer_token() -> Result<(String, Transfer), Error> {
    let storage = StorageTransaction::new(MNEMONIC);
    storage
        .transfer(
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
            100_000_000_000_000_000_000,
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_transfer() {
        dotenv().ok();
        let result = transfer_token().await;
        match result {
            Ok((tx_hash, _)) => {
                println!("{}", tx_hash);
                assert!(true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}
