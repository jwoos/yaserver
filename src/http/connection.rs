use crate::http;
use std::cell;
use std::collections;
use std::io;
use std::io::{Read, Write};
use std::net;
use std::vec;

pub struct Connection {
    stream: cell::RefCell<net::TcpStream>,
}

impl Connection {
    pub fn new(stream: net::TcpStream) -> Connection {
        println!("New connection from {}", stream.peer_addr().unwrap());

        return Connection {
            stream: cell::RefCell::new(stream),
        };
    }

    fn read_stream(&self) -> io::Result<vec::Vec<u8>> {
        let mut stream = self.stream.borrow_mut();
        let mut buffer: vec::Vec<u8> = vec::Vec::new();
        loop {
            let mut partial_buffer = [0u8; 4096];
            match stream.read(&mut partial_buffer) {
                Ok(size) => {
                    buffer.extend_from_slice(&partial_buffer);
                    if size < partial_buffer.len() {
                        break;
                    }
                }
                Err(e) => {
                    println!(
                        "An error occurred, terminating connection with {}",
                        stream.peer_addr().unwrap()
                    );
                    stream.shutdown(net::Shutdown::Both)?;
                    return Err(e);
                }
            }
        }

        return Ok(buffer);
    }

    fn write_stream(&self, data: &[u8]) -> io::Result<()> {
        let mut stream = self.stream.borrow_mut();
        stream.write(data)?;
        stream.flush()?;
        return Ok(());
    }

    pub fn handle(&self) -> io::Result<()> {
        let buffer = self.read_stream()?;

        let request = match http::request::Request::parse(buffer) {
            Ok(req) => req,
            Err(resp) => {
                self.write_stream(resp.as_bytes())?;

                return Ok(());
            }
        };
        println!(
            "{:?} {:?} {:?}",
            request.get_method(),
            request.get_path(),
            request.get_version()
        );

        let response = http::response::Response::new(
            http::HTTPVersion::HTTP_1_1,
            200,
            collections::HashMap::new(),
            vec::Vec::new(),
        );
        let response_bytes = match response.build_bytes() {
            Some(resp) => resp,
            None => {
                let stream = self.stream.borrow_mut();
                stream.shutdown(net::Shutdown::Both)?;
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could not construct response",
                ));
            }
        };

        self.write_stream(&response_bytes)?;

        let stream = self.stream.borrow_mut();
        stream.shutdown(net::Shutdown::Both)?;

        return Ok(());
    }
}
