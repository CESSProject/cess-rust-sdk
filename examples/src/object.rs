use cess_rust_sdk::gateway::object::{download_encrypt, upload_encrypt};
use cess_rust_sdk::subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use cess_rust_sdk::{
    gateway::object::{download, upload},
    utils::{account::get_pair_address_as_ss58_address, str::get_random_code},
};
use std::io::Cursor;
use tokio::io::AsyncReadExt;

async fn upload_object() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let object = "Hello, this is an object.";
    let reader = Cursor::new(object.as_bytes());

    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());

    let response = upload(
        gateway,
        reader,
        "object_name",
        "territory",
        &acc,
        &message,
        signed_msg,
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

async fn upload_object_encrypt() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let object = "Hello, this is an object.";
    let reader = Cursor::new(object.as_bytes());

    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());

    let response = upload_encrypt(
        gateway,
        reader,
        "object_name",
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

async fn download_object() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let fid = "67d1acf19a8970ce9117d016708098189088e3c4d10799add8d1a04d383ddd56";
    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());

    let response = download(gateway, fid, &acc, &message, signed_msg).await;

    match response {
        Ok(mut reader) => {
            let mut buffer = Vec::new();
            if let Err(e) = reader.read_to_end(&mut buffer).await {
                println!("Failed to read response: {:?}", e);
                return;
            }

            // Convert the buffer to a String
            match String::from_utf8(buffer) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("Failed to parse content as UTF-8: {:?}", e),
            }
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false);
        }
    }
}

async fn download_object_encrypt() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let fid = "51780c734db86de781b9c2f7f6ac4d7296a0c0bf18f4c3850a3a18e37d1ed809";
    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());

    let response = download_encrypt(gateway, fid, &acc, &message, signed_msg, "password").await;

    match response {
        Ok(mut reader) => {
            let mut buffer = Vec::new();
            if let Err(e) = reader.read_to_end(&mut buffer).await {
                println!("Failed to read response: {:?}", e);
                return;
            }

            // Convert the buffer to a String
            match String::from_utf8(buffer) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("Failed to parse content as UTF-8: {:?}", e),
            }
        }
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
    async fn test_upload_object() {
        upload_object().await;
    }

    #[tokio::test]
    async fn test_download_object() {
        download_object().await;
    }

    #[tokio::test]
    async fn test_upload_object_encrypt() {
        upload_object_encrypt().await;
    }

    #[tokio::test]
    async fn test_download_object_encrypt() {
        download_object_encrypt().await;
    }
}
