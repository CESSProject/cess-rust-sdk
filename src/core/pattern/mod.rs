use std::{path::PathBuf, time::Duration};

// Unit precision of CESS token
pub const TOKEN_PRECISION_CESS: &str = "000000000000";

pub const BLOCK_INTERVAL: Duration = Duration::from_secs(6);

pub const MAX_SUBMITED_IDLE_FILE_META: usize = 30;

pub const URL: &str = "wss://testnet-rpc0.cess.cloud:443/ws/";
pub const PUBLIC_DEOSS: &str = "http://deoss-pub-gateway.cess.cloud/";
pub const PUBLIC_DEOSS_ACCOUNT: &str = "cXhwBytXqrZLr1qM5NHJhCzEMckSTzNKw17ci2aHft6ETSQm9";

pub const MIN_BUCKET_NAME_LENGTH: usize = 3;
pub const MAX_BUCKET_NAME_LENGHT: usize = 63;

pub const SIZE_1_KI_B: u32 = 1024;
pub const SIZE_1_MI_B: u32 = 1024 * SIZE_1_KI_B;
pub const SIZE_1_GI_B: u32 = 1024 * SIZE_1_MI_B;

pub const SEGMENT_SIZE: u32 = 16 * SIZE_1_MI_B;
pub const FRAEMENT_SIZE: u32 = 8 * SIZE_1_MI_B;
pub const BLOCK_NUMBER: u32 = 1024;
pub const DATA_SHARDS: u32 = 2;
pub const PAR_SHARDS: u32 = 1;

#[derive(Default)]
pub struct ChallengeInfo {
    pub random: Vec<Vec<u8>>,
    pub random_index_list: Vec<u32>,
    pub start: u32,
}

#[derive(Default)]
pub struct SegmentDataInfo {
    pub segment_hash: PathBuf,
    pub fragment_hash: Vec<PathBuf>,
}
