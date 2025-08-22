//! # SP1 Poseidon2 Hash (bn256)
pub mod fields;
pub mod merkle_tree;
pub mod poseidon2;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::bn256::FpBN256;

    #[test]
    fn test_hash() {
        let random_bytes: [u8; 32] = rand::random();
        let f1 = crate::fields::bn256::bytes_to_fp_elements(&random_bytes)[0];
        let hash = crate::poseidon2::Poseidon2::hash(&[f1], 1);
        println!("{:?}", f1);
    }
}