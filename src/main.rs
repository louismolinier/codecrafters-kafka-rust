#![allow(unused_imports)]
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut message_size: [u8; 4] = [0, 0, 0, 0];
                stream.read(&mut message_size).unwrap();
                let size = i32::from_be_bytes(message_size) as usize;
                let mut message = vec![0; size];
                stream.read_exact(&mut message).unwrap();

                let mut response = Vec::new();
                response.extend(&message_size);
                response.extend(&message[4..8]);
                stream.write_all(&response).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
