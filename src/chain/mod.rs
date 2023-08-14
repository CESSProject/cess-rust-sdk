pub mod audit;
pub mod deoss;
pub mod file_bank;
pub mod sminer;
pub mod storage_handler;

use sp_keyring::sr25519::sr25519::Pair;
use subxt::ext::sp_core::Pair as sp_core_pair;

pub struct Sdk {
    pair: Pair,
    name: String,
}

impl Sdk {
    pub fn new(mnemonic: &str, service_name: &str) -> Self {
        let pair =
            <sp_keyring::sr25519::sr25519::Pair as sp_core_pair>::from_string(mnemonic, None)
                .unwrap();
        Self {
            pair,
            name: service_name.to_string(),
        }
    }

    pub fn get_sdk_name(&self) -> &str {
        &self.name
    }

    pub fn set_sdk_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
