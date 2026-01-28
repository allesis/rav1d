use std::{collections::HashMap, sync::Arc};
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
