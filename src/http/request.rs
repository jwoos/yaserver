use crate::http;
use std::collections::HashMap;
use std::vec;

pub struct Request {
    method: http::HTTPMethod,
    data: vec::Vec<u8>,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn new(method: http::HTTPMethod, data: vec::Vec<u8>) -> Request {
        let hashmap: HashMap<String, String> = HashMap::new();
        return Request {
            method: method,
            data: data,
            headers: hashmap,
        };
    }

    pub fn parse(data: vec::Vec<u8>) -> (HashMap<String, String>, String) {
        let mut headers: HashMap<String, String> = HashMap::new();
        return (headers, "".to_string());
    }
}
