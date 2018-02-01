use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to socket");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Received connection");
    }
}
