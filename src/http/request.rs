use crate::data;
use crate::http;
use ascii::{AsAsciiStr, AsciiChar, AsciiStr, AsciiString};
use std::collections::HashMap;
use std::vec;

pub struct Request {
    method: http::HTTPMethod,
    path: AsciiString,
    version: http::HTTPVersion,
    headers: HashMap<AsciiString, AsciiString>,
    body: vec::Vec<u8>,
}

const BAD_REQUEST: &'static str = "HTTP/1.1 400 Bad Request\r\n\r\n";

impl Request {
    pub fn parse(data: vec::Vec<u8>) -> Result<Request, &'static str> {
        let index = match data::utility::find(&data[..], 0, &[b'\r', b'\n', b'\r', b'\n']) {
            Some(index) => index,
            None => {
                return Err(BAD_REQUEST);
            }
        };

        // TODO deal with invalid ASCII
        let head = match (&data[..]).slice_ascii(..index) {
            Ok(val) => val.to_ascii_string(),
            Err(_) => {
                return Err(BAD_REQUEST);
            }
        };
        let mut head_iter = head.lines();
        let body = vec::Vec::from(&data[index + 4..]);

        let first_line: vec::Vec<&AsciiStr> = match head_iter.next() {
            Some(line) => line.split(AsciiChar::Space).collect(),
            None => {
                return Err(BAD_REQUEST);
            }
        };
        if first_line.len() != 3 {
            return Err(BAD_REQUEST);
        }

        let method = http::string_to_http_method(first_line[0].as_str()).ok_or(BAD_REQUEST)?;
        let path = AsciiString::from(first_line[1]);
        let version = http::string_tp_http_version(first_line[2].as_str()).ok_or(BAD_REQUEST)?;

        let mut headers: HashMap<AsciiString, AsciiString> = HashMap::new();

        for line in head_iter {
            let parts = line.split(AsciiChar::Space).collect::<vec::Vec<_>>();
            let mut val = AsciiString::new();

            for v_part in &parts[1..] {
                val.push_str(v_part);
            }

            headers.insert(AsciiString::from(parts[0]), val);
        }

        return Ok(Request {
            method: method,
            path: path,
            version: version,
            headers: headers,
            body: body,
        });
    }

    pub fn get_method(&self) -> http::HTTPMethod {
        return self.method;
    }

    pub fn get_path(&self) -> &AsciiStr {
        return &self.path[..];
    }

    pub fn get_version(&self) -> http::HTTPVersion {
        return self.version;
    }

    pub fn get_headers(&self) -> &HashMap<AsciiString, AsciiString> {
        return &self.headers;
    }

    pub fn get_body(&self) -> &vec::Vec<u8> {
        return &self.body;
    }
}
