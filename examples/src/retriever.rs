#[cfg(test)]
mod tests {
    use cess_rust_sdk::chain::{AnySigner, DynSigner};
    use cess_rust_sdk::retriever::gateway::{
        BatchUploadRequest, UploadFileOpts, authorize_gateways, batch_upload_file, download_data, gen_gateway_access_token, request_batch_upload, upload_file, wrap_message_for_signing
    };
    use cess_rust_sdk::subxt::PolkadotConfig;
    use cess_rust_sdk::subxt::ext::sp_core::sr25519::Pair as Sr25519Pair;
    use cess_rust_sdk::subxt::ext::sp_core::Pair;
    use cess_rust_sdk::subxt::tx::PairSigner;
    use tokio::io::AsyncWriteExt as _;

    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio;
    use cess_rust_sdk::subxt::ext::sp_core::{sr25519::Pair as PairS};

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

    #[tokio::test]
    async fn test_batch_upload_request() {
        let message = "123456";
        let territory = "test1";
        let filename = "testfile1";
        let file_size = 4096 * 1024; // 4 MB

        let mnemonic =
            "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
        let base_url = "http://154.194.34.195:1306";
        let pair = Sr25519Pair::from_phrase(mnemonic, None).unwrap().0;
        let acc =
            cess_rust_sdk::utils::account::get_pair_address_as_ss58_address(pair.clone()).unwrap();

        // derive sr25519 keypair and sign the message
        let wrapped = wrap_message_for_signing(message.as_bytes());
        let signature = pair.sign(&wrapped);

        // get gateway token
        let token = gen_gateway_access_token(base_url, message, &acc, signature.as_ref())
            .await
            .expect("failed to gen token");
        println!("token: {}", token);

        // create request
        let req = BatchUploadRequest {
            base_url: base_url,
            token: &token,
            territory,
            filename,
            file_size,
            encrypt: false,
            async_upload: false,
            no_tx_proxy: false,
        };

        let hash = request_batch_upload(req)
            .await
            .expect("failed to request batch upload");

        println!("batch upload hash: {}", hash);
    }

    /// Upload the actual file in chunks using the batch upload hash
    #[tokio::test]
    async fn test_batch_upload_file() {
        use std::path::Path;
        use tokio::fs::File;

        let message = "123456";
        let mnemonic =
            "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
        let base_url = "http://154.194.34.195:1306";

        let pair = Sr25519Pair::from_phrase(mnemonic, None).unwrap().0;
        let acc =
            cess_rust_sdk::utils::account::get_pair_address_as_ss58_address(pair.clone()).unwrap();

        let wrapped = wrap_message_for_signing(message.as_bytes());
        let signature = pair.sign(&wrapped);

        let token = gen_gateway_access_token(base_url, message, &acc, signature.as_ref())
            .await
            .expect("failed to gen token");
        println!("token: {}", token);

        // Step 1: request new batch upload session
        const TOTAL_SIZE: i64 = 4096 * 1024;
        let req = BatchUploadRequest {
            base_url,
            token: &token,
            territory: "test1",
            filename: "testfile1",
            file_size: TOTAL_SIZE,
            encrypt: false,
            async_upload: false,
            no_tx_proxy: false,
        };

        let hash = request_batch_upload(req)
            .await
            .expect("failed to request batch upload");
        println!("batch upload hash: {}", hash);

        // Step 2: prepare 4 MB file
        let file_path = Path::new("./testfile1");
        let mut file = File::create(&file_path)
            .await
            .expect("failed to create file");

        let random_data: Vec<u8> = (0..TOTAL_SIZE).map(|_| rand::random::<u8>()).collect();
        file.write_all(&random_data)
            .await
            .expect("failed to write random data");
        file.sync_all().await.expect("sync file");
        drop(file);

        let metadata = tokio::fs::metadata(&file_path).await.expect("metadata");
        assert_eq!(metadata.len(), TOTAL_SIZE as u64, "file incomplete");

        // Step 3: upload in chunks using the hash
        let chunk_size: u64 = 512 * 1024;
        let total_size = TOTAL_SIZE as u64;
        let mut start = 0u64;

        while start < total_size {
            let end = std::cmp::min(start + chunk_size, total_size);

            let mut f = tokio::fs::File::open(&file_path)
                .await
                .expect("failed to open file");

            let res = batch_upload_file(base_url, &token, &hash, &mut f, start, end)
                .await
                .expect("batch upload failed");

            println!(
                "uploaded chunk {}-{}, fid: {}, chunk_end: {}, file_name: {:?}",
                start, end, res.fid, res.chunk_end, res.file_info.file_name
            );

            start = end;
        }
    }

    #[tokio::test]
    async fn test_authorize_gateways() {
        let mnemonic =
            "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";

        let pair = PairS::from_string(mnemonic, None).unwrap();
        let boxed: AnySigner = Box::new(PairSigner::<PolkadotConfig, _>::new(pair));
        let signer = DynSigner::new(boxed);
        match authorize_gateways("http://154.194.34.195:1306", signer).await {
            Ok(()) => assert!(true),
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
        
        
        
    }
}
