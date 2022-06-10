// use bytes::{Buf, BytesMut};
use tokio::{
    net::{TcpListener, TcpStream},
    stream::StreamExt, io::AsyncWriteExt,
};

async fn handle_connection(stream: &mut TcpStream) {
    loop {
        // let mut data = BytesMut::new();

        let mut response = "+PONG/t/n".as_bytes();
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
