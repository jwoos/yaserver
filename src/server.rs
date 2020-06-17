use crate::data;
use crate::http;
use std::collections;
use std::io;
use std::io::prelude::*;
use std::net;
use std::vec;

pub struct Server {
    host: String,
    port: String,
}

impl Server {
    pub fn new(host: String, port: String) -> Server {
        return Server { host, port };
    }

    pub fn get_host(&self) -> &str {
        return &self.host[..];
    }

    pub fn get_port(&self) -> &str {
        return &self.port[..];
    }

    pub fn get_address(&self) -> String {
        return [&self.host[..], &self.port[..]].join(":");
    }

    pub fn handle_connection(&self, mut stream: net::TcpStream) -> io::Result<()> {
        println!("New connection from {}", stream.peer_addr().unwrap());

        let mut buffer: std::vec::Vec<u8> = std::vec::Vec::new();
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

        let request = match http::request::Request::parse(buffer) {
            Ok(req) => req,
            Err(resp) => {
                stream.write(resp.as_bytes())?;
                stream.flush()?;

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
                stream.shutdown(net::Shutdown::Both)?;
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could not construct response",
                ));
            }
        };

        stream.write(&response_bytes)?;
        stream.flush()?;

        stream.shutdown(net::Shutdown::Both)?;

        return Ok(());
    }
}
