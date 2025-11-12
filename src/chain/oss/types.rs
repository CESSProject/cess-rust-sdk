#[derive(Debug)]
pub struct OssData {
    pub domain: String,
    pub peer_id: String,
}

#[derive(Debug)]
pub struct Oss {
    pub account: String,
    pub data: OssData,
}
