use cess_rust_sdk::{
    chain::balances::transaction::StorageTransaction, core::Error,
    polkadot::balances::events::Transfer,
};

const MNEMONIC: &str =
    "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

pub async fn transfer_token() -> Result<(String, Transfer), Error> {
    let storage = StorageTransaction::from_mnemonic(MNEMONIC);
    storage
        .transfer(
            "cXk7BTJjoSFTJx1Y2hF2LCFgX83Hh4QGeKpAqCWRXHDDcRtwv",
            5_000_000_000_000_000_000,
        )
        .await
}

// Components m/44'/354'/0'/0'/0'
// m → The master node (root of the HD wallet tree).
// 44' → Purpose field (BIP44 standard).
// 354' → Coin type (for Polkadot/Substrate networks, 354 is the SLIP-0044 coin type for Polkadot).
// 0' → Account index (first account).
// 0' → Change (0 = external addresses).
// 0' → Address index (first address).

pub async fn transfer_token_with_ledger() -> Result<(String, Transfer), Error> {
    let storage = StorageTransaction::from_ledger("m/44'/354'/0'/0'/0'")?;
    let account = ""; // destination account address
    let amount = 5_000_000_000_000_000_000u128;

    storage.transfer(account, amount).await
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

    #[tokio::test]
    async fn test_transfer_token_with_ledger() {
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
