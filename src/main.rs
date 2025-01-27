#![allow(unused_imports)]
use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 7];
                _stream.write(&buf).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
