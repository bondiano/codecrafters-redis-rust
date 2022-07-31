mod command;

use bytes::{BytesMut};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};

use command::{ handle_command };

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream.unwrap();
        tokio::spawn(async move {
            handle_connection(&mut stream).await;
        });
    }
}

async fn handle_connection(stream: &mut TcpStream) {
    loop {
        let mut command = BytesMut::new();
        stream.read_buf(&mut command).await.unwrap();

        let mut response = handle_command(&command).await.unwrap();

        stream.write_buf(&mut response).await.unwrap();
    }
}
