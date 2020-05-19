use crate::data;
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
                    println!(
                        "{}",
                        String::from_utf8_lossy(&partial_buffer[..partial_buffer.len()])
                    );
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
                    stream.shutdown(net::Shutdown::Both).unwrap();
                    return Err(e);
                }
            }
        }

        let bytes = data::bytes::Bytes::new(buffer);

        let index = match bytes.find(0, &[b'\r', b'\n', b'\r', b'\n']) {
            Ok(index) => index,
            Err(_) => {
                let response = "HTTP/1.1 400 Bad Request\r\n\r\n";

                stream.write(response.as_bytes())?;
                stream.flush()?;
                return Ok(());
            }
        };

        let response = "HTTP/1.1 200 OK\r\n\r\nOK";

        stream.write(response.as_bytes())?;
        stream.flush()?;

        stream.shutdown(net::Shutdown::Both)?;

        return Ok(());
    }
}
