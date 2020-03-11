//using the standard library to spin up a simple web server
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("a connection was made");
    }
}
