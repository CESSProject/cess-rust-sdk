use std::{
    fmt::{Display, Formatter, Result},
    path::PathBuf,
    time::Duration,
};

// DOT is "." character
const DOT: &str = ".";

// Unit precision of CESS token
pub const TOKEN_PRECISION_CESS: &str = "000000000000";

pub const BLOCK_INTERVAL: Duration = Duration::from_secs(6);

pub const MAX_SUBMITED_IDLE_FILE_META: usize = 30;

pub const PUBLIC_DEOSS: &str = "https://deoss-pub-gateway.cess.cloud/";
pub const PUBLIC_DEOSS_ACCOUNT: &str = "cXhwBytXqrZLr1qM5NHJhCzEMckSTzNKw17ci2aHft6ETSQm9";

pub enum Pallets {
    Audit,
    Oss,
    FileBank,
    TeeWorker,
    SMiner,
    StorageHandler,
    System,
}

impl Display for Pallets {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Pallets::Audit => write!(f, "Audit"),
            Pallets::Oss => write!(f, "Oss"),
            Pallets::FileBank => write!(f, "FileBank"),
            Pallets::TeeWorker => write!(f, "TeeWorker"),
            Pallets::SMiner => write!(f, "Sminer"),
            Pallets::StorageHandler => write!(f, "StorageHandler"),
            Pallets::System => write!(f, "System"),
        }
    }
}

pub enum ChainState {
    // AUDIT
    UnverifyProof,
    ChallengeDuration,
    ChallengeSnapshot,

    // OSS
    AuthorityList,

    // S_MINER
    AllMiner,
    MinerItems,
    RewardMap,
    Expanders,

    // TEE_WORKER
    TeeWorkerMap,
    TeeProd2Pk,

    // FILE_BANK
    File,
    Bucket,
    BucketList,
    DealMap,
    FillerMap,
    PendingReplace,
    RestoralOrder,
    RestoralTargetInfo,

    // STORAGE_HANDLER
    UserSpaceInfo,
    UnitPrice,

    // NET_SNAPSHOT
    // CHALLENGE_SNAPSHOT, // TODO: Check if this is in AUDIT and NET_SNAPSHOT both.

    // SYSTEM
    Account,
    Events,
}

impl Display for ChainState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ChainState::UnverifyProof => write!(f, "UnverifyProof"),
            ChainState::ChallengeDuration => write!(f, "ChallengeDuration"),
            ChainState::ChallengeSnapshot => write!(f, "ChallengeSnapShot"),
            ChainState::AuthorityList => write!(f, "AuthorityList"),
            ChainState::AllMiner => write!(f, "AllMiner"),
            ChainState::MinerItems => write!(f, "MinerItems"),
            ChainState::RewardMap => write!(f, "RewardMap"),
            ChainState::Expanders => write!(f, "Expenders"),
            ChainState::TeeWorkerMap => write!(f, "TeeWorkerMap"),
            ChainState::TeeProd2Pk => write!(f, "TeePodr2Pk"),
            ChainState::File => write!(f, "File"),
            ChainState::Bucket => write!(f, "Bucket"),
            ChainState::BucketList => write!(f, "UserBucketList"),
            ChainState::DealMap => write!(f, "DealMap"),
            ChainState::FillerMap => write!(f, "FillerMap"),
            ChainState::PendingReplace => write!(f, "PendingReplacements"),
            ChainState::RestoralOrder => write!(f, "RestoralOrder"),
            ChainState::RestoralTargetInfo => write!(f, "RestoralTarget"),
            ChainState::UserSpaceInfo => write!(f, "UserOwnedSpace"),
            ChainState::UnitPrice => write!(f, "UnitPrice"),
            ChainState::Account => write!(f, "Account"),
            ChainState::Events => write!(f, "Events"),
        }
    }
}

pub enum Extrinsics {
    // AUDIT
    TxAuditSubmitProof,
    TxAuditSubmitIdleProof,
    TxAuditSubmitServiceProof,

    // OSS
    TxOssRegister,
    TxOssUpdate,
    TxOssDestroy,
    TxOssAuthorize,
    TxOssUnauthorize,

    // S_MINER
    TxSMinerRegister,
    TxSMinerIncreaseStakes,
    TxSMinerUpdatePeerId,
    TxSMinerUpdateIncome,
    TxSMinerclaimReward,

    // FILE_BANK
    TxFileBankPutBucket,
    TxFileBankDelBucket,
    TxFileBankDelFile,
    TxFileBankDelFiller,
    TxFileBankUploadDec,
    TxFileBankUploadFiller,
    TxFileBankFileReport,
    TxFileBankReplaceFile,
    TxFileBankMinerExitPrep,
    TxFileBankWithdraw,
    TxFileBankGenRestoreOrder,
    TxFileBankClaimRestoreOrder,
    TxFileBankClaimNoExistOrder,
    TxFileBankRestoralComplete,
    TxFileBankCertIdleSpace,
    TxFileBankReplaceIdleSpace,

    // STORAGE_HANDLER
    TxStorageBuySpace,
    TxStorageExpansionSpace,
    TxStorageRenewalSpace,
}

impl Display for Extrinsics {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Extrinsics::TxAuditSubmitProof => {
                write!(f, "{}submit_proof", extrinsics_default_prefix(self))
            }
            Extrinsics::TxAuditSubmitIdleProof => write!(
                f,
                "{}submit_idle_proof",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxAuditSubmitServiceProof => write!(
                f,
                "{}submit_service_proof",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxOssRegister => {
                write!(f, "{}register", extrinsics_default_prefix(self))
            }
            Extrinsics::TxOssUpdate => {
                write!(f, "{}update", extrinsics_default_prefix(self))
            }
            Extrinsics::TxOssDestroy => {
                write!(f, "{}destroy", extrinsics_default_prefix(self))
            }
            Extrinsics::TxOssAuthorize => {
                write!(f, "{}authorize", extrinsics_default_prefix(self))
            }
            Extrinsics::TxOssUnauthorize => write!(
                f,
                "{}cancel_authorize",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxSMinerRegister => {
                write!(f, "{}regnstk", extrinsics_default_prefix(self))
            }
            Extrinsics::TxSMinerIncreaseStakes => write!(
                f,
                "{}increase_collateral",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxSMinerUpdatePeerId => {
                write!(f, "{}update_peer_id", extrinsics_default_prefix(self))
            }
            Extrinsics::TxSMinerUpdateIncome => write!(
                f,
                "{}update_beneficiary",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxSMinerclaimReward => {
                write!(f, "{}receive_reward", extrinsics_default_prefix(self))
            }
            Extrinsics::TxFileBankPutBucket => {
                write!(f, "{}create_bucket", extrinsics_default_prefix(self))
            }
            Extrinsics::TxFileBankDelBucket => {
                write!(f, "{}delete_bucket", extrinsics_default_prefix(self))
            }
            Extrinsics::TxFileBankDelFile => {
                write!(f, "{}delete_file", extrinsics_default_prefix(self))
            }
            Extrinsics::TxFileBankDelFiller => {
                write!(f, "{}delete_filler", extrinsics_default_prefix(self))
            }
            Extrinsics::TxFileBankUploadDec => write!(
                f,
                "{}upload_declaration",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankUploadFiller => {
                write!(f, "{}upload_filler", extrinsics_default_prefix(self))
            }
            Extrinsics::TxFileBankFileReport => write!(
                f,
                "{}transfer_report",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankReplaceFile => write!(
                f,
                "{}replace_file_report",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankMinerExitPrep => write!(
                f,
                "{}miner_exit_prep",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankWithdraw => {
                write!(f, "{}miner_withdraw", extrinsics_default_prefix(self))
            }
            Extrinsics::TxFileBankGenRestoreOrder => write!(
                f,
                "{}generate_restoral_order",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankClaimRestoreOrder => write!(
                f,
                "{}claim_restoral_order",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankClaimNoExistOrder => write!(
                f,
                "{}claim_restoral_noexist_order",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankRestoralComplete => write!(
                f,
                "{}restoral_order_complete",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankCertIdleSpace => write!(
                f,
                "{}cert_idle_space",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxFileBankReplaceIdleSpace => write!(
                f,
                "{}replace_idle_space",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxStorageBuySpace => {
                write!(f, "{}buy_space", extrinsics_default_prefix(self))
            }
            Extrinsics::TxStorageExpansionSpace => write!(
                f,
                "{}expansion_space",
                extrinsics_default_prefix(self)
            ),
            Extrinsics::TxStorageRenewalSpace => {
                write!(f, "{}renewal_space", extrinsics_default_prefix(self))
            }
        }
    }
}

fn extrinsics_default_prefix(extrinsics: &Extrinsics) -> String {
    match extrinsics {
        Extrinsics::TxAuditSubmitProof
        | Extrinsics::TxAuditSubmitIdleProof
        | Extrinsics::TxAuditSubmitServiceProof => {
            format!("{}{}", Pallets::Audit, DOT)
        }
        Extrinsics::TxOssRegister
        | Extrinsics::TxOssUpdate
        | Extrinsics::TxOssDestroy
        | Extrinsics::TxOssAuthorize
        | Extrinsics::TxOssUnauthorize => format!("{}{}", Pallets::Oss, DOT),
        Extrinsics::TxSMinerRegister
        | Extrinsics::TxSMinerIncreaseStakes
        | Extrinsics::TxSMinerUpdatePeerId
        | Extrinsics::TxSMinerUpdateIncome
        | Extrinsics::TxSMinerclaimReward => format!("{}{}", Pallets::SMiner, DOT),
        Extrinsics::TxFileBankPutBucket
        | Extrinsics::TxFileBankDelBucket
        | Extrinsics::TxFileBankDelFile
        | Extrinsics::TxFileBankDelFiller
        | Extrinsics::TxFileBankUploadDec
        | Extrinsics::TxFileBankUploadFiller
        | Extrinsics::TxFileBankFileReport
        | Extrinsics::TxFileBankReplaceFile
        | Extrinsics::TxFileBankMinerExitPrep
        | Extrinsics::TxFileBankWithdraw
        | Extrinsics::TxFileBankGenRestoreOrder
        | Extrinsics::TxFileBankClaimRestoreOrder
        | Extrinsics::TxFileBankClaimNoExistOrder
        | Extrinsics::TxFileBankRestoralComplete
        | Extrinsics::TxFileBankCertIdleSpace
        | Extrinsics::TxFileBankReplaceIdleSpace => {
            format!("{}{}", Pallets::FileBank, DOT)
        }
        Extrinsics::TxStorageBuySpace
        | Extrinsics::TxStorageExpansionSpace
        | Extrinsics::TxStorageRenewalSpace => {
            format!("{}{}", Pallets::StorageHandler, DOT)
        }
    }
}

pub enum RPC {
    // SYSTEM
    RpcSysProperties,
    RpcSysSyncState,
    RpcSysVersion,
    RpcSysChain,

    // NET
    RpcNetListening,
}

impl Display for RPC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RPC::RpcSysProperties => write!(f, "system_properties"),
            RPC::RpcSysSyncState => write!(f, "system_syncState"),
            RPC::RpcSysVersion => write!(f, "system_version"),
            RPC::RpcSysChain => write!(f, "system_chain"),
            RPC::RpcNetListening => write!(f, "net_listening"),
        }
    }
}

pub enum ServiceName {
    DeOss,
    SMiner,
    SDK,
}

impl Display for ServiceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ServiceName::DeOss => write!(f, "deoss"),
            ServiceName::SMiner => write!(f, "bucket"),
            ServiceName::SDK => write!(f, "client"),
        }
    }
}

pub enum MinerState {
    Positive,
    Frozen,
    Exit,
    Lock,
}

impl Display for MinerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MinerState::Positive => write!(f, "positive"),
            MinerState::Frozen => write!(f, "frozen"),
            MinerState::Exit => write!(f, "exit"),
            MinerState::Lock => write!(f, "lock"),
        }
    }
}

pub enum Error {
    Failed,
    Timeout,
    Empty,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::Failed => write!(f, "failed"),
            Error::Timeout => write!(f, "timeout"),
            Error::Empty => write!(f, "empty"),
        }
    }
}

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

#[cfg(test)]
mod test {
    use super::{ChainState, Extrinsics, Pallets, RPC};

    #[test]
    fn test_pallets_to_string() {
        let audit = Pallets::Audit;
        assert_eq!(audit.to_string(), "Audit".to_string());

        let oss = Pallets::Oss;
        assert_eq!(oss.to_string(), "Oss".to_string());
    }

    #[test]
    fn test_chain_state_to_string() {
        let unverify_proof = ChainState::UnverifyProof;
        assert_eq!(unverify_proof.to_string(), "UnverifyProof".to_string());

        let user_space_info = ChainState::UserSpaceInfo;
        assert_eq!(user_space_info.to_string(), "UserOwnedSpace".to_string());
    }

    #[test]
    fn test_extrinsics_to_string() {
        let tx_audit_submit_proof = Extrinsics::TxAuditSubmitProof;
        assert_eq!(tx_audit_submit_proof.to_string(), "Audit.submit_proof");

        let tx_oss_register = Extrinsics::TxOssRegister;
        assert_eq!(tx_oss_register.to_string(), "Oss.register");

        let tx_s_miner_register = Extrinsics::TxSMinerRegister;
        assert_eq!(tx_s_miner_register.to_string(), "Sminer.regnstk");

        let tx_file_bank_put_bucket = Extrinsics::TxFileBankPutBucket;
        assert_eq!(
            tx_file_bank_put_bucket.to_string(),
            "FileBank.create_bucket"
        );

        let tx_storage_buy_space = Extrinsics::TxStorageBuySpace;
        assert_eq!(tx_storage_buy_space.to_string(), "StorageHandler.buy_space");
    }

    #[test]
    fn test_rpc_to_string() {
        let rpc_sys_properties = RPC::RpcSysProperties;
        assert_eq!(rpc_sys_properties.to_string(), "system_properties");
    }
}
