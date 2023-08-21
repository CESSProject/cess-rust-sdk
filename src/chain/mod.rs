pub mod audit;
pub mod deoss;
pub mod file;
pub mod file_bank;
pub mod sminer;
pub mod snapshot;
pub mod storage_handler;
pub mod tee_worker;

use sp_keyring::sr25519::sr25519::Pair;
use subxt::ext::sp_core::Pair as sp_core_pair;

use crate::core::utils::account::encode_public_key_as_cess_account;

pub struct Sdk {
    pair: Pair,
    name: String,
    signature_acc: String,
    enabled_p2p: bool,
}

impl Sdk {
    pub fn new(mnemonic: &str, service_name: &str, enabled_p2p: bool) -> Self {
        let pair =
            <sp_keyring::sr25519::sr25519::Pair as sp_core_pair>::from_string(mnemonic, None)
                .unwrap();
        Self {
            pair: pair.clone(),
            name: service_name.to_string(),
            signature_acc: encode_public_key_as_cess_account(&pair.public().0.clone()).unwrap(),
            enabled_p2p,
        }
    }

    pub fn get_sdk_name(&self) -> &str {
        &self.name
    }

    pub fn set_sdk_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_signature_acc(&self) -> String {
        self.signature_acc.clone()
    }

    pub fn set_enabled_p2p(&mut self, set: bool) {
        self.enabled_p2p = set;
    }

    pub fn get_enabled_p2p(&self) -> bool {
        self.enabled_p2p
    }
}
