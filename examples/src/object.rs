use std::io::Cursor;

use cess_rust_sdk::gateway::{object::download, object::upload};
use tokio::io::AsyncReadExt;

async fn upload_object() {
    let gateway = "https://deoss-sgp.cess.network";
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice";
    let object = "Hello, this is an object.";
    let reader = Cursor::new(object.as_bytes());

    let response = upload(gateway, reader, "object_name", "territory", mnemonic).await;
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
    let response = download(gateway, fid, mnemonic).await;
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
}
