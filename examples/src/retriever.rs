#[cfg(test)]
mod tests {
    use cess_rust_sdk::retriever::gateway::{
        download_data, gen_gateway_access_token, upload_file, wrap_message_for_signing,
        UploadFileOpts,
    };
    use cess_rust_sdk::subxt::ext::sp_core::sr25519::Pair as Sr25519Pair;
    use cess_rust_sdk::subxt::ext::sp_core::Pair;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio;

    #[tokio::test]
    async fn test_upload() {
        let base_url = "http://154.194.34.195:1306";
        let territory = "test1";
        let filename = "file.txt";
        let mnemonic =
            "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

        // generate keypair from mnemonic
        let pair = Sr25519Pair::from_phrase(mnemonic, None).unwrap().0;
        let acc =
            cess_rust_sdk::utils::account::get_pair_address_as_ss58_address(pair.clone()).unwrap();

        let message = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let wrapped = wrap_message_for_signing(message.as_bytes());

        let signed_msg = pair.sign(&wrapped);

        // request access token
        let token = gen_gateway_access_token(base_url, &message, &acc, signed_msg.as_ref())
            .await
            .expect("failed to gen token");

        println!("token: {}", token);

        let file_path = std::path::Path::new(&filename);
        if !file_path.exists() {
            panic!("File not found: {:?}", file_path);
        }

        // upload params
        let opts = UploadFileOpts {
            base_url,
            token: &token,
            territory,
            filename,
            file_path,
            async_upload: true,
            no_proxy: false,
            encrypt: false,
        };

        let resp_bytes = upload_file(opts).await.unwrap();
        let resp_str = String::from_utf8_lossy(&resp_bytes);
        println!("Upload response: {}", resp_str);
    }

    #[tokio::test]
    async fn test_download() {
        let base_url = "http://154.194.34.195:1306";
        let fid = "3914a07f9c4123111e148f1038f29cd25d151f31e7da833bd902c98b21ced4a3";

        download_data(
            base_url,
            fid,
            "",
            std::path::Path::new("./test_file_downloaded.txt"),
            None,
            None,
            None,
        )
        .await
        .expect("download");
    }
}
