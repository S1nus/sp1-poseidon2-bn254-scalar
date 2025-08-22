use crate::fields::bn256::FpBN256;
use core::ops::{AddAssign, MulAssign};

use crate::utils;

#[derive(Clone, Debug)]
pub struct Poseidon2Params {
    pub(crate) t: usize, // statesize
    pub(crate) d: usize, // sbox degree
    pub(crate) rounds_f_beginning: usize,
    pub(crate) rounds_p: usize,
    #[allow(dead_code)]
    pub(crate) rounds_f_end: usize,
    pub(crate) rounds: usize,
    pub(crate) mat_internal_diag_m_1: Vec<FpBN256>,
    pub(crate) _mat_internal: Vec<Vec<FpBN256>>,
    pub(crate) round_constants: Vec<Vec<FpBN256>>,
}

impl Poseidon2Params {
    #[allow(clippy::too_many_arguments)]

    pub const INIT_SHAKE: &'static str = "Poseidon2";

    pub fn new(
        t: usize,
        d: usize,
        rounds_f: usize,
        rounds_p: usize,
        mat_internal_diag_m_1: &[FpBN256],
        mat_internal: &[Vec<FpBN256>],
        round_constants: &[Vec<FpBN256>],
    ) -> Self {
        assert!(d == 3 || d == 5 || d == 7 || d == 11);
        assert_eq!(rounds_f % 2, 0);
        let r = rounds_f / 2;
        let rounds = rounds_f + rounds_p;

        Poseidon2Params {
            t,
            d,
            rounds_f_beginning: r,
            rounds_p,
            rounds_f_end: r,
            rounds,
            mat_internal_diag_m_1: mat_internal_diag_m_1.to_owned(),
            _mat_internal: mat_internal.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }

    // Unused
    pub fn equivalent_round_constants(
        round_constants: &[Vec<FpBN256>],
        mat_internal: &[Vec<FpBN256>],
        rounds_f_beginning: usize,
        rounds_p: usize,
    ) -> Vec<Vec<FpBN256>> {
        let mut opt = vec![Vec::new(); rounds_p + 1];
        let mat_internal_inv = utils::mat_inverse(mat_internal);

        let p_end = rounds_f_beginning + rounds_p - 1;
        let mut tmp = round_constants[p_end].clone();
        for i in (0..rounds_p - 1).rev() {
            let inv_cip = Self::mat_vec_mul(&mat_internal_inv, &tmp);
            opt[i + 1] = vec![inv_cip[0]];
            tmp = round_constants[rounds_f_beginning + i].clone();
            for i in 1..inv_cip.len() {
                tmp[i].add_assign(&inv_cip[i]);
            }
        }
        opt[0] = tmp;
        opt[rounds_p] = vec![FpBN256::ZERO; opt[0].len()]; // opt[0].len() = t

        opt
    }

    pub fn mat_vec_mul(mat: &[Vec<FpBN256>], input: &[FpBN256]) -> Vec<FpBN256> {
        let t = mat.len();
        debug_assert!(t == input.len());
        let mut out = vec![FpBN256::ZERO; t];
        for row in 0..t {
            for (col, inp) in input.iter().enumerate() {
                let mut tmp = mat[row][col];
                tmp.mul_assign(inp);
                out[row].add_assign(&tmp);
            }
        }
        out
    }
}
