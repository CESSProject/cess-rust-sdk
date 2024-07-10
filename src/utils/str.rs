use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::time::SystemTime;

const LETTER_ID_BITS: i32 = 6;
const LETTER_ID_MASK: i32 = 1 << (LETTER_ID_BITS - 1);
const LETTER_ID_MAX: i32 = 63 / LETTER_ID_BITS;
const BASE_STR: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()[]{}+-*/_=.";

pub fn get_random_code(length: u8) -> Result<String, Box<dyn std::error::Error>> {
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
