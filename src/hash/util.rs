use std::hash::{DefaultHasher, Hash, Hasher};

use vq::{Quantizer, ScalarQuantizer};

use super::{HashType, HASHMASK};

pub fn quantize(coeffs: Vec<i32>) -> Vec<u8> {
    let vec_coeffs = coeffs
        .iter()
        .map(|coeff| *coeff as f32)
        .collect::<Vec<f32>>();

    let sq: ScalarQuantizer = ScalarQuantizer::new(0.0, 31.0, 32).unwrap();

    let qcoeffs = sq.quantize(&vec_coeffs).unwrap();

    qcoeffs
}

pub fn hashcoeffs(coeffs: Vec<u8>) -> HashType {
    let mut hasher = DefaultHasher::new();
    coeffs.iter().for_each(|coeff| (*coeff).hash(&mut hasher));
    let hash = hasher.finish();
    (hash & (HASHMASK as u64))
        .try_into()
        .expect("FAILED TO CONVERT HASH")
}
