use super::upload_response::UploadResponse;
use crate::core::Error;
use base58::ToBase58;
use mime_guess::from_path;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart, Client, RequestBuilder,
};
use std::io::Write;
use std::{fs::OpenOptions, os::unix::fs::MetadataExt, path::Path, time::Duration};
use subxt::ext::sp_core::sr25519::Signature;
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncSeekExt as _, AsyncWriteExt as _, BufReader},
};

const CHUNK_SIZE: usize = 200 * 1024 * 1024;
const MAX_RETRIES: u8 = 5;
const RESUME_FILE_SUFFIX: &str = ".upload_resume";

pub async fn upload(
    gateway_url: &str,
    file_path: &str,
    territory: &str,
    acc: &str,
    message: &str,
    signed_msg: Signature,
) -> Result<UploadResponse, Error> {
    upload_file(
        gateway_url,
        file_path,
        territory,
        acc,
        message,
        signed_msg,
        None,
    )
    .await
}

pub async fn upload_encrypt(
    gateway_url: &str,
    file_path: &str,
    territory: &str,
    acc: &str,
    message: &str,
    signed_msg: Signature,
    cipher: &str,
) -> Result<UploadResponse, Error> {
    if cipher.is_empty() {
        return Err(Error::Custom("cipher cannot be empty!".into()));
    }
    upload_file(
        gateway_url,
        file_path,
        territory,
        acc,
        message,
        signed_msg,
        Some(cipher.to_string()),
    )
    .await
}

async fn upload_file(
    gateway_url: &str,
    file_path: &str,
    territory: &str,
    acc: &str,
    message: &str,
    signed_msg: Signature,
    cipher: Option<String>,
) -> Result<UploadResponse, Error> {
    let metadata = fs::metadata(file_path).await?;

    if metadata.is_dir() {
        return Err("Given path is not a file.".into());
    }

    if metadata.size() == 0 {
        return Err("File is an empty file.".into());
    }

    let mut headers = HeaderMap::new();

    headers.insert("Territory", HeaderValue::from_str(territory)?);
    headers.insert("Account", HeaderValue::from_str(acc)?);
    headers.insert("Message", HeaderValue::from_str(message)?);
    headers.insert(
        "Signature",
        HeaderValue::from_str(&signed_msg.0.to_base58())?,
    );

    if let Some(cipher) = cipher {
        headers.insert("Cipher", HeaderValue::from_str(&cipher)?);
    }

    let mut form = multipart::Form::new();

    let upload_url = format!("{}/file", gateway_url);

    let mut file = File::open(file_path).await?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).await?;

    form = form.part(
        "file",
        multipart::Part::stream(file_content.clone()).file_name(file_path.to_string()),
    );

    let client = Client::builder().build()?;

    let request_builder: RequestBuilder = client.put(upload_url).headers(headers).multipart(form);

    let response = request_builder.send().await?;
    let status_code = response.status();
    let upload_response: UploadResponse = response.json().await?;

    if !status_code.is_success() {
        return Err("DeOss service failure, please retry or contact administrator.".into());
    }
    Ok(upload_response)
}

pub async fn upload_file_in_chunks_resumable(
    file_path: &str,
    gateway_url: &str,
    file_name: &str,
    territory: &str,
    account: &str,
    message: &str,
    signed_msg: Signature,
) -> Result<UploadResponse, Error> {
    let path = Path::new(file_path);
    let mut file = BufReader::new(File::open(path).await?);
    let metadata = fs::metadata(path).await?;
    let file_size = metadata.len();

    let resume_path = format!("{}{}", file_path, RESUME_FILE_SUFFIX);
    let mut start = read_resume_point(&resume_path).unwrap_or(0);

    if start >= file_size {
        println!(
            "Invalid resume point {} for file size {}, resetting to 0.",
            start, file_size
        );
        start = 0;
    }

    if start > 0 {
        println!("Resuming from byte {}...", start);
        file.seek(tokio::io::SeekFrom::Start(start)).await?;
    }

    let client = Client::new();
    let mut buffer = vec![0; CHUNK_SIZE];

    while start < file_size {
        let end = (start + CHUNK_SIZE as u64 - 1).min(file_size - 1);

        if start > end || end >= file_size {
            println!("Invalid chunk range: start={}, end={}, file_size={}", start, end, file_size);
            break;
        }

        let chunk_len = (end - start + 1) as usize;
        file.read_exact(&mut buffer[..chunk_len]).await?;

        println!(
            "Uploading chunk: start={}, end={}, file_size={}, chunk_len={}",
            start, end, file_size, chunk_len
        );

        let mut headers = HeaderMap::new();
        headers.insert("Territory", HeaderValue::from_str(territory)?);
        headers.insert("Account", HeaderValue::from_str(account)?);
        headers.insert("Message", HeaderValue::from_str(message)?);
        headers.insert(
            "Signature",
            HeaderValue::from_str(&signed_msg.0.to_base58())?,
        );

        let content_range = format!("bytes {}-{}/{}", start, end, file_size);
        headers.insert("Content-Range", HeaderValue::from_str(&content_range)?);
        headers.insert("Content-Length", HeaderValue::from(chunk_len as u64));

        println!("Sending Content-Range: {}", content_range);

        let mime_type = from_path(file_path)
            .first_or_octet_stream()
            .essence_str()
            .to_string();
        headers.insert("Content-Type", HeaderValue::from_str(&mime_type)?);

        let url = format!("{}/resume/{}", gateway_url, file_name);
        let mut attempt = 0;

        loop {
            let resp = client
                .put(&url)
                .headers(headers.clone())
                .body(buffer[..chunk_len].to_vec())
                .send()
                .await;

            match resp {
                Ok(response) => {
                    let status = response.status().as_u16();

                    if status == 308 {
                        println!("Chunk {}-{} uploaded, continuing...", start, end);
                        save_resume_point(&resume_path, end + 1)?;
                        start = end + 1;
                        break;
                    } else if status == 200 {
                        let text = response.text().await?;
                        let upload_response: UploadResponse =
                            serde_json::from_str(&text).map_err(|e| {
                                Error::Custom(format!("Failed to parse response: {}", e))
                            })?;
                        println!("Final chunk uploaded successfully: {}-{}", start, end);
                        let _ = fs::remove_file(&resume_path).await;
                        return Ok(upload_response);
                    } else {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::Custom(format!(
                            "Unexpected status {}: {}",
                            status, body
                        )));
                    }
                }
                Err(e) => {
                    attempt += 1;
                    eprintln!("Network error uploading chunk {}-{}: {}", start, end, e);
                }
            }

            if attempt >= MAX_RETRIES {
                return Err(Error::Custom(format!(
                    "Failed to upload chunk {}-{} after {} retries",
                    start, end, attempt
                )));
            }

            let backoff = 2u64.pow(attempt.into()) * 100;
            println!("Retrying in {}ms...", backoff);
            tokio::time::sleep(Duration::from_millis(backoff)).await;
        }
    }

    Err(Error::Custom(
        "Upload finished but server did not return final response.".into(),
    ))
}

fn save_resume_point(path: &str, byte: u64) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    writeln!(file, "{}", byte)?;
    Ok(())
}

fn read_resume_point(path: &str) -> Option<u64> {
    if let Ok(content) = std::fs::read_to_string(path) {
        if let Ok(byte) = content.trim().parse::<u64>() {
            return Some(byte);
        }
    }
    None
}

pub async fn download(
    gateway_url: &str,
    fid: &str,
    acc: &str,
    message: &str,
    signed_msg: Signature,
    save_path: &str,
) -> Result<(), Error> {
    download_file(gateway_url, fid, acc, message, signed_msg, save_path, None).await
}

pub async fn download_encrypt(
    gateway_url: &str,
    fid: &str,
    acc: &str,
    message: &str,
    signed_msg: Signature,
    save_path: &str,
    cipher: &str,
) -> Result<(), Error> {
    download_file(
        gateway_url,
        fid,
        acc,
        message,
        signed_msg,
        save_path,
        Some(cipher.to_string()),
    )
    .await
}

async fn download_file(
    gateway_url: &str,
    fid: &str,
    acc: &str,
    message: &str,
    signed_msg: Signature,
    save_path: &str,
    cipher: Option<String>,
) -> Result<(), Error> {
    let mut save_path = String::from(save_path);
    let mut gateway_url = String::from(gateway_url);

    if let Ok(metadata) = fs::metadata(&save_path).await {
        if metadata.is_dir() {
            save_path = format!("{}/{}", save_path, fid);
        }

        if metadata.len() == 0 {
            return Ok(());
        }
    }

    if gateway_url.is_empty() {
        return Err("Invalid gateway url.".into());
    }

    if !gateway_url.ends_with('/') {
        gateway_url = format!("{}/", gateway_url);
    }

    let download_url = format!("{}file/download/", gateway_url);

    let mut headers = HeaderMap::new();

    headers.insert("Operation", HeaderValue::from_static("download"));
    headers.insert("Account", HeaderValue::from_str(acc)?);
    headers.insert("Message", HeaderValue::from_str(message)?);
    headers.insert(
        "Signature",
        HeaderValue::from_str(&signed_msg.0.to_base58())?,
    );

    if let Some(cipher) = cipher {
        headers.insert("Cipher", HeaderValue::from_str(&cipher)?);
    }

    let client = Client::new();
    let request_builder: RequestBuilder = client
        .get(format!("{}{}", download_url, fid))
        .headers(headers);

    let f = File::create(&save_path).await?;
    let response = request_builder.send().await?;
    let status_code = response.status();

    if !status_code.is_success() {
        return Err("Failed to download.".into());
    }
    let mut writer = f;

    let mut response_body = response.bytes().await?;
    while !response_body.is_empty() {
        let bytes_written = writer.write(&response_body).await?;
        response_body = response_body[bytes_written..].to_vec().into();
    }

    Ok(())
}
