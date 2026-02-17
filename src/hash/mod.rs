use std::{
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    sync::Arc,
};
pub mod util;

use parking_lot::Mutex;

use crate::levels::TxfmSize;
pub type HashType = u16;
pub const PLANE_COUNT: usize = 3;
pub const HASHMASK: HashType = HashType::MAX;
pub type HashMapType = HashMap<HashType, HashObject>;
pub type HashMapVecType = Arc<Mutex<[[HashMapType; TxfmSize::_NUM_RECT]; PLANE_COUNT]>>;

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
    eob.hash(&mut hasher);
    let hash = hasher.finish();
    (hash & (HASHMASK as u64))
        .try_into()
        .expect("FAILED TO CONVERT HASH")
}

// Ensure that `$test_size` is less than `$max_size`
// We could also turn this into a `const fn` as follows:
//
// ```
// const fn ensure_sizing_fn(test_size: usize, max_size: usize) {
//   assert!(test_size <= max_size);
// }
// ```
//
// However, this means that we don't get LSP warnings from static analysis
// So we use a macro to ensure we do
macro_rules! ensure_sizing {
    ($test_size:expr, $max_size:expr) => {
        // This will allow us to catch bad sizing match ups before runtime
        // We need the `const _: () =` here or the assert! is not evaluated
        // until runtime
        // Which is not what we want
        const _: () = assert!($test_size as usize <= $max_size as usize);
    };
}

pub(crate) use ensure_sizing;
