use base64::{engine::general_purpose, Engine as _};
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use schnorrkel::{ExpansionMode, Keypair, MiniSecretKey, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::convert::TryInto;
use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RetrieverError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("base64 error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("invalid length: {0}")]
    BadLen(String),

    #[error("crypto error: {0}")]
    Crypto(String),

    #[error("signature error: {0}")]
    Sig(String),

    #[error("{0}")]
    Other(String),
}

/// In-memory capsule with RistrettoPoint/Scalar
pub struct Capsule {
    pub e: RistrettoPoint,
    pub v: RistrettoPoint,
    pub s: Scalar,
}

/// JSON-serializable capsule: base64 strings for compressed points and scalar bytes
#[derive(Serialize, Deserialize)]
struct SerializableCapsule {
    e: String,
    v: String,
    s: String,
}

impl Capsule {
    fn to_serializable(&self) -> SerializableCapsule {
        let e_c = self.e.compress();
        let v_c = self.v.compress();
        let s_bytes = self.s.to_bytes();

        SerializableCapsule {
            e: general_purpose::STANDARD.encode(e_c.as_bytes()),
            v: general_purpose::STANDARD.encode(v_c.as_bytes()),
            s: general_purpose::STANDARD.encode(s_bytes),
        }
    }

    fn from_serializable(sc: &SerializableCapsule) -> Result<Self, RetrieverError> {
        let e_b = general_purpose::STANDARD.decode(&sc.e)?;
        let v_b = general_purpose::STANDARD.decode(&sc.v)?;
        let s_b = general_purpose::STANDARD.decode(&sc.s)?;

        if e_b.len() != 32 || v_b.len() != 32 || s_b.len() != 32 {
            return Err(RetrieverError::BadLen(
                "serialized capsule fields must be 32 bytes".into(),
            ));
        }

        let e_comp = CompressedRistretto::from_slice(&e_b)
            .map_err(|_| RetrieverError::Crypto("invalid slice length for e".into()))?;
        let v_comp = CompressedRistretto::from_slice(&v_b)
            .map_err(|_| RetrieverError::Crypto("invalid slice length for v".into()))?;

        let e = e_comp
            .decompress()
            .ok_or_else(|| RetrieverError::Crypto("invalid compressed point e".into()))?;
        let v = v_comp
            .decompress()
            .ok_or_else(|| RetrieverError::Crypto("invalid compressed point v".into()))?;

        let s_arr: [u8; 32] = s_b
            .as_slice()
            .try_into()
            .map_err(|_| RetrieverError::BadLen("scalar must be 32 bytes".into()))?;
        let s = Scalar::from_bytes_mod_order(s_arr);

        Ok(Capsule { e, v, s })
    }
}

/// Re-encrypt key: input capsule JSON and rk bytes (32 bytes scalar or hex/base64 text of 32 bytes).
/// Returns new capsule JSON bytes.
pub fn re_encrypt_key(capsule_json: &[u8], rk_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let sc: SerializableCapsule = serde_json::from_slice(capsule_json)?;
    let capsule = Capsule::from_serializable(&sc)?;

    let rk_scalar = parse_scalar_bytes(rk_bytes)?;

    let s_g = capsule.s * RISTRETTO_BASEPOINT_POINT;

    let mut hasher = Sha512::new();
    hasher.update(capsule.e.compress().as_bytes());
    hasher.update(capsule.v.compress().as_bytes());
    let h = Scalar::from_hash(hasher);

    let point = capsule.v + h * capsule.e;

    if point != s_g {
        return Err(RetrieverError::Crypto("re-encrypt key verification failed".into()).into());
    }

    let new_e = rk_scalar * capsule.e;
    let new_v = rk_scalar * capsule.v;
    let new_s = capsule.s;

    let new_capsule = Capsule {
        e: new_e,
        v: new_v,
        s: new_s,
    };

    let ssc = new_capsule.to_serializable();
    let out = serde_json::to_vec(&ssc)?;
    Ok(out)
}

/// Decrypt re-key: given mini/secret key bytes (ms_bytes), pk_x bytes (32), and re-encrypted capsule json.
/// Returns 32-byte AES key (SHA256(point))
///
/// ms_bytes: should be either 32 bytes (mini-secret seed) or 64 bytes (expanded SecretKey) depending on source.
pub fn decrypt_re_key(
    ms_bytes: &[u8],
    pk_x_bytes: &[u8],
    new_capsule_json: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
    let sc: SerializableCapsule = serde_json::from_slice(new_capsule_json)?;
    let new_capsule = Capsule::from_serializable(&sc)?;

    if pk_x_bytes.len() != 32 {
        return Err(RetrieverError::BadLen("pk_x must be 32 bytes".into()).into());
    }
    let pk_x = PublicKey::from_bytes(pk_x_bytes).map_err(|e| RetrieverError::Sig(e.to_string()))?;
    let pk_x_point = ristretto_point_from_pk(&pk_x);

    let secret_key: SecretKey = if ms_bytes.len() == 32 {
        let msk = MiniSecretKey::from_bytes(ms_bytes)
            .map_err(|_| RetrieverError::BadLen("invalid mini secret key bytes".into()))?;
        msk.expand_to_keypair(ExpansionMode::Ed25519).secret.clone()
    } else if ms_bytes.len() == 64 {
        SecretKey::from_bytes(ms_bytes)
            .map_err(|_| RetrieverError::BadLen("invalid SecretKey bytes".into()))?
    } else {
        return Err(RetrieverError::BadLen(
            "ms_bytes must be 32 (mini) or 64 (secret) bytes".into(),
        )
        .into());
    };

    let sk_b_scalar = Scalar::from_bytes_mod_order(secret_key.to_bytes()[0..32].try_into()?);

    let s_point = sk_b_scalar * pk_x_point;

    let pk_b = secret_key.to_public();
    let pk_b_bytes = pk_b.to_bytes();

    let mut d_hasher = Sha512::new();
    d_hasher.update(pk_x.to_bytes());
    d_hasher.update(pk_b_bytes);
    d_hasher.update(s_point.compress().as_bytes());
    let d = Scalar::from_hash(d_hasher);

    let sum = new_capsule.e + new_capsule.v;
    let point = d * sum;

    let mut sha = Sha256::new();
    sha.update(point.compress().as_bytes());
    let key = sha.finalize();

    Ok(key.to_vec())
}

/// Decrypt original capsule: given ms_bytes (mini/secret) and capsule JSON -> AES key (32 bytes)
pub fn decrypt_key(ms_bytes: &[u8], capsule_json: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let sc: SerializableCapsule = serde_json::from_slice(capsule_json)?;
    let capsule = Capsule::from_serializable(&sc)?;

    // parse secret key bytes as above
    let secret_key: SecretKey = if ms_bytes.len() == 32 {
        let msk = MiniSecretKey::from_bytes(ms_bytes)
            .map_err(|_| RetrieverError::BadLen("invalid mini secret key bytes".into()))?;
        msk.expand_to_keypair(ExpansionMode::Ed25519).secret.clone()
    } else if ms_bytes.len() == 64 {
        SecretKey::from_bytes(ms_bytes)
            .map_err(|_| RetrieverError::BadLen("invalid SecretKey bytes".into()))?
    } else {
        return Err(RetrieverError::BadLen(
            "ms_bytes must be 32 (mini) or 64 (secret) bytes".into(),
        )
        .into());
    };

    let sk_scalar = Scalar::from_bytes_mod_order(secret_key.to_bytes()[0..32].try_into()?);

    let sum = capsule.e + capsule.v;
    let point = sk_scalar * sum;

    let mut sha = Sha256::new();
    sha.update(point.compress().as_bytes());
    Ok(sha.finalize().to_vec())
}

/// Generate re-encryption key: inputs ms_bytes (mini/secret) and pk_b_bytes (32)
/// Returns (rk_bytes (32), pk_x_bytes (32))
pub fn gen_re_encryption_key(
    ms_bytes: &[u8],
    pk_b_bytes: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    if pk_b_bytes.len() != 32 {
        return Err(RetrieverError::BadLen("pk_b must be 32 bytes".into()).into());
    }
    let pk_b = PublicKey::from_bytes(pk_b_bytes).map_err(|e| RetrieverError::Sig(e.to_string()))?;

    let secret_a: SecretKey = if ms_bytes.len() == 32 {
        let msk = MiniSecretKey::from_bytes(ms_bytes)
            .map_err(|_| RetrieverError::BadLen("invalid mini secret key bytes".into()))?;
        msk.expand_to_keypair(ExpansionMode::Ed25519).secret.clone()
    } else if ms_bytes.len() == 64 {
        SecretKey::from_bytes(ms_bytes)
            .map_err(|_| RetrieverError::BadLen("invalid SecretKey bytes".into()))?
    } else {
        return Err(RetrieverError::BadLen(
            "ms_bytes must be 32 (mini) or 64 (secret) bytes".into(),
        )
        .into());
    };

    let mut csprng = OsRng;
    let keypair_x = Keypair::generate_with(&mut csprng);
    let pk_x = keypair_x.public;
    let sk_x_bytes = keypair_x.secret.to_bytes();
    let sk_x_scalar = Scalar::from_bytes_mod_order(sk_x_bytes[0..32].try_into()?);

    let pk_b_point = ristretto_point_from_pk(&pk_b);
    let point = sk_x_scalar * pk_b_point;

    let mut d_hasher = Sha512::new();
    d_hasher.update(pk_x.to_bytes());
    d_hasher.update(pk_b.to_bytes());
    d_hasher.update(point.compress().as_bytes());
    let d = Scalar::from_hash(d_hasher);

    let sk_a_scalar = Scalar::from_bytes_mod_order(secret_a.to_bytes()[0..32].try_into()?);

    let rk = sk_a_scalar * d.invert();

    let rk_bytes = rk.to_bytes();
    let pkx_bytes = pk_x.to_bytes();

    Ok((rk_bytes.to_vec(), pkx_bytes.to_vec()))
}

/// parse scalar from input bytes:
/// - if exactly 32 bytes -> use them as scalar little-endian
/// - otherwise try base64 decode; if still not 32 -> try hex decode
pub fn parse_scalar_bytes(b: &[u8]) -> Result<Scalar, RetrieverError> {
    if b.len() == 32 {
        let arr: [u8; 32] = b
            .try_into()
            .map_err(|_| RetrieverError::BadLen("expected 32 bytes".into()))?;
        return Ok(Scalar::from_bytes_mod_order(arr));
    }

    if let Ok(s) = std::str::from_utf8(b) {
        if let Ok(decoded) = general_purpose::STANDARD.decode(s) {
            if decoded.len() == 32 {
                let arr: [u8; 32] = decoded.try_into().map_err(|_| {
                    RetrieverError::BadLen("expected 32 bytes after base64 decode".into())
                })?;
                return Ok(Scalar::from_bytes_mod_order(arr));
            }
        }

        if let Ok(decoded_hex) = hex::decode(s) {
            if decoded_hex.len() == 32 {
                let arr: [u8; 32] = decoded_hex.try_into().map_err(|_| {
                    RetrieverError::BadLen("expected 32 bytes after hex decode".into())
                })?;
                return Ok(Scalar::from_bytes_mod_order(arr));
            }
        }
    }

    Err(RetrieverError::BadLen(
        "could not parse scalar: expected 32 bytes or base64/hex encoding of 32 bytes".into(),
    ))
}

/// Convert schnorrkel PublicKey to RistrettoPoint (assumes the public key encodes a compressed Ristretto)
fn ristretto_point_from_pk(pk: &PublicKey) -> RistrettoPoint {
    let compressed = CompressedRistretto(pk.to_bytes());
    compressed.decompress().expect("Invalid public key")
}
