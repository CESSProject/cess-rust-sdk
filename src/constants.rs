use std::time::Duration;

// Unit precision of CESS token
pub const TOKEN_PRECISION_CESS: &str = "000000000000";

pub const BLOCK_INTERVAL: Duration = Duration::from_secs(6);

pub const MAX_SUBMITED_IDLE_FILE_META: usize = 30;

pub const MIN_BUCKET_NAME_LENGTH: usize = 3;
pub const MAX_BUCKET_NAME_LENGHT: usize = 63;

pub const SIZE_1_KI_B: u32 = 1024;
pub const SIZE_1_MI_B: u32 = 1024 * SIZE_1_KI_B;
pub const SIZE_1_GI_B: u32 = 1024 * SIZE_1_MI_B;

pub const SEGMENT_SIZE: u32 = 64 * SIZE_1_MI_B;
pub const FRAEMENT_SIZE: u32 = 16 * SIZE_1_MI_B;
pub const BLOCK_NUMBER: u32 = 1024;
pub const DATA_SHARDS: u32 = 4;
pub const PAR_SHARDS: u32 = 2;
