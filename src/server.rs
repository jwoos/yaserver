use std::io::prelude::*;

const BUFFER_SIZE: usize = 512;

pub struct Server {
    host: String,
    port: String,
}

impl Server {
    pub fn new(host: String, port: String) -> Server {
        return Server{
            host,
            port,
        };
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

    pub fn handle_connection(&self, mut stream: std::net::TcpStream) {
        let mut buffers = std::vec::Vec::new();

        loop {
            let mut buf = [0; BUFFER_SIZE];
            if let Ok(read_size) = stream.read(&mut buf) {
                println!("read_size: {}", read_size);
                if read_size > 0 {
                    buffers.push(buf);
                }

                if read_size < BUFFER_SIZE {
                    break;
                }
            } else {
                println!("Error reading from buffer!");
            }
        }

        for (i, data) in buffers.iter().enumerate() {
            println!("Buffer {}", i);
            println!("{}", String::from_utf8_lossy(&data[..]));
        }

        let response = "HTTP/1.1 200 OK\r\n\r\nOK";

        if let Err(e) = stream.write(response.as_bytes()) {
            println!("Error writing response: {}", e);
        }

        if let Err(e) = stream.flush() {
            println!("Error flushing stream: {}", e);
        }
    }
}
