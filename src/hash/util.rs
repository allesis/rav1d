use std::hash::{DefaultHasher, Hash, Hasher};

use super::{CoeffVecType, HashMapVecType, HashObject, HashType, HASHMASK};
use crate::msac::{rav1d_msac_decode_bool_equi, rav1d_msac_decode_bools, MsacContext};

pub fn quantize(coeffs: CoeffVecType) -> CoeffVecType {
    coeffs
        .iter()
        .map(|coeff| (*coeff as i32) >> 3)
        .collect::<CoeffVecType>()
}

pub fn hashcoeffs(coeffs: CoeffVecType) -> HashType {
    let mut hasher = DefaultHasher::new();
    coeffs.iter().for_each(|coeff| {
        if *coeff != 0 {
            (*coeff).hash(&mut hasher)
        }
    });
    let hash = hasher.finish();
    (hash & (HASHMASK as u64))
        .try_into()
        .expect("FAILED TO CONVERT HASH")
}

pub fn get_hash(msac: &mut MsacContext) -> HashType {
    let mut hash: HashType = rav1d_msac_decode_bools(msac, HashType::BITS.try_into().unwrap())
        .try_into()
        .unwrap();
    hash
}

pub fn add_hash_object(hashmap: HashMapVecType, coeffs: Vec<i32>, eob: i32, res_ctx: u8, txtp: u8) {
    let qcoeffs = quantize(coeffs.clone());

    let hash = hashcoeffs(qcoeffs);

    if *coeffs.get(0).unwrap() == 131 {
        dbg!(&coeffs);
    }

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
            println!("HASH");
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
