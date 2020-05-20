use crate::data;
use crate::http;
use std::ascii;
use std::collections::HashMap;
use std::vec;

// TODO the rest of the HTTP request
pub struct Request<'a> {
    data: data::bytes::Bytes,
    head: String,
    method: http::HTTPMethod,
    path: &'a str,
    headers: HashMap<&'a str, &'a str>,
    body: &'a [u8],
}

impl<'a> Request<'a> {
    pub fn new(
        data: data::bytes::Bytes,
        head: String,
        method: http::HTTPMethod,
        path: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: &'a [u8],
    ) -> Request<'a> {
        return Request {
            data,
            head,
            method,
            path,
            headers,
            body,
        };
    }

    /*
     *        pub fn parse(data: data::bytes::Bytes) -> Result<Request<'a>, ()> {
     *            let iter = data.split(&[b'\r', b'\n']);
     *
     *            // first line
     *            if let Some(first) = iter.next() {
     *                let first_iter = first.split()
     *
     *            } else {
     *                return Err(());
     *            }
     *
     *            return Some(Request {
     *            });
     *        }
     */
}
