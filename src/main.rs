use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to socket");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    stream.read(&mut buffer).expect("Error during stream read");
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 97\r\n\r\n<html><head><title>Example test page</title></head><body><h1>You made it here!</h1></body></html>";

    stream.write(response.as_bytes()).expect("Error during stream write");
    stream.flush().unwrap();
}
