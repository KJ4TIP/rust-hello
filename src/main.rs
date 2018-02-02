use std::fs::File;
use std::vec::Vec;
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

    let response = build_response(
"<html>
    <head>
        <title>Example test page</title>
    </head>
    <body>
        <h1>You made it here!</h1>
    </body>
</html>
");

    stream.write(&response).expect("Error during stream write");
    stream.flush().unwrap();
}

fn build_response(contents: &str) -> Vec<u8> {
    let mut response = String::from("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: ");

    response.push_str(&contents.len().to_string());
    response.push_str("\r\n\r\n");
    response.push_str(contents);
    
    return response.into_bytes();
}
