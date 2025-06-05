// Let's use this as a reference 
// https://github.com/HorizenLabs/poseidon2/tree/main/plain_implementations
use crypto_bigint::{U256, Wrapping};
use crypto_bigint::impl_modulus;
use std::ops::{Add, Sub, Mul};


// bn254 scalar field modulus is:
// 21888242871839275222246405745257275088548364400416034343698204186575808495617
// In hex
// 30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001

impl_modulus!(Fq, U256, "30644E72E131A029B85045B68181585D2833E84879B9709143E1F593F0000001");