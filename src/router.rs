use std::collections::HashMap;
use std::vec::Vec;

pub struct Router {
    cache: HashMap<String, String>,
    static_cache: HashMap<String, Vec<u8>>,
}
