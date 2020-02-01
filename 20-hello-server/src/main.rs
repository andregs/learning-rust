// building a single-threaded web server

use std::net::TcpListener;

fn main() {
    // binding might fail, so it returns a Result<T, E>
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();

    // a single stream represents one connection attempt between client & server
    for stream in listener.incoming() {
        // connection might fail due to OS reasons, like exceeding max connections
        let _stream = stream.unwrap();
        println!("Connection established!");
    }

    // we already can "cargo run" and browse to http://127.0.0.1:7878
    // depending on your browser you'll see one or more "Connection established!" on the console output
}
