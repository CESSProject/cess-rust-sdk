use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UploadResponse {
    pub fid: String,
}
