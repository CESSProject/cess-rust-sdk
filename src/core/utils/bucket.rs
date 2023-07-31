use regex::Regex;

use crate::core::pattern::{MAX_BUCKET_NAME_LENGHT, MIN_BUCKET_NAME_LENGTH};

use super::ip::is_valid_ip;

fn check_bucket_name(name: &str) -> bool {
    if name.len() < MIN_BUCKET_NAME_LENGTH || name.len() > MAX_BUCKET_NAME_LENGHT {
        return false;
    }

    let re = Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap();
    if !re.is_match(name) {
        return false;
    }

    if name.contains("..") {
        return false;
    }

    if name.starts_with('.')
        || name.starts_with('-')
        || name.starts_with('_')
        || name.ends_with('.')
        || name.ends_with('-')
        || name.ends_with('_')
    {
        return false;
    }

    if is_valid_ip(name) {
        return false;
    }

    true
}
