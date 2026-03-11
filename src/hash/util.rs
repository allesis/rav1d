use std::hash::{DefaultHasher, Hash, Hasher};

use vq::{Quantizer, ScalarQuantizer};

use super::{HashMapVecType, HashObject, HashType, HASHMASK};
use crate::msac::{rav1d_msac_decode_bool_equi, rav1d_msac_decode_bools, MsacContext};

pub fn quantize(coeffs: Vec<i32>) -> Vec<u8> {
    let vec_coeffs = coeffs
        .iter()
        .map(|coeff| *coeff as f32)
        .collect::<Vec<f32>>();

    let sq: ScalarQuantizer = ScalarQuantizer::new(0.0, 255.0, 256).unwrap();

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

pub fn get_hash(msac: &mut MsacContext) -> HashType {
    let mut hash: HashType = 0;
    let temp: u16 = rav1d_msac_decode_bools(msac, 16).try_into().unwrap();

    dbg!(temp);

    assert!(temp == 42);

    for _ in 0..HashType::BITS {
        hash <<= 1;
        let bit = rav1d_msac_decode_bool_equi(msac) as HashType;
        hash |= bit;
    }
    dbg!(hash);
    hash
}

pub fn add_hash_object(hashmap: HashMapVecType, coeffs: Vec<i32>, eob: i32, res_ctx: u8, txtp: u8) {
    let qcoeffs = quantize(coeffs.clone());

    let hash = hashcoeffs(qcoeffs);

    let hash_object = HashObject {
        vec: coeffs,
        eob,
        res_ctx,
        txtp,
    };

    let mut hashmap_lock = hashmap
        .write()
        .expect("Failed to get write lock on hashmap");
    hashmap_lock.insert(hash, hash_object);
}

pub fn get_hash_object(hashmap: HashMapVecType, hash: HashType) -> (Vec<i32>, u8, u8, i32) {
    let hashmap_lock = hashmap.read().expect("Failed to get read lock on hashmap");
    match hashmap_lock.get(&hash) {
        Some(hash_object) => {
            return (
                hash_object.vec.clone(),
                hash_object.res_ctx,
                hash_object.txtp,
                hash_object.eob,
            );
        }
        None => {
            dbg!(hash);
            panic!("READ A HASH BUT COULD NOT FIND IT IN HASHMAP");
        }
    }
}
