use std::sync::Mutex;

use lazy_static::lazy_static;

pub const URL: &str = "wss://testnet-rpc0.cess.cloud:443/ws/";
pub const PUBLIC_DEOSS: &str = "http://deoss-pub-gateway.cess.cloud/";
pub const PUBLIC_DEOSS_ACCOUNT: &str = "cXhwBytXqrZLr1qM5NHJhCzEMckSTzNKw17ci2aHft6ETSQm9";

lazy_static! {
    static ref CUSTOM_URL: Mutex<Option<String>> = Mutex::new(None);
    static ref CUSTOM_DEOSS_URL: Mutex<Option<String>> = Mutex::new(None);
    static ref CUSTOM_DEOSS_ACCOUNT: Mutex<Option<String>> = Mutex::new(None);
}

pub fn get_custom_url() -> Option<String> {
    CUSTOM_URL.lock().unwrap().clone()
}

pub fn set_custom_url(new_value: Option<String>) {
    let mut data = CUSTOM_URL.lock().unwrap();
    *data = new_value;
}

pub fn get_custom_deoss_url() -> Option<String> {
    CUSTOM_DEOSS_URL.lock().unwrap().clone()
}

pub fn set_custom_deoss_url(new_value: Option<String>) {
    let mut data = CUSTOM_DEOSS_URL.lock().unwrap();
    *data = new_value;
}

pub fn get_custom_deoss_account() -> Option<String> {
    CUSTOM_DEOSS_ACCOUNT.lock().unwrap().clone()
}

pub fn set_custom_deoss_account(new_value: Option<String>) {
    let mut data = CUSTOM_DEOSS_ACCOUNT.lock().unwrap();
    *data = new_value;
}