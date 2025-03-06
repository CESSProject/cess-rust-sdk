use super::upload_response::UploadResponse;
use crate::{
    core::Error,
    utils::{account::get_pair_address_as_ss58_address, str::get_random_code},
};
use base58::ToBase58;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    multipart, Client, RequestBuilder,
};
use std::os::unix::fs::MetadataExt;
use subxt::ext::sp_core::{sr25519::Pair as PairS, Pair};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt as _},
};

pub async fn upload(
    gateway_url: &str,
    file_path: &str,
    territory: &str,
    mnemonic: &str,
) -> Result<UploadResponse, Error> {
    let metadata = fs::metadata(file_path).await?;

    if metadata.is_dir() {
        return Err("Given path is not a file.".into());
    }

    if metadata.size() == 0 {
        return Err("File is an empty file.".into());
    }

    let pair = PairS::from_string(mnemonic, None)?;
    let acc = get_pair_address_as_ss58_address(pair.clone())?;
    let message = get_random_code(16)?;
    let signed_msg = pair.sign(message.as_bytes());

    let mut headers = HeaderMap::new();

    headers.insert("Territory", HeaderValue::from_str(territory)?);
    headers.insert("Account", HeaderValue::from_str(&acc)?);
    headers.insert("Message", HeaderValue::from_str(&message)?);
    headers.insert(
        "Signature",
        HeaderValue::from_str(&signed_msg.0.to_base58())?,
    );

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
    mnemonic: &str,
    save_path: &str,
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

    let pair = PairS::from_string(mnemonic, None)?;
    let acc = get_pair_address_as_ss58_address(pair.clone())?;
    let message = get_random_code(16)?;
    let signed_msg = pair.sign(message.as_bytes());

    let mut headers = HeaderMap::new();

    headers.insert("Operation", HeaderValue::from_static("download"));
    headers.insert("Account", HeaderValue::from_str(&acc)?);
    headers.insert("Message", HeaderValue::from_str(&message)?);
    headers.insert(
        "Signature",
        HeaderValue::from_str(&signed_msg.0.to_base58())?,
    );

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
