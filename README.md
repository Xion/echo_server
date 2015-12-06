# Echo server

Simple echo server written in Rust.

```
$ cargo run -- -h
Usage: echo_server [options]

Options:
    -h, --help          Show this usage message
    -p, --port [PORT]   Port to listen on


$ cargo run -- -p 4444

# in other terminal window
$ nc localhost 4444
Hi!
Hi!
It works!
It works!
```
