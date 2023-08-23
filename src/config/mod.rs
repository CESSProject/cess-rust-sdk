use std::sync::Mutex;

use lazy_static::lazy_static;

const URL: &str = "wss://testnet-rpc0.cess.cloud:443/ws/";
const PUBLIC_DEOSS: &str = "http://deoss-pub-gateway.cess.cloud/";
const PUBLIC_DEOSS_ACCOUNT: &str = "cXhwBytXqrZLr1qM5NHJhCzEMckSTzNKw17ci2aHft6ETSQm9";

lazy_static! {
    static ref CUSTOM_URL: Mutex<Option<String>> = Mutex::new(None);
    static ref CUSTOM_DEOSS_URL: Mutex<Option<String>> = Mutex::new(None);
    static ref CUSTOM_DEOSS_ACCOUNT: Mutex<Option<String>> = Mutex::new(None);
}

pub fn get_url() -> String {
    if let Some(url) = get_custom_url() {
        url
    } else {
        URL.to_string()
    }
}

pub fn get_deoss_url() -> String {
    if let Some(custom_deoss_url) = get_custom_deoss_url() {
        custom_deoss_url
    } else {
        PUBLIC_DEOSS.to_string()
    }
}

pub fn get_deoss_account() -> String {
    if let Some(custom_deoss_account) = get_custom_deoss_account() {
        custom_deoss_account
    } else {
        PUBLIC_DEOSS_ACCOUNT.to_string()
    }
}

fn get_custom_url() -> Option<String> {
    CUSTOM_URL.lock().unwrap().clone()
}

pub fn set_custom_url(new_value: Option<String>) {
    let mut data = CUSTOM_URL.lock().unwrap();
    *data = new_value;
}

fn get_custom_deoss_url() -> Option<String> {
    CUSTOM_DEOSS_URL.lock().unwrap().clone()
}

pub fn set_custom_deoss_url(new_value: Option<String>) {
    let mut data = CUSTOM_DEOSS_URL.lock().unwrap();
    *data = new_value;
}

fn get_custom_deoss_account() -> Option<String> {
    CUSTOM_DEOSS_ACCOUNT.lock().unwrap().clone()
}

pub fn set_custom_deoss_account(new_value: Option<String>) {
    let mut data = CUSTOM_DEOSS_ACCOUNT.lock().unwrap();
    *data = new_value;
}
