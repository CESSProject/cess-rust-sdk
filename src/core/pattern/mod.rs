use std::{
    fmt::{Display, Formatter, Result},
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
            Pallets::Audit => write!(f, "{}", "Audit"),
            Pallets::Oss => write!(f, "{}", "Oss"),
            Pallets::FileBank => write!(f, "{}", "FileBank"),
            Pallets::TeeWorker => write!(f, "{}", "TeeWorker"),
            Pallets::SMiner => write!(f, "{}", "Sminer"),
            Pallets::StorageHandler => write!(f, "{}", "StorageHandler"),
            Pallets::System => write!(f, "{}", "System"),
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
            ChainState::UnverifyProof => write!(f, "{}", "UnverifyProof"),
            ChainState::ChallengeDuration => write!(f, "{}", "ChallengeDuration"),
            ChainState::ChallengeSnapshot => write!(f, "{}", "ChallengeSnapShot"),
            ChainState::AuthorityList => write!(f, "{}", "AuthorityList"),
            ChainState::AllMiner => write!(f, "{}", "AllMiner"),
            ChainState::MinerItems => write!(f, "{}", "MinerItems"),
            ChainState::RewardMap => write!(f, "{}", "RewardMap"),
            ChainState::Expanders => write!(f, "{}", "Expenders"),
            ChainState::TeeWorkerMap => write!(f, "{}", "TeeWorkerMap"),
            ChainState::TeeProd2Pk => write!(f, "{}", "TeePodr2Pk"),
            ChainState::File => write!(f, "{}", "File"),
            ChainState::Bucket => write!(f, "{}", "Bucket"),
            ChainState::BucketList => write!(f, "{}", "UserBucketList"),
            ChainState::DealMap => write!(f, "{}", "DealMap"),
            ChainState::FillerMap => write!(f, "{}", "FillerMap"),
            ChainState::PendingReplace => write!(f, "{}", "PendingReplacements"),
            ChainState::RestoralOrder => write!(f, "{}", "RestoralOrder"),
            ChainState::RestoralTargetInfo => write!(f, "{}", "RestoralTarget"),
            ChainState::UserSpaceInfo => write!(f, "{}", "UserOwnedSpace"),
            ChainState::UnitPrice => write!(f, "{}", "UnitPrice"),
            ChainState::Account => write!(f, "{}", "Account"),
            ChainState::Events => write!(f, "{}", "Events"),
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
                write!(f, "{}{}", extrinsics_default_prefix(self), "submit_proof")
            }
            Extrinsics::TxAuditSubmitIdleProof => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "submit_idle_proof"
            ),
            Extrinsics::TxAuditSubmitServiceProof => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "submit_service_proof"
            ),
            Extrinsics::TxOssRegister => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "register")
            }
            Extrinsics::TxOssUpdate => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "update")
            }
            Extrinsics::TxOssDestroy => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "destroy")
            }
            Extrinsics::TxOssAuthorize => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "authorize")
            }
            Extrinsics::TxOssUnauthorize => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "cancel_authorize"
            ),
            Extrinsics::TxSMinerRegister => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "regnstk")
            }
            Extrinsics::TxSMinerIncreaseStakes => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "increase_collateral"
            ),
            Extrinsics::TxSMinerUpdatePeerId => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "update_peer_id")
            }
            Extrinsics::TxSMinerUpdateIncome => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "update_beneficiary"
            ),
            Extrinsics::TxSMinerclaimReward => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "receive_reward")
            }
            Extrinsics::TxFileBankPutBucket => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "create_bucket")
            }
            Extrinsics::TxFileBankDelBucket => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "delete_bucket")
            }
            Extrinsics::TxFileBankDelFile => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "delete_file")
            }
            Extrinsics::TxFileBankDelFiller => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "delete_filler")
            }
            Extrinsics::TxFileBankUploadDec => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "upload_declaration"
            ),
            Extrinsics::TxFileBankUploadFiller => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "upload_filler")
            }
            Extrinsics::TxFileBankFileReport => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "transfer_report"
            ),
            Extrinsics::TxFileBankReplaceFile => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "replace_file_report"
            ),
            Extrinsics::TxFileBankMinerExitPrep => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "miner_exit_prep"
            ),
            Extrinsics::TxFileBankWithdraw => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "miner_withdraw")
            }
            Extrinsics::TxFileBankGenRestoreOrder => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "generate_restoral_order"
            ),
            Extrinsics::TxFileBankClaimRestoreOrder => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "claim_restoral_order"
            ),
            Extrinsics::TxFileBankClaimNoExistOrder => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "claim_restoral_noexist_order"
            ),
            Extrinsics::TxFileBankRestoralComplete => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "restoral_order_complete"
            ),
            Extrinsics::TxFileBankCertIdleSpace => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "cert_idle_space"
            ),
            Extrinsics::TxFileBankReplaceIdleSpace => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "replace_idle_space"
            ),
            Extrinsics::TxStorageBuySpace => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "buy_space")
            }
            Extrinsics::TxStorageExpansionSpace => write!(
                f,
                "{}{}",
                extrinsics_default_prefix(self),
                "expansion_space"
            ),
            Extrinsics::TxStorageRenewalSpace => {
                write!(f, "{}{}", extrinsics_default_prefix(self), "renewal_space")
            }
        }
    }
}

fn extrinsics_default_prefix(extrinsics: &Extrinsics) -> String {
    match extrinsics {
        Extrinsics::TxAuditSubmitProof
        | Extrinsics::TxAuditSubmitIdleProof
        | Extrinsics::TxAuditSubmitServiceProof => {
            format!("{}{}", Pallets::Audit.to_string(), DOT)
        }
        Extrinsics::TxOssRegister
        | Extrinsics::TxOssUpdate
        | Extrinsics::TxOssDestroy
        | Extrinsics::TxOssAuthorize
        | Extrinsics::TxOssUnauthorize => format!("{}{}", Pallets::Oss.to_string(), DOT),
        Extrinsics::TxSMinerRegister
        | Extrinsics::TxSMinerIncreaseStakes
        | Extrinsics::TxSMinerUpdatePeerId
        | Extrinsics::TxSMinerUpdateIncome
        | Extrinsics::TxSMinerclaimReward => format!("{}{}", Pallets::SMiner.to_string(), DOT),
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
            format!("{}{}", Pallets::FileBank.to_string(), DOT)
        }
        Extrinsics::TxStorageBuySpace
        | Extrinsics::TxStorageExpansionSpace
        | Extrinsics::TxStorageRenewalSpace => {
            format!("{}{}", Pallets::StorageHandler.to_string(), DOT)
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
            RPC::RpcSysProperties => write!(f, "{}", "system_properties"),
            RPC::RpcSysSyncState => write!(f, "{}", "system_syncState"),
            RPC::RpcSysVersion => write!(f, "{}", "system_version"),
            RPC::RpcSysChain => write!(f, "{}", "system_chain"),
            RPC::RpcNetListening => write!(f, "{}", "net_listening"),
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
            ServiceName::DeOss => write!(f, "{}", "deoss"),
            ServiceName::SMiner => write!(f, "{}", "bucket"),
            ServiceName::SDK => write!(f, "{}", "client"),
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
            MinerState::Positive => write!(f, "{}", "positive"),
            MinerState::Frozen => write!(f, "{}", "frozen"),
            MinerState::Exit => write!(f, "{}", "exit"),
            MinerState::Lock => write!(f, "{}", "lock"),
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
            Error::Failed => write!(f, "{}", "failed"),
            Error::Timeout => write!(f, "{}", "timeout"),
            Error::Empty => write!(f, "{}", "empty"),
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
