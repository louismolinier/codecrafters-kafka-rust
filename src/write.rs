use std::io::Write;
use std::net::TcpStream;

const UNSUPPORTED_VERSION: u16 = 35;

pub fn write(message: Vec<u8>, mut stream: &TcpStream) {
    let mut response = Vec::new();
    let message_size: [u8; 4] = [0, 0, 0, 0];
    let request_api_key = &message[0..2];
    let _request_api_version = &message[2..4];
    let correlation_id = &message[4..8];
    response.extend(&message_size);
    response.extend(correlation_id);
    if i16::from_be_bytes([request_api_key[0], request_api_key[1]]) > 4 {
        response.extend(UNSUPPORTED_VERSION.to_be_bytes());
    }
    stream.write_all(&response).unwrap();
}
