use crate::data;
use crate::http;
use crate::thread;
use std::collections;
use std::io;
use std::io::prelude::*;
use std::net;
use std::vec;

pub struct ServerParams {
    staticDirectory: String,
}

pub struct Server {
    host: String,
    port: String,
    address: String,
    threadPool: thread::ThreadPool,
}

impl Server {
    pub fn new(host: String, port: String, thread_count: usize) -> Server {
        let address = [&host[..], &port[..]].join(":");
        return Server {
            host,
            port,
            address,
            threadPool: thread::ThreadPool::new(thread_count),
        };
    }

    pub fn get_host(&self) -> &str {
        return &self.host[..];
    }

    pub fn get_port(&self) -> &str {
        return &self.port[..];
    }

    pub fn get_address(&self) -> &str {
        return &self.address[..];
    }

    pub fn serve(&self) {
        println!("Serving on {}", self.address);
        let listener = std::net::TcpListener::bind(&self.address).unwrap();

        for stream_res in listener.incoming() {
            self.threadPool.execute(move || {
                let stream: net::TcpStream = match stream_res {
                    Ok(stream) => stream,
                    Err(e) => {
                        println!("Error establishing connection: {}", e);
                        return;
                    }
                };

                let connection = http::connection::Connection::new(stream);
                if let Err(e) = connection.handle() {
                    println!("Error handling connection: {}", e);
                }
            });
        }
    }
}
