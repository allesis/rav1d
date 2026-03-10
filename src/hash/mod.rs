use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

pub mod util;
pub type HashType = u16;
pub type HashMapType = HashMap<HashType, HashObject>;
pub type HashMapVecType = Arc<RwLock<HashMapType>>;
pub type HashBufferType = Arc<Mutex<Vec<(HashType, HashObject, usize)>>>;
pub const HASHMASK: HashType = HashType::MAX;

pub struct HashObject {
    pub vec: Vec<i32>,
    pub eob: i32,
    pub res_ctx: u8,
    pub txtp: u8,
}
