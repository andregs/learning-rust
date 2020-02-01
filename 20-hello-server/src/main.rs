// building a single-threaded web server

use std::fs;
use std::io::prelude::*;
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

    // now execute "cargo run" and browse to http://127.0.0.1:7878
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // to debug request details
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n"; // byte string syntax

    let (status, filename) = if buffer.starts_with(get) {
        ("200 OK", "hello.html")
    } else {
        ("400 BAD REQUEST", "400.html")
    };
    
    let body = fs::read_to_string(filename).unwrap();
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, body);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
