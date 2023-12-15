use aes::cipher::{
    block_padding::Pkcs7, generic_array::GenericArray, BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};

use anyhow::Result;
use std::{error::Error, fmt};

// AES key and IV lengths
const AES_KEY_LENGTH: usize = 32;

#[derive(Debug)]
pub enum CryptoError {
    KeyLengthExceeded,
    KeyLengthEmpty,
    PaddingSizeError,
    InvalidBlockSize,
    InvalidPKCS7Data,
    InvalidPKCS7Padding,
    OtherError(String),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::KeyLengthExceeded => write!(f, "Key length cannot exceed 32 bytes"),
            CryptoError::KeyLengthEmpty => write!(f, "Key length cannot be empty"),
            CryptoError::PaddingSizeError => write!(f, "Padding size error"),
            CryptoError::InvalidBlockSize => write!(f, "Invalid block size"),
            CryptoError::InvalidPKCS7Data => write!(f, "Invalid PKCS7 data (empty or not padded)"),
            CryptoError::InvalidPKCS7Padding => write!(f, "Invalid padding on input"),
            CryptoError::OtherError(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for CryptoError {}

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub fn aes_cbc_encrypt(plain_text: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if secret_key.len() > AES_KEY_LENGTH {
        return Err(CryptoError::KeyLengthExceeded);
    }

    if secret_key.is_empty() {
        return Err(CryptoError::KeyLengthEmpty);
    }

    let pt_len = plain_text.len();
    let block_size = 16;
    let padding_length = block_size - (pt_len % block_size);
    let padded_length = pt_len + padding_length;

    let key = GenericArray::from_slice(&secret_key);
    let iv = GenericArray::from_slice(&key.as_slice()[..block_size]);
    let mut buf = vec![0u8; padded_length];
    buf[..pt_len].copy_from_slice(&plain_text);
    let ct = Aes256CbcEnc::new(key, &iv)
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();

    Ok(ct.to_vec())
}

pub fn aes_cbc_decrypt(cipher_text: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if secret_key.len() > 32 {
        return Err(CryptoError::KeyLengthExceeded);
    }

    if secret_key.is_empty() {
        return Err(CryptoError::KeyLengthEmpty);
    }

    let key = GenericArray::from_slice(&secret_key);
    let iv = GenericArray::from_slice(&key.as_slice()[..16]);

    let mut ct = cipher_text.to_vec();

    let plain_text = Aes256CbcDec::new(&key, &iv)
        .decrypt_padded_mut::<Pkcs7>(&mut ct)
        .unwrap();

    Ok(plain_text.to_vec())
}
