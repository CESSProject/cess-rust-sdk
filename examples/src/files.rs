use cess_rust_sdk::gateway::file::{download, upload};

async fn upload_file() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let file = "file.txt";
    let response = upload(gateway, file, "hello", "hello", mnemonic).await;
    match response {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

async fn download_file() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let fid = "48609e0f30979f40f838deeed66da835086f787fe6dae2f8dbe364afd28793b6";
    let response = download(gateway, fid, mnemonic, "download.txt").await;
    match response {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_upload_file() {
        upload_file().await;
    }

    #[tokio::test]
    async fn test_download_file() {
        download_file().await;
    }
}
