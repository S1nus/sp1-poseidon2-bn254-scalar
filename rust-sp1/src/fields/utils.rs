use crate::fields::bn256::{FpBN256, U256Field};

/// Converts a hex string into FpBN256
/// Interpret as a big-endian number, reducing as needed
/// FIXME: vet this is actually correct!
/// TODO: Can this be made a const fn? Avoid lazy_static for performance
pub fn from_hex(s: &str) -> FpBN256 {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(s).expect("invalid hex");

    let mut res = FpBN256::new(&U256Field::ZERO);
    let radix = FpBN256::new(&U256Field::from_u64(256));

    for &byte in bytes.iter() {
        res *= &radix;
        res += FpBN256::new(&U256Field::from_u64(byte as u64));
    }

    res
}

#[cfg(feature = "std")]
use crypto_bigint::Random;
#[cfg(feature = "std")]
use rand::thread_rng;

#[cfg(feature = "std")]
pub fn random_scalar() -> FpBN256 {
    let mut rng = thread_rng();
    FpBN256::random(&mut rng)
}

#[cfg(feature = "std")]
pub fn random_scalar_without_0() -> FpBN256 {
    use crypto_bigint::Zero;
    loop {
        let element = random_scalar();
        if (!element.is_zero()).into() {
            return element;
        }
    }
}
