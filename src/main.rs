#![allow(unused_imports)]
use std::net::TcpListener;
use std::io::Write;
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buf = vec![0;1024];
                let _ = _stream.read(&mut buf);
                let string_buf = String::from_utf8(buf).unwrap();
                println!("{}", string_buf);

                let parts = string_buf.split("\n");

                for part in parts {
                    println!("{}", part);
                    if part.eq("PING") {
                        _stream.write_all(b"+PONG\r\n").unwrap();
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
