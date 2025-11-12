use cess_rust_sdk::{
    chain::oss::{query::StorageQuery, types::Oss},
    core::Error,
};

pub async fn get_oss_list() -> Result<Option<Vec<Oss>>, Error> {
    Ok(StorageQuery::oss_list(None).await.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_oss_list() {
        dotenv().ok();

        let result = get_oss_list().await;
        match result {
            Ok(oss_list) => {
                if let Some(oss_list) = oss_list {
                    for oss_info in oss_list {
                        println!("Domain: {:?}", oss_info.account);
                    }
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
