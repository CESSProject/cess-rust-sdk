//! # String Utilities
//!
//! Provides helper functions for generating secure random strings and codes.
//!
//! These utilities are typically used for temporary identifiers, tokens,
//! or verification codes within the SDK. Randomness is seeded using
//! the current system time to ensure non-deterministic output.
//!
//! ## Features
//! - Generate random alphanumeric and symbolic strings
//! - Produce variable-length random codes with reproducible structure
//!
//! ## Notes
//! The randomness source (`StdRng`) is pseudorandom and suitable for
//! general use cases, but not cryptographically secure.

use crate::core::Error;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::time::SystemTime;

const LETTER_ID_BITS: i32 = 6;
const LETTER_ID_MASK: i32 = 1 << (LETTER_ID_BITS - 1);
const LETTER_ID_MAX: i32 = 63 / LETTER_ID_BITS;

/// Base character set used for random string generation.
const BASE_STR: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()[]{}+-*/_=.";

/// Generates a random string of the specified length.
///
/// Uses a time-based seed combined with an additional random offset to
/// create non-repeating outputs across invocations.
///
/// # Arguments
/// * `length` – Desired length of the output string.
///
/// # Returns
/// A random alphanumeric string of `length` characters.
///
/// # Errors
/// Returns an [`Error`] if an internal RNG operation fails (rare).
///
/// # Example
/// ```ignore
/// let code = str::get_random_code(12)?;
/// println!("Generated code: {}", code);
/// ```
pub fn get_random_code(length: u8) -> Result<String, Error> {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
        + rand::random::<i64>().wrapping_abs() as u64;

    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let mut bytes = Vec::with_capacity(length as usize);
    for _ in 0..length {
        bytes.push(
            BASE_STR
                .chars()
                .nth(rng.gen_range(0..BASE_STR.len()))
                .unwrap(),
        );
    }

    Ok(bytes.into_iter().collect())
}

/// Generates a pseudo-random string of a given length.
///
/// This variant avoids additional randomness from the system
/// and uses only the current timestamp as seed.
///
/// # Arguments
/// * `n` – Length of the desired random string.
///
/// # Returns
/// A random string composed of characters from [`BASE_STR`].
///
/// # Example
/// ```
/// let s = str::rand_str(8);
/// println!("Random string: {}", s);
/// ```
pub fn rand_str(n: usize) -> String {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let mut rand_str = String::with_capacity(n);
    let mut cache = rng.gen::<i64>();
    let mut remain = LETTER_ID_MAX;

    for _ in (0..n).rev() {
        if remain == 0 {
            cache = rng.gen::<i64>();
            remain = LETTER_ID_MAX;
        }
        let idx = (cache & LETTER_ID_MASK as i64) as usize;
        if idx < BASE_STR.len() {
            rand_str.push(BASE_STR.chars().nth(idx).unwrap());
            remain -= 1;
        }
        cache >>= LETTER_ID_BITS;
    }

    rand_str
}
