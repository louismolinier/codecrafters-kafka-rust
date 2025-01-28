#![allow(unused_imports)]

use std::net::TcpListener;

mod read;
mod write;

use read::read;
use write::write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let message = read(&stream);
                write(message, &stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
