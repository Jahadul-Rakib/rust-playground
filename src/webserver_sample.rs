use std::io::{Read, Write};
use std::net::TcpStream;

pub fn test() {
    let tcp_listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => handler(stream),
            Err(e) => println!("{}", e.to_string()),
        }
    }
}

fn handler(mut stream: TcpStream) {
    let mut reader = [0; 1024];
    stream.read(&mut reader).unwrap();

    println!("{}", String::from_utf8(reader.to_vec()).unwrap());

    stream.write("{welcome}".as_bytes()).unwrap();
    stream.flush().unwrap();
}
