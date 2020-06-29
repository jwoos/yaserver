use crate::http;
use crate::thread;
use std::collections::HashMap;
use std::net;
use std::sync::Arc;

// put into Router??
#[derive(Clone)]
pub struct ServerParams {
    pub static_directory: String,
}

impl ServerParams {
    pub fn new(static_directory: String) -> ServerParams {
        return ServerParams { static_directory };
    }
}

pub struct Server {
    host: String,
    port: String,
    address: String,
    thread_pool: thread::ThreadPool,
    params: Arc<ServerParams>,
}

impl Server {
    pub fn new(host: String, port: String, thread_count: usize, params: ServerParams) -> Server {
        let address = [&host[..], &port[..]].join(":");

        return Server {
            host,
            port,
            address,
            thread_pool: thread::ThreadPool::new(thread_count),
            params: Arc::new(params),
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
            let params = self.params.clone();
            self.thread_pool.execute(move || {
                let stream: net::TcpStream = match stream_res {
                    Ok(stream) => stream,
                    Err(e) => {
                        println!("Error establishing connection: {}", e);
                        return;
                    }
                };

                let connection = http::connection::Connection::new(stream, params);
                if let Err(e) = connection.handle() {
                    println!("Error handling connection: {}", e);
                }
            });
        }
    }
}
