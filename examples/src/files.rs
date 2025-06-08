use cess_rust_sdk::gateway::file::{download, download_encrypt, upload, upload_encrypt};
use cess_rust_sdk::subxt::ext::sp_core::{
    sr25519::{Pair as PairS, Signature},
    Pair,
};
use cess_rust_sdk::utils::account::get_pair_address_as_ss58_address;
use cess_rust_sdk::utils::str::get_random_code;
use std::fmt::Error;

async fn upload_file() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let file = "file.txt";

    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
    let message = get_random_code(16).unwrap();
    let signed_msg = pair.sign(message.as_bytes());

    let response = upload(gateway, file, "Promo Storage", &acc, &message, signed_msg).await;
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

async fn upload_encrypt_with_cyborg() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let file = "file.txt";

    let pair = PairS::from_string(mnemonic, None).unwrap();
    let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();

    let (message, signed_msg) = sign_cess_message(pair).unwrap();

    let response = upload_encrypt(
        gateway,
        file,
        "Promo Storage",
        &acc,
        &message,
        signed_msg,
        "6762b23726e12f101cb57769c73949f90a03340ba53f6c6387e1b5ff3509870b",
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

fn sign_cess_message(pair: PairS) -> Result<(String, Signature), Error> {
    let message = get_random_code(16).expect("Failed to generate random code");
    let signature = pair.sign(message.as_bytes());

    Ok((message, signature))
}

#[cfg(test)]
mod test {

    use cess_rust_sdk::gateway::file::upload_file_in_chunks_resumable;

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

    #[tokio::test]
    async fn test_upload_encrypt_with_cyborg() {
        upload_encrypt_with_cyborg().await;
    }

    const RESUME_FILE_SUFFIX: &str = ".upload_resume";

    #[tokio::test]
    async fn test_upload_file_in_chunks_resumable() {
        use std::path::Path;

        let gateway = "https://deoss-sgp.cess.network";
        let file = "waves.mp4"; // replace with a real test file path
        let file_name = "waves.mp4"; // name to be used in request
        let territory = "Movies";

        let mnemonic =
            "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
        let pair = PairS::from_string(mnemonic, None).unwrap();
        let acc = get_pair_address_as_ss58_address(pair.clone()).unwrap();
        let message = get_random_code(16).unwrap();
        let signature = pair.sign(message.as_bytes());

        // Perform the resumable upload
        let result = upload_file_in_chunks_resumable(
            file, gateway, file_name, territory, &acc, &message, signature,
        )
        .await;

        match result {
            Ok(a) => {

                println!("Upload completed successfully .");
                dbg!(&a);
                // You might also want to verify the server response if needed.
            }
            Err(e) => {
                println!("Upload failed with error: {:?}", e);
                assert!(false, "Upload failed");
            }
        }

        // Optionally assert that resume file is gone
        let resume_path = format!("{}{}", file, RESUME_FILE_SUFFIX);
        assert!(
            !Path::new(&resume_path).exists(),
            "Resume file should be deleted after successful upload"
        );
    }
}
