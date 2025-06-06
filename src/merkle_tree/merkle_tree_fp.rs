use crate::fields::bn256::FpBN256;
use std::marker::PhantomData;

pub trait MerkleTreeHash {
    fn compress(&self, input: &[&FpBN256]) -> FpBN256;
}

#[derive(Clone, Debug)]
pub struct MerkleTree<P: MerkleTreeHash> {
    perm: P,
    field: PhantomData<FpBN256>,
}

impl<P: MerkleTreeHash> MerkleTree<P> {
    pub fn new(perm: P) -> Self {
        MerkleTree {
            perm,
            field: PhantomData,
        }
    }

    fn round_up_pow_n(input: usize, n: usize) -> usize {
        debug_assert!(n >= 1);
        let mut res = 1;
        // try powers, starting from n
        loop {
            res *= n;
            if res >= input {
                break;
            }
        }
        res
    }

    pub fn accumulate(&mut self, set: &[FpBN256]) -> FpBN256 {
        let set_size = set.len();
        let mut bound = Self::round_up_pow_n(set_size, 2);
        loop {
            if bound >= 2 {
                break;
            }
            bound *= 2;
        }
        let mut nodes: Vec<FpBN256> = Vec::with_capacity(bound);
        for s in set {
            nodes.push(s.to_owned());
        }
        // pad
        for _ in nodes.len()..bound {
            nodes.push(nodes[set_size - 1].to_owned());
        }

        while nodes.len() > 1 {
            let new_len = nodes.len() / 2;
            let mut new_nodes: Vec<FpBN256> = Vec::with_capacity(new_len);
            for i in (0..nodes.len()).step_by(2) {
                let inp = [&nodes[i], &nodes[i + 1]];
                let dig = self.perm.compress(&inp);
                new_nodes.push(dig);
            }
            nodes = new_nodes;
        }
        nodes[0].to_owned()
    }
}
