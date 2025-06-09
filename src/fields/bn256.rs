// Scalar field of BN254 curve using `crypto-bigint`
// Compatible with both 32-bit (e.g., SP1 zkVM) and 64-bit targets.

use crypto_bigint::{Encoding, Uint, impl_modulus, modular::constant_mod::Residue};

// --- 256 bit field size for platform word width ---
#[cfg(target_pointer_width = "64")]
const LIMBSIZE: usize = 4;
#[cfg(target_pointer_width = "32")]
const LIMBSIZE: usize = 8;

pub type U256Field = Uint<LIMBSIZE>;

// bn254 scalar field modulus is:
// 21888242871839275222246405745257275088548364400416034343698204186575808495617
// In hex
// 30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001

impl_modulus!(
    ModulusBN254,
    U256Field,
    "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001"
);

pub type FpBN256 = Residue<ModulusBN254, LIMBSIZE>;

/// Trait providing in-place modular operations
pub trait ModMathInPlace {
    fn square_in_place(&mut self) -> &mut Self;
    fn double_in_place(&mut self) -> &mut Self;
}

impl ModMathInPlace for FpBN256 {
    #[inline]
    fn square_in_place(&mut self) -> &mut Self {
        *self *= *self;
        self
    }

    #[inline]
    fn double_in_place(&mut self) -> &mut Self {
        *self += *self;
        self
    }
}

/// Converts a big-endian byte slice into a `Vec<FpBN256>` by modular reduction.
/// Each chunk must be exactly 32 bytes (256 bits).
pub fn bytes_to_fp_elements(bytes: &[u8]) -> Vec<FpBN256> {
    bytes
        .chunks_exact(32)
        .map(|chunk| {
            let n = U256Field::from_be_slice(chunk);
            FpBN256::new(&n)
        })
        .collect()
}

/// Converts a slice of `[FpBN256]` into a big-endian `Vec<u8>` using canonical integer representation.
pub fn fp_elements_to_bytes(elems: &[FpBN256]) -> Vec<u8> {
    elems
        .iter()
        .flat_map(|e| e.retrieve().to_be_bytes())
        .collect()
}

/// Converts a big-endian byte slice into `Vec<FpBN256>` assuming each chunk is in Montgomery form.
/// Does **not** perform reduction — use only with known Montgomery values.
pub fn bytes_to_fp_elements_mont(bytes: &[u8]) -> Vec<FpBN256> {
    bytes
        .chunks_exact(32)
        .map(|chunk| {
            let int = U256Field::from_be_slice(chunk);
            FpBN256::from_montgomery(int)
        })
        .collect()
}

/// Converts a `[FpBN256]` slice into a big-endian `Vec<u8>` representing raw Montgomery form.
/// No modular reduction is performed.
pub fn fp_elements_to_bytes_mont(elems: &[FpBN256]) -> Vec<u8> {
    elems
        .iter()
        .flat_map(|e| e.to_montgomery().to_be_bytes())
        .collect()
}
