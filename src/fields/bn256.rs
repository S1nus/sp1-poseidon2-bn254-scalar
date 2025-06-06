// use ark_ff::fields::{Fp256, MontBackend, MontConfig};

// #[derive(MontConfig)]
// #[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
// #[generator = "7"]
// pub struct FqConfig;
// pub type FpBN256 = Fp256<MontBackend<FqConfig, 4>>;

use crypto_bigint::{U256, impl_modulus, modular::constant_mod::Residue};

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
