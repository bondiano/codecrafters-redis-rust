#[allow(unused_imports)]
use std::env;

#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;

#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()>{
    let mut buffer = vec![];
    loop {
        match stream.read(&mut buffer) {
            Ok(_) => {
                // let request = String::from_utf8_lossy(&buffer);
                stream.write("+PONG\r\n".as_bytes())?;


                // if request.contains("PING") {
                //     stream.write("+PONG\r\n".as_bytes())?;

                //     break;
                // } else {
                //     println!("{}", request);
                //     stream.write("-\r\n".as_bytes())?;
                // }
                // println!("{}", String::from_utf8_lossy(&buffer[..]));
                // stream.write(&buffer)?;
            }
            Err(e) => {
                println!("Error: {}", e);
                // return Err(e);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        handle_client(&mut stream?)?;
    }

    Ok(())
}
