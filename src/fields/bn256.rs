use ark_ff::fields::{Fp256, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "7"]
pub struct FqConfig;
pub type FpBN256 = Fp256<MontBackend<FqConfig, 4>>;
