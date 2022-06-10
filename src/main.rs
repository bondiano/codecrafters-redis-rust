#[allow(unused_imports)]
use std::env;

#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;

#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()>{
    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        let request = String::from_utf8_lossy(&buffer);

        if request.contains("PING") {
            stream.write("+PONG\r\n".as_bytes())?;
        } else {
            stream.write("-ERR\r\n".as_bytes())?;
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    match listener.accept() {
        Ok((mut socket, addr)) => {
                println!("accepted new client: {addr:?}");

                handle_client(&mut socket)?;
        },
        Err(e) => println!("couldn't accept client: {:?}", e),
    }

    Ok(())
}
