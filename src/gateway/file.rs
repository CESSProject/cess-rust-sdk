use super::upload_response::UploadResponse;
use crate::core::Error;
use base58::ToBase58;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart, Client, RequestBuilder,
};
use subxt::ext::sp_core::sr25519::Signature;
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt as _},
};

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

    if metadata.len() == 0 {
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
