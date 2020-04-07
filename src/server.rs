use std::io;
use std::io::prelude::*;
use std::net;
use std::vec;

const BUFFER_SIZE: usize = 512;

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
        let mut buffer: vec::Vec<u8> = vec::Vec::new();
        let read_size = stream.read_to_end(&mut buffer);
        println!("{}", String::from_utf8_lossy(&buffer[..]));

        let response = "HTTP/1.1 200 OK\r\n\r\nOK";

        stream.write(response.as_bytes())?;
        stream.flush()?;

        return Ok(());
    }
}
