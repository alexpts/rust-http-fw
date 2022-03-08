use std::net::TcpStream;
use std::io;
use std::io::{Read, Write};
use pts_fw::server;

static RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\ncontent-length: 1\r\n\r\n1";

fn handle_connection(mut stream: &TcpStream, buffer: &mut [u8; 512]) -> Result<usize, io::Error> {
    stream.read( buffer)?;
    return stream.write(RESPONSE);
}

fn main() {
    let mut server = server::http::new(handle_connection);
    server.bind("127.0.0.1:3000");
    server.start();
}

