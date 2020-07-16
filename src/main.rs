mod data;
mod http;
mod router;
mod server;
mod thread;

use clap::{App, Arg};
use ctrlc;
use std::process;

const ARG_STATIC_DIRECTORY: &str = "STATIC_DIRECTORY";
const ARG_HOST: &str = "HOST";
const ARG_PORT: &str = "PORT";
const ARG_THREAD_COUNT: &str = "THREAD_COUNT";

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
        .arg(
            Arg::with_name(ARG_THREAD_COUNT)
                .required(true)
                .long("thread_count")
                .takes_value(true)
                .default_value("16")
                .help("The number of threads to spin up"),
        )
        .get_matches();

    ctrlc::set_handler(move || {
        println!("Exiting");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let server = server::Server::new(
        String::from(matches.value_of(ARG_HOST).unwrap()),
        String::from(matches.value_of(ARG_PORT).unwrap()),
        matches
            .value_of(ARG_THREAD_COUNT)
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        server::ServerParams::new(String::from(
            matches.value_of(ARG_STATIC_DIRECTORY).unwrap_or(""),
        )),
    );

    server.serve();
}
