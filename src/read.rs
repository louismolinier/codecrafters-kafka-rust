use std::io::Read;
use std::net::TcpStream;

pub fn read(mut stream: &TcpStream) -> Vec<u8> {
    let mut message_size: [u8; 4] = [0, 0, 0, 0];
    stream.read_exact(&mut message_size).unwrap();
    let size = i32::from_be_bytes(message_size) as usize;
    let mut message = vec![0; size];
    stream.read_exact(&mut message).unwrap();
    return message;
}
