use crate::http;
use crate::server::ServerParams;
use ascii::{AsciiChar, AsciiStr};
use std::cell;
use std::collections;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net;
use std::sync::Arc;
use std::vec;

pub struct Connection {
    stream: cell::RefCell<net::TcpStream>,
    params: Arc<ServerParams>,
}

impl Connection {
    pub fn new(stream: net::TcpStream, params: Arc<ServerParams>) -> Connection {
        println!("New connection from {}", stream.peer_addr().unwrap());

        return Connection {
            stream: cell::RefCell::new(stream),
            params,
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

    // TODO change it to take vector
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

        let path = request.get_path();
        println!(
            "{:?} {:?} {:?}",
            request.get_method(),
            path,
            request.get_version()
        );

        // TODO fix
        if self.params.static_directory.len() > 0 {
            let parts = path.split(AsciiChar::Slash).collect::<vec::Vec<_>>();
            if parts[1] == self.params.static_directory {
                let mut file = File::open(String::from(path[1..].as_str())).unwrap();
                let mut data = vec::Vec::new();

                file.read_to_end(&mut data).unwrap();

                // TODO separate into function
                let response = http::response::Response::new(
                    http::HTTPVersion::HTTP_1_1,
                    200,
                    collections::HashMap::new(),
                    data,
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
            }
        } else {
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
        }

        return Ok(());
    }
}
