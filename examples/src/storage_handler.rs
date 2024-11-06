use cess_rust_sdk::chain::storage_handler::query::StorageQuery;

pub async fn get_unit_price() -> Result<Option<u128>, Box<dyn std::error::Error>> {
    Ok(StorageQuery::unit_price(None).await.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_unit_price() {
        dotenv().ok();

        let result = get_unit_price().await;
        match result {
            Ok(territory_info) => {
                if let Some(price) = territory_info {
                    assert_eq!(price, 30000000000000000000u128);
                }
                assert!(true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}
