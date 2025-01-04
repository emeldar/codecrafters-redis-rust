#![allow(unused_imports)]
use std::net::TcpListener;
use std::io::Write;
use std::io::Read;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    loop {
        for stream in listener.incoming() {
            thread::spawn(|| {
                match stream {
                    Ok(mut _stream) => {
                        loop {
                            let mut buf = vec![0;1024];
                            let _ = _stream.read(&mut buf);
                            let string_buf = String::from_utf8(buf).unwrap();
        
                            let parts = string_buf.split("\r\n");
        
                            for part in parts {
                                if part == "PING" {
                                    _stream.write_all(b"+PONG\r\n").unwrap();
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
            });
        }
    }
}
