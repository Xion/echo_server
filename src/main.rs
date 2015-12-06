extern crate getopts;


use getopts::Options;
use std::env;


/// Print the instructions about invoking the program from the command line.
fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut options = Options::new();
    options.optflag("h", "help", "Show this usage message.");

    let args = match options.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };
    if args.opt_present("h") {
        print_usage(&program, options);
        return;
    }

    println!("Hello world");
}
