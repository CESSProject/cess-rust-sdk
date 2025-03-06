use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub fid: String,
}

#[derive(Deserialize, Debug)]
pub struct UploadResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<Data>,
}
