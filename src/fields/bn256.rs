// use ark_ff::fields::{Fp256, MontBackend, MontConfig};

// #[derive(MontConfig)]
// #[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
// #[generator = "7"]
// pub struct FqConfig;
// pub type FpBN256 = Fp256<MontBackend<FqConfig, 4>>;

use crypto_bigint::{impl_modulus, modular::constant_mod::Residue, Encoding, U256};

// bn254 scalar field modulus is:
// 21888242871839275222246405745257275088548364400416034343698204186575808495617
// In hex
// 30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001

impl_modulus!(
    ModulusBN254,
    U256,
    "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001"
);
pub type FpBN256 = Residue<ModulusBN254, 4>; // LIMBS = 4 for max 256bit field

pub trait ModMathInPlace {
    fn square_in_place(&mut self) -> &mut Self;
    fn double_in_place(&mut self) -> &mut Self;
}

impl ModMathInPlace for FpBN256 {
    fn square_in_place(&mut self) -> &mut Self {
        *self *= *self;
        self
    }
    fn double_in_place(&mut self) -> &mut Self {
        *self += *self;
        self
    }
}

/// Converts a big endian byte slice by reducing each 32-byte chunk into a Vec<FpBN256>.
pub fn bytes_to_fp_elements(bytes: &[u8]) -> Vec<FpBN256> {
    bytes.chunks(32)
        .map(|chunk| {
            let mut buf = [0u8; 32];
            buf[..chunk.len()].copy_from_slice(chunk);
            let n = U256::from_be_bytes(buf);
            FpBN256::new(&n) // modular reduction mod p
        })
        .collect()
}

/// Converts a [FpBN256] into a big-endian Vec<u8> by reduction.
/// This uses `.retrieve()` to canonical integer representation.
pub fn fp_elements_to_bytes(elems: &[FpBN256]) -> Vec<u8> {
    elems.iter()
        .flat_map(|e| e.retrieve().to_be_bytes())
        .collect()
}

/// Converts a big-endian byte slice into Vec<FpBN256> assuming each 32-byte chunk is in Montgomery form.
/// NOTE: NO modular reduction is performed — use only if with Montgomery-form field elements.
pub fn bytes_to_fp_elements_mont(bytes: &[u8]) -> Vec<FpBN256> {
    bytes.chunks_exact(32)
        .map(|chunk| {
            let int = U256::from_be_bytes(chunk.try_into().unwrap());
            FpBN256::from_montgomery(int)
        })
        .collect()
}

/// Converts a [FpBN256] into a big-endian Vec<u8> as **raw** Montgomery form.
/// NOTE: NO modular reduction is performed — use only if with Montgomery-form field elements.
pub fn fp_elements_to_bytes_mont(elems: &[FpBN256]) -> Vec<u8> {
    elems.iter()
        .flat_map(|e| e.to_montgomery().to_be_bytes())
        .collect()
}
