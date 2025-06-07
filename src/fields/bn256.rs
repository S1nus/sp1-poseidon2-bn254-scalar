// use ark_ff::fields::{Fp256, MontBackend, MontConfig};

// #[derive(MontConfig)]
// #[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
// #[generator = "7"]
// pub struct FqConfig;
// pub type FpBN256 = Fp256<MontBackend<FqConfig, 4>>;

use crypto_bigint::{Encoding, Uint, impl_modulus, modular::constant_mod::Residue};

// bn254 scalar field modulus is:
// 21888242871839275222246405745257275088548364400416034343698204186575808495617
// In hex
// 30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001

/// Type alias to ensure no conflicts with SP1 U256 definition
pub type U256Field = Uint<4>;

// Expansion for:
// impl_modulus!(
//     ModulusBN254,
//     U256Field,
//     "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001"
// );
//
// Required as internally from_be_hex(...) is fucked by SP1
#[derive(Clone,Copy,Debug,Default,Eq,PartialEq)]
pub struct ModulusBN254{}

impl <const DLIMBS:usize>crypto_bigint::modular::constant_mod::ResidueParams<{
    <U256Field>::LIMBS
}>for ModulusBN254 where U256Field:crypto_bigint::ConcatMixed<MixedOutput = crypto_bigint::Uint<DLIMBS>>,{
    const LIMBS:usize =  <U256Field>::LIMBS;
    // BE Bytes, Hex = 0x30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001
    const MODULUS: U256Field = U256Field::from_be_slice(&[
        0x30, 0x64, 0x4E, 0x72, 0xE1, 0x31, 0xA0, 0x29,
        0xB8, 0x50, 0x45, 0xB6, 0x81, 0x81, 0x58, 0x5D,
        0x28, 0x33, 0xE8, 0x48, 0x79, 0xB9, 0x70, 0x91,
        0x43, 0xE1, 0xF5, 0x93, 0xF0, 0x00, 0x00, 0x01,
    ]);
    const R:U256Field = crypto_bigint::Uint::MAX.const_rem(&Self::MODULUS).0.wrapping_add(&crypto_bigint::Uint::ONE);
    const R2:U256Field = crypto_bigint::Uint::const_rem_wide(Self::R.square_wide(), &Self::MODULUS).0;
    const MOD_NEG_INV:crypto_bigint::Limb = crypto_bigint::Limb(crypto_bigint::Word::MIN.wrapping_sub(Self::MODULUS.inv_mod2k_vartime(crypto_bigint::Word::BITS as usize).as_limbs()[0].0,),);
    const R3:U256Field = crypto_bigint::modular::montgomery_reduction(&Self::R2.square_wide(), &Self::MODULUS,Self::MOD_NEG_INV,);
}

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
    bytes
        .chunks(32)
        .map(|chunk| {
            let n = U256Field::from_be_slice(chunk);
            FpBN256::new(&n)
        })
        .collect()
}

/// Converts a \[[FpBN256]\] into a big-endian Vec<u8> by reduction.
/// This uses `.retrieve()` to canonical integer representation.
pub fn fp_elements_to_bytes(elems: &[FpBN256]) -> Vec<u8> {
    elems
        .iter()
        .flat_map(|e| e.retrieve().to_be_bytes())
        .collect()
}

/// Converts a big-endian byte slice into Vec<FpBN256> assuming each 32-byte chunk is in Montgomery form.
/// NOTE: NO modular reduction is performed — use only if with Montgomery-form field elements.
pub fn bytes_to_fp_elements_mont(bytes: &[u8]) -> Vec<FpBN256> {
    bytes
        .chunks_exact(32)
        .map(|chunk| {
            let int = U256Field::from_be_slice(chunk);
            FpBN256::from_montgomery(int)
        })
        .collect()
}

/// Converts a \[[FpBN256]\] into a big-endian Vec<u8> as **raw** Montgomery form.
/// NOTE: NO modular reduction is performed — use only if with Montgomery-form field elements.
pub fn fp_elements_to_bytes_mont(elems: &[FpBN256]) -> Vec<u8> {
    elems
        .iter()
        .flat_map(|e| e.to_montgomery().to_be_bytes())
        .collect()
}
