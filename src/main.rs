use bytes::{BytesMut};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};

async fn handle_connection(stream: &mut TcpStream) {
    loop {
        let mut data = BytesMut::new();
        stream.read_buf(&mut data).await.unwrap();

        println!("{}", String::from_utf8_lossy(&data));

        let mut response = "+PONG\r\n".as_bytes();
        stream.write_buf(&mut response).await.unwrap();
    }
}

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
