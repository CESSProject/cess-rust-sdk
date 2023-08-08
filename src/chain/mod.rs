mod file_bank;

use std::time::Duration;
#[derive(Default)]
pub struct Sdk {
    rpc_addr: Vec<String>,
    packing_time: Duration,
    token_symbol: String,
    network_env: String,
    signature_acc: String,
    name: String,
    enabled_p2p: bool,
}

impl Sdk {
    pub fn new(service_name: &str) -> Self {
        Self {
            name: service_name.to_string(),
            ..Default::default()
        }
    }

    pub fn get_sdk_name(&self) -> &str {
        &self.name
    }

    pub fn set_sdk_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_signature_acc(&self) -> &str {
        &self.signature_acc
    }

    pub fn get_network_env(&self) -> &str {
        &self.network_env
    }

    pub fn is_p2p_enabled(&self) -> bool {
        self.enabled_p2p
    }
}
