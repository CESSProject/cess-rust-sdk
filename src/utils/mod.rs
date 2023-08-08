pub fn hex_string_to_bytes(hex: &str) -> [u8; 64] {
    let hex_without_prefix = if hex.starts_with("0x") {
        &hex[2..]
    } else {
        hex
    };

    let bytes = hex::decode(hex_without_prefix).expect("Failed to decode hex string");
    let mut result = [0u8; 64];

    if bytes.len() != result.len() {
        panic!("Hex string does not have the expected length");
    }

    result.copy_from_slice(&bytes);
    result
}
