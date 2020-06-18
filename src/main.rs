mod data;
mod http;
mod server;

use clap::{App, Arg};
use std::net;

const ARG_STATIC_DIRECTORY: &str = "STATIC_DIRECTORY";
const ARG_HOST: &str = "HOST";
const ARG_PORT: &str = "PORT";

fn main() {
    let matches = App::new("yaserver")
        .version("0.1.0")
        .about("Yet Another Server")
        .arg(
            Arg::with_name(ARG_STATIC_DIRECTORY)
                .long("static_directory")
                .required(false)
                .takes_value(true)
                .help("The path to serve statically"),
        )
        .arg(
            Arg::with_name(ARG_HOST)
                .required(true)
                .long("port")
                .takes_value(true)
                .default_value("127.0.0.1")
                .help("The host to serve on"),
        )
        .arg(
            Arg::with_name(ARG_PORT)
                .required(true)
                .long("host")
                .takes_value(true)
                .default_value("8000")
                .help("The port to serve on"),
        )
        .get_matches();

    let server = server::Server::new(
        String::from(matches.value_of(ARG_HOST).unwrap()),
        String::from(matches.value_of(ARG_PORT).unwrap()),
    );

    let address = server.get_address();

    println!("Serving on {}", address);

    let listener = std::net::TcpListener::bind(address).unwrap();

    for stream_res in listener.incoming() {
        let stream: net::TcpStream = match stream_res {
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
