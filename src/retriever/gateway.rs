use base64::{engine::general_purpose::STANDARD, Engine as _};
use hex;
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, error::Error, fs::File, io::Read, path::Path};
use subxt::ext::sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use subxt::ext::sp_core::{sr25519, ByteArray as _};
use tokio::io::{AsyncReadExt as _, AsyncSeekExt as _};

pub enum Endpoint {
    GenToken,
    UploadFile,
    BatchUpload,
    BatchRequest,
    GetFile,
    ReEncrypt,
    Capsule,
}

impl Endpoint {
    pub fn path(&self) -> &'static str {
        match self {
            Endpoint::GenToken => "/gateway/gentoken",
            Endpoint::UploadFile => "/gateway/upload/file",
            Endpoint::BatchUpload => "/gateway/upload/batch/file",
            Endpoint::BatchRequest => "/gateway/upload/batch/request",
            Endpoint::GetFile => "/gateway/download",
            Endpoint::ReEncrypt => "/gateway/reencrypt",
            Endpoint::Capsule => "/gateway/capsule",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReencryptReq {
    pub did: String,
    pub capsule: Vec<u8>,
    pub rk: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub fid: String,
    pub file_name: String,
    pub base_dir: Option<String>,
    pub file_size: Option<i64>,
    pub owner: Option<Vec<u8>>,
    pub territory: Option<String>,
    pub segments: Option<Vec<String>>,
    pub fragments: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchUploadResp {
    pub fid: String,
    pub chunk_end: i64,
    pub file_info: FileInfo,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BatchFilesInfo {
    pub hash: Option<String>,
    pub file_name: Option<String>,
    pub owner: Option<Vec<u8>>,
    pub territory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_upload: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_tx_proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

pub struct UploadFileOpts<'a> {
    pub base_url: &'a str,
    pub token: &'a str,
    pub territory: &'a str,
    pub filename: &'a str,
    pub file_path: &'a Path,
    pub async_upload: bool,
    pub no_proxy: bool,
    pub encrypt: bool,
}

#[derive(Debug)]
pub struct BatchUploadRequest<'a> {
    pub base_url: &'a str,
    pub token: &'a str,
    pub territory: &'a str,
    pub filename: &'a str,
    pub file_size: i64,
    pub encrypt: bool,
    pub async_upload: bool,
    pub no_tx_proxy: bool,
}

#[allow(dead_code)]
const DEFAULT_PART_SIZE: usize = 32 * 1024 * 1024;

/// generic http helper
pub async fn send_http_request(
    method: Method,
    url: &str,
    headers: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = Client::new();
    let mut req = client.request(method, url);

    if let Some(hdrs) = headers {
        for (k, v) in hdrs {
            req = req.header(k, v);
        }
    }

    if let Some(data) = body {
        req = req.body(data);
    }

    let resp = req.send().await?;
    let status = resp.status();
    let bytes = resp.bytes().await?;

    if !status.is_success() {
        return Err(format!("HTTP {}: {}", status, String::from_utf8_lossy(&bytes)).into());
    }

    Ok(bytes.to_vec())
}

/// Proxy re-encryption
pub async fn proxy_re_encrypt(
    base_url: &str,
    token: &str,
    did: &str,
    capsule: &[u8],
    rk: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
    let url = format!("{}{}", base_url, Endpoint::ReEncrypt.path());
    let req = ReencryptReq {
        did: did.to_string(),
        capsule: capsule.to_vec(),
        rk: rk.to_vec(),
    };
    let body = serde_json::to_vec(&req)?;
    let mut headers = HashMap::new();
    headers.insert("Authorization".into(), format!("Bearer {}", token));

    let resp_bytes = send_http_request(Method::POST, &url, Some(headers), Some(body)).await?;
    let resp: Response<Vec<u8>> = serde_json::from_slice(&resp_bytes)?;
    Ok(resp.data)
}

/// Download file
pub async fn download_data(
    base_url: &str,
    fid: &str,
    segment: &str,
    fpath: &Path,
    capsule: Option<&[u8]>,
    rk: Option<&[u8]>,
    pkx: Option<&[u8]>,
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "{}/{}/{}",
        base_url.trim_end_matches('/'),
        Endpoint::GetFile.path().trim_start_matches('/'),
        format_args!("{}/{}", fid, segment)
    );

    let mut headers = HashMap::new();
    if let (Some(caps), Some(rk), Some(pkx_bytes)) = (capsule, rk, pkx) {
        headers.insert("Capsule".into(), STANDARD.encode(caps));
        headers.insert("Rkb".into(), STANDARD.encode(rk));

        if pkx_bytes.len() != 32 {
            return Err("invalid pkx length".into());
        }
        let pkx_pub = sr25519::Public::from_slice(pkx_bytes).map_err(|_| "invalid Pkx length")?;
        let pkx_ss58 = pkx_pub.to_ss58check_with_version(Ss58AddressFormat::custom(11330));
        headers.insert("Pkx".into(), pkx_ss58);
    }

    let bytes = send_http_request(Method::GET, &url, Some(headers), None).await?;
    std::fs::write(fpath, bytes)?;
    Ok(())
}

/// Retrieve capsule + gateway pubkey
pub async fn get_precapsule_and_pubkey(
    base_url: &str,
    fid: &str,
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let url = format!(
        "{}/{}/{}",
        base_url.trim_end_matches('/'),
        Endpoint::Capsule.path().trim_start_matches('/'),
        fid
    );
    let bytes = send_http_request(Method::GET, &url, None, None).await?;
    let resp: Response<HashMap<String, Vec<u8>>> = serde_json::from_slice(&bytes)?;
    let data = resp.data;
    Ok((data["capsule"].clone(), data["pubkey"].clone()))
}

/// Generate gateway access token
pub async fn gen_gateway_access_token(
    base_url: &str,
    message: &str,
    account: &str,
    sign: &[u8],
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", base_url, Endpoint::GenToken.path());

    let sign_hex = hex::encode(sign);

    let mut form = HashMap::new();
    form.insert("account", account);
    form.insert("message", message);
    form.insert("sign", &sign_hex);

    // encode as application/x-www-form-urlencoded
    let body = serde_urlencoded::to_string(&form)?;

    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".into(),
        "application/x-www-form-urlencoded".into(),
    );

    let resp_bytes =
        send_http_request(Method::POST, &url, Some(headers), Some(body.into_bytes())).await?;
    let resp: Response<String> = serde_json::from_slice(&resp_bytes)?;
    Ok(resp.data)
}

/// Upload file (sync or async)
pub async fn upload_file(opts: UploadFileOpts<'_>) -> Result<Vec<u8>, Box<dyn Error>> {
    let url = format!("{}{}", opts.base_url, Endpoint::UploadFile.path());

    let mut file = File::open(opts.file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let mut form = Form::new().text("territory", opts.territory.to_string());
    if opts.async_upload {
        form = form.text("async", "true".to_string());
    }
    if opts.no_proxy {
        form = form.text("noProxy", "true".to_string());
    }
    if opts.encrypt {
        form = form.text("encrypt", "true".to_string());
    }
    form = form.part(
        "file",
        Part::bytes(buf).file_name(opts.filename.to_string()),
    );

    let client = Client::new();
    let resp = client
        .post(&url)
        .header("token", format!("Bearer {}", opts.token))
        .multipart(form)
        .send()
        .await?;

    let bytes = resp.bytes().await?;
    Ok(bytes.to_vec())
}

pub async fn request_batch_upload(req: BatchUploadRequest<'_>) -> Result<String, Box<dyn Error>> {
    let info = BatchFilesInfo {
        file_name: Some(req.filename.to_string()),
        territory: Some(req.territory.to_string()),
        total_size: Some(req.file_size),
        encrypt: Some(req.encrypt),
        async_upload: Some(req.async_upload),
        no_tx_proxy: Some(req.no_tx_proxy),
        ..Default::default()
    };

    let body = serde_json::to_vec(&info)?;
    let mut headers = HashMap::new();
    headers.insert("Content-Type".into(), "application/json".into());
    headers.insert("token".into(), format!("Bearer {}", req.token));

    let url = format!("{}{}", req.base_url, Endpoint::BatchRequest.path());
    let resp_bytes = send_http_request(Method::POST, &url, Some(headers), Some(body)).await?;
    let resp: Response<String> = serde_json::from_slice(&resp_bytes)?;
    Ok(resp.data)
}

pub async fn batch_upload_file<R>(
    base_url: &str,
    token: &str,
    hash: &str,
    reader: &mut R,
    start: u64,
    end: u64,
) -> Result<BatchUploadResp, Box<dyn Error>>
where
    R: tokio::io::AsyncRead + tokio::io::AsyncSeek + Unpin,
{
    if end <= start {
        return Err("invalid byte range".into());
    }

    let size = (end - start) as usize;
    let mut buf = vec![0u8; size];
    reader.seek(std::io::SeekFrom::Start(start)).await?;
    reader.read_exact(&mut buf).await?;

    let form = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(buf).file_name("part".to_string()),
    );

    let client = reqwest::Client::new();
    let url = format!("{}{}", base_url, Endpoint::BatchUpload.path());

    let resp = client
        .post(&url)
        .header("token", format!("Bearer {}", token))
        .header("Range", format!("bytes={}-{}", start, end))
        .header("hash", hash)
        .multipart(form)
        .send()
        .await?;

    let resp_bytes = resp.bytes().await?;
    let resp: Response<BatchUploadResp> = serde_json::from_slice(&resp_bytes).map_err(|e| {
        format!(
            "failed to parse response: {e}, body: {}",
            String::from_utf8_lossy(&resp_bytes)
        )
    })?;

    Ok(resp.data)
}

pub fn wrap_message_for_signing(msg: &[u8]) -> Vec<u8> {
    // Step 1: compute SHA-256 hash of the message
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let hash = hasher.finalize();

    // Step 2: wrap the hash with <Bytes> and </Bytes>
    let mut wrapped = Vec::with_capacity(8 + hash.len() + 9); // "<Bytes>" + hash + "</Bytes>"
    wrapped.extend_from_slice(b"<Bytes>");
    wrapped.extend_from_slice(&hash);
    wrapped.extend_from_slice(b"</Bytes>");

    wrapped
}
