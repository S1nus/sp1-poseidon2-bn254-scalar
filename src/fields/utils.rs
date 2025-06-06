use crate::fields::bn256::FpBN256;

use crypto_bigint::{Random, U256, Zero};

/// Converts a hex string into FpBN256
/// Interpret as a big-endian number, reducing as needed
/// TODO: vet this is actually correct!
pub fn from_hex(s: &str) -> FpBN256 {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(s).expect("invalid hex");

    let mut res = FpBN256::new(&U256::ZERO);
    let radix = FpBN256::new(&U256::from_u64(256));

    for &byte in bytes.iter() {
        res *= &radix;
        res += FpBN256::new(&U256::from_u64(byte as u64));
    }

    res
}

pub fn random_scalar<FpBN246>() -> FpBN256 {
    let mut rng = rand::thread_rng();
    FpBN256::random(&mut rng)
}

pub fn random_scalar_without_0<FpBN246>() -> FpBN256 {
    loop {
        let element = random_scalar::<FpBN256>();
        if (!element.is_zero()).into() {
            return element;
        }
    }
}
