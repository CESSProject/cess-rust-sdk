use cess_rust_sdk::gateway::file::{download, download_encrypt, upload, upload_encrypt};
use cess_rust_sdk::subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use cess_rust_sdk::utils::account::get_pair_address_as_ss58_address;
use cess_rust_sdk::utils::str::get_random_code;

async fn upload_file() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let file = "file.txt";

    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());

    let response = upload(gateway, file, "territory", &acc, &message, signed_msg).await;
    match response {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

async fn upload_file_encrypt() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let file = "file.txt";

    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());

    let response = upload_encrypt(
        gateway,
        file,
        "territory",
        &acc,
        &message,
        signed_msg,
        "password",
    )
    .await;
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
    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());
    let response = download(gateway, fid, &acc, &message, signed_msg, "download.txt").await;
    match response {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

async fn download_file_encrypt() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let fid = "6ee3c755442a924c2a5b58e2487e3a5b9c2cd46694b9ead7529a307e7fd4b986";
    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());
    let response = download_encrypt(
        gateway,
        fid,
        &acc,
        &message,
        signed_msg,
        "download.txt",
        "password",
    )
    .await;
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

    #[tokio::test]
    async fn test_upload_file_encrypt() {
        upload_file_encrypt().await;
    }

    #[tokio::test]
    async fn test_download_file_encrypt() {
        download_file_encrypt().await;
    }
}
