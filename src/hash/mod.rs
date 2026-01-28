use std::{
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    sync::Arc,
};
pub mod util;

use parking_lot::Mutex;

use crate::levels::TxfmSize;
pub type HashType = u16;
pub const BLOCK_SIZE_COUNT: usize = 22;
pub const HASHMASK: HashType = HashType::MAX;
pub type HashMapType = HashMap<HashType, HashObject>;
pub type HashMapVecType = Arc<Mutex<[[HashMapType; TxfmSize::_NUM_RECT]; BLOCK_SIZE_COUNT]>>;

pub struct HashObject {
    pub vec: Vec<i32>,
    pub eob: i32,
    pub res_ctx: u8,
    pub txtp: u8,
}

fn hashcoeffs(coeffs: Vec<i32>, eob: u16) -> HashType {
    let mut hasher = DefaultHasher::new();
    coeffs.iter().for_each(|coeff| {
        if *coeff == 0 {
        } else {
            (*coeff).hash(&mut hasher)
        }
    });
    //eob.hash(&mut hasher);
    let hash = hasher.finish();
    (hash & (HASHMASK as u64))
        .try_into()
        .expect("FAILED TO CONVERT HASH")
}
