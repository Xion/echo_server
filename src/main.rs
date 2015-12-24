extern crate getopts;


use getopts::Options;
use std::env;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;


const DEFAULT_PORT: u16 = 7;
const BUFFER_SIZE: usize = 256;


fn main() {
    let argv: Vec<String> = env::args().collect();
    let program = argv[0].clone();

    let mut options = Options::new();
    options.optflag("h", "help", "Show this usage message");
    options.optopt("p", "port", "Port to listen on", "PORT");

    let args = options.parse(&argv[1..])
        .unwrap_or_else(|e| panic!(e.to_string()));
    if args.opt_present("h") {
        print_usage(&program, options);
        return;
    }

    let port = args.opt_str("p").unwrap_or(DEFAULT_PORT.to_string())
        .parse::<u16>().expect(&format!("Invalid port number"));

    let listener = listen(port)
        .expect(&format!("Cannot bind to port {}", port));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                // TODO(xion): proper logging
                writeln!(io::stderr(),
                    "Failed to accept incoming connection: {:?}", e).unwrap();
            }
        }
    }
    drop(listener);
}


/// Print the instructions about invoking the program from the command line.
fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}


/// Create a TCP listener for given port.
fn listen(port: u16) -> io::Result<TcpListener> {
    TcpListener::bind(("0.0.0.0", port))
}

/// Handle the client of an echo server,
/// sending back whatever we receive from them.
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        if !stream.read(&mut buffer).is_ok()  {
            // TODO(xion): log error if it's other than EOF/broken pipe/etc.
            return;
        }
        if !stream.write(&buffer).is_ok() {
            // TODO(xion): log the error?
            return;
        }
    }
}
