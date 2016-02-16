extern crate getopts;


use getopts::Options;
use std::env;
use std::io::{self, Read, Write};
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

    let args = options.parse(&argv[1..]).unwrap();
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
        if let Err(err) = stream.read(&mut buffer) {
            if err.kind() == io::ErrorKind::Interrupted { continue; }
            if is_conn_broken_err(&err) { break; }
            panic!(err.to_string());
        }
        if let Err(err) = stream.write(&buffer) {
            if err.kind() == io::ErrorKind::Interrupted { continue; }
            if is_conn_broken_err(&err) { break; }
            panic!(err.to_string());
        }
    }
}

/// Checks whether given I/O error represents a broken TCP connection.
fn is_conn_broken_err(err: &io::Error) -> bool {
    match err.kind() {
        io::ErrorKind::BrokenPipe |
        io::ErrorKind::ConnectionAborted |
        io::ErrorKind::ConnectionReset => true,
        _ => false,
    }
}
