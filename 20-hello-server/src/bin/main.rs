// building a multi-threaded web server
// note this file is under src/bin/main.rs and we also have src/lib.rs
// that means the primary create in our dir is the library

use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use hello_server::ThreadPool;

fn main() {
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // now execute "cargo run" and browse to http://127.0.0.1:7878 to get a page quickly
    // now execute "cargo run" and browse to http://127.0.0.1:7878/sleep to get a page after some seconds
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // to debug request details
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        ("200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "hello.html")
    } else {
        ("400 BAD REQUEST", "400.html")
    };
    
    let body = fs::read_to_string(filename).unwrap();
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, body);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
