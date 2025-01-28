use std::io::Write;
use std::net::TcpStream;

const UNSUPPORTED_VERSION: u16 = 35;

pub fn write(message: Vec<u8>, mut stream: &TcpStream) {
    let request_api_key = &message[0..2];
    let request_api_version = &message[2..4];
    let correlation_id = &message[4..8];
    let _body = &message[8..];

    let mut data = Vec::new();
    data.extend(correlation_id);
    if i16::from_be_bytes([request_api_version[0], request_api_version[1]]) > 4 {
        data.extend(UNSUPPORTED_VERSION.to_be_bytes());
    } else {
        data.extend((0 as u16).to_be_bytes());
    }
    if i16::from_be_bytes([request_api_key[0], request_api_key[1]]) == 18 {
        data.extend((2 as u8).to_be_bytes());
        data.extend((18 as u16).to_be_bytes());
        data.extend((0 as u16).to_be_bytes());
        data.extend((4 as u16).to_be_bytes());
        data.extend((0 as u8).to_be_bytes());
        data.extend((420 as u32).to_be_bytes());
        data.extend((0 as u8).to_be_bytes());
    }
    let size = data.len() as u32;
    let message_size: [u8; 4] = size.to_be_bytes();
    let mut reponse = Vec::from(message_size);
    reponse.extend(&data);

    stream.write_all(&data).unwrap();
}
