mod http;
mod server;

const HOST: &str = "127.0.0.1";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Requires a port argument");
        std::process::exit(1);
    }

    let server = server::Server::new(String::from(HOST), {
        if let Some(p) = args.get(1) {
            p.clone()
        } else {
            "8000".to_string()
        }
    });

    let address = server.get_address();

    println!("Serving on {}", address);

    let listener = std::net::TcpListener::bind(address).unwrap();

    for stream_res in listener.incoming() {
        let stream = match stream_res {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error establishing connection: {}", e);
                continue;
            }
        };

        if let Err(e) = server.handle_connection(stream) {
            println!("Error handling connection: {}", e);
        }
    }
}
