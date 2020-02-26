use crate::http;

pub struct Request {
    method: http::HTTPMethod,
    data: std::vec::Vec<u8>,
}

impl Request {
    pub fn new(method: http::HTTPMethod, data: std::vec::Vec<u8>) -> Request {
        return Request {
            method,
            data,
        };
    }
}
