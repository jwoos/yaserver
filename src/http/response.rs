use crate::http;
use ascii::AsciiString;
use std::collections::HashMap;
use std::vec;

pub struct Response {
    version: http::HTTPVersion,
    code: u16,
    headers: HashMap<AsciiString, AsciiString>,
    body: std::vec::Vec<u8>,
}

impl Response {
    pub fn new(
        version: http::HTTPVersion,
        code: u16,
        headers: HashMap<AsciiString, AsciiString>,
        body: std::vec::Vec<u8>,
    ) -> Response {
        return Response {
            version,
            code,
            headers,
            body,
        };
    }

    pub fn build_bytes(&self) -> Option<vec::Vec<u8>> {
        let mut res = String::new();
        let version_string = http::http_version_to_string(self.version)?;

        res.push_str(&format!("{} {} {}", version_string, self.code, "\r\n"));

        for (k, v) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", k, v));
        }

        res.push_str(&format!("{}: {}\r\n", "Content-Length", self.body.len()));

        res.push_str("\r\n");

        let mut bytes: vec::Vec<u8> = vec::Vec::new();
        bytes.extend_from_slice(res.as_bytes());
        bytes.extend(self.body.iter());

        return Some(bytes);
    }
}
