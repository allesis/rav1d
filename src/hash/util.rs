use libc::c_int;

use super::{HashMapVecType, HashObject, HashType, ensure_sizing, hashcoeffs};
use crate::msac::{MsacContext, rav1d_msac_decode_bools};

pub fn get_hash(msac: &mut MsacContext) -> HashType {
    ensure_sizing!(HashType::BITS, u8::MAX);
    let hash: HashType = rav1d_msac_decode_bools(msac, HashType::BITS as u8) as HashType;
    return hash;
}

pub fn add_hash_object(
    coefs: Vec<i32>,
    eob: u16,
    res_ctx: u8,
    txtp: u8,
    hashmap: HashMapVecType,
    tx_size: usize,
    block_size: usize,
) {
    let hash = hashcoeffs(coefs.clone(), eob);

    let hash_object = HashObject {
        vec: coefs,
        eob: eob as i32,
        res_ctx: res_ctx,
        txtp: txtp,
    };

    let mut hashmaps_lock = hashmap.lock();
    let hashmaps_lock_tx = hashmaps_lock
        .get_mut(block_size)
        .expect("BAD INDEX ON BLOCK SIZE");
    let hashmap_lock = hashmaps_lock_tx.get_mut(tx_size);
    if let Some(hashmap_lock) = hashmap_lock {
        hashmap_lock.insert(hash, hash_object);
    } else {
        panic!("FAILED");
    }
}

pub fn get_hash_object(
    hashmap: Option<HashMapVecType>,
    hash: HashType,
    tx_size: usize,
    block_size: usize,
) -> (Vec<i32>, u8, u8, c_int) {
    if let Some(hashmap) = hashmap.clone() {
        let hashmaps_lock = hashmap.lock();
        if let Some(hashmaps_lock_tx) = hashmaps_lock.get(block_size) {
            if let Some(hashmap_lock) = hashmaps_lock_tx.get(tx_size) {
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
                        // This is also bad, we have a hash but it doesnt reference anything!
                        // We can either panic, or try our best
                        // TODO: Add a check for strict standards following
                        // If it is, we panic, otherwise return as a empty frame and try to keep going
                        //
                        // For now we just panic
                        panic!(
                            "READ A HASH BUT COULD NOT FIND IT IN HASHMAP\nHASH {:?}\nTX {:?}\nBLOCK_SIZE: {:?}\n",
                            hash, tx_size, block_size,
                        );
                        //return 0;
                    }
                }
            }
        }
    }
    panic!("SOMETHING WENT WRONG DURING HASH LOOKUP");
}
