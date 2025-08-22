use crate::fields::bn256::FpBN256;
use core::ops::{MulAssign, SubAssign};

// gaussian elimination
pub fn mat_inverse(mat: &[Vec<FpBN256>]) -> Vec<Vec<FpBN256>> {
    let n = mat.len();
    assert!(mat[0].len() == n);

    let mut m = mat.to_owned();
    let mut inv = vec![vec![FpBN256::ZERO; n]; n];
    for (i, invi) in inv.iter_mut().enumerate() {
        invi[i] = FpBN256::ONE;
    }

    // upper triangle
    for row in 0..n {
        for j in 0..row {
            let el = m[row][j];
            for col in 0..n {
                if col < j {
                    m[row][col] = FpBN256::ZERO;
                } else {
                    let mut tmp = m[j][col];
                    tmp.mul_assign(&el);
                    m[row][col].sub_assign(&tmp);
                }
                if col > row {
                    inv[row][col] = FpBN256::ZERO;
                } else {
                    let mut tmp = inv[j][col];
                    tmp.mul_assign(&el);
                    inv[row][col].sub_assign(&tmp);
                }
            }
        }
        let el_inv = invert_unwrap(&m[row][row]);
        for col in 0..n {
            match col.cmp(&row) {
                std::cmp::Ordering::Less => inv[row][col].mul_assign(&el_inv),
                std::cmp::Ordering::Equal => {
                    m[row][col] = FpBN256::ONE;
                    inv[row][col].mul_assign(&el_inv)
                }
                std::cmp::Ordering::Greater => m[row][col].mul_assign(&el_inv),
            }
        }
    }

    // back substitution
    for row in (0..n).rev() {
        for j in (row + 1..n).rev() {
            let el = m[row][j];
            for col in 0..n {
                #[cfg(debug_assertions)]
                {
                    if col >= j {
                        m[row][col] = FpBN256::ZERO;
                    }
                }
                let mut tmp = inv[j][col];
                tmp.mul_assign(&el);
                inv[row][col].sub_assign(&tmp);
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        for (row, mrow) in m.iter().enumerate() {
            for (col, v) in mrow.iter().enumerate() {
                if row == col {
                    debug_assert!(*v == FpBN256::ONE);
                } else {
                    debug_assert!(*v == FpBN256::ZERO);
                }
            }
        }
    }

    inv
}

pub fn mat_transpose(mat: &[Vec<FpBN256>]) -> Vec<Vec<FpBN256>> {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut transpose = vec![vec![FpBN256::ZERO; rows]; cols];

    for (row, matrow) in mat.iter().enumerate() {
        for col in 0..cols {
            transpose[col][row] = matrow[col];
        }
    }
    transpose
}

/// Multiplicative inverse, panics if not found.
pub fn invert_unwrap(x: &FpBN256) -> FpBN256 {
    let (inv, ok) = x.invert();
    if <crypto_bigint::CtChoice as Into<bool>>::into(ok) == false {
        panic!("inversion failed");
    }
    inv
}
