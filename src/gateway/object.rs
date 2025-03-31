use super::upload_response::UploadResponse;
use crate::core::Error;
use base58::ToBase58;
use futures_util::stream::StreamExt;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Body, Client, RequestBuilder,
};
use subxt::ext::sp_core::sr25519::Signature;
use tokio::io::AsyncRead;
use tokio_util::io::{ReaderStream, StreamReader};

pub async fn upload<R: AsyncRead + Send + Sync + Unpin + 'static>(
    gateway_url: &str,
    reader: R,
    object_name: &str,
    territory: &str,
    acc: &str,
    message: &str,
    signed_msg: Signature,
) -> Result<UploadResponse, Box<dyn std::error::Error>> {
    if object_name.trim().is_empty() {
        return Err("Invalid object name.".into());
    }

    let mut headers = HeaderMap::new();

    // headers.insert("Bucket", HeaderValue::from_str(bucket)?);
    headers.insert("Territory", HeaderValue::from_str(territory)?);
    headers.insert("Account", HeaderValue::from_str(acc)?);
    headers.insert("Message", HeaderValue::from_str(message)?);
    headers.insert(
        "Signature",
        HeaderValue::from_str(&signed_msg.0.to_base58())?,
    );

    let upload_url = format!("{}/object/{}", gateway_url, object_name);

    let client = Client::builder().build()?;

    let stream = ReaderStream::new(reader);
    let body = Body::wrap_stream(
        stream.map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))),
    );

    let request_builder = client.put(upload_url).headers(headers).body(body);

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
) -> Result<impl AsyncRead + Unpin, Error> {
    let mut gateway_url = String::from(gateway_url);

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

    let client = Client::new();
    let request_builder: RequestBuilder = client
        .get(format!("{}{}", download_url, fid))
        .headers(headers);

    let response = request_builder.send().await?;
    let status_code = response.status();

    if !status_code.is_success() {
        return Err("Failed to download.".into());
    }

    let stream = response
        .bytes_stream()
        .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));
    let reader = StreamReader::new(stream);

    Ok(reader)
}
