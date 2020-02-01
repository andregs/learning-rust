// building a single-threaded web server

use std::io::prelude::*; // r/w streams
use std::net::{TcpListener, TcpStream};

fn main() {
    // binding might fail, so it returns a Result<T, E>
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();

    // a single stream represents one connection attempt between client & server
    for stream in listener.incoming() {
        // connection might fail due to OS reasons, like exceeding max connections
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    // we already can "cargo run" and browse to http://127.0.0.1:7878
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // to debug request details
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // two blank lines for headers and body
    let response = "HTTP/1.1 204 No Content\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
