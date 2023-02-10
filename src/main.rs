mod command;
mod storage;

use bytes::BytesMut;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, self},
    net::{TcpListener, TcpStream},
    stream::StreamExt,
};

use command::handle_command;
use storage::Storage;

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let mut incoming = listener.incoming();

    let storage = Storage::new();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream.unwrap();
        let mut storage_clone = storage.clone();
        tokio::spawn(async move {
            match handle_connection(&mut stream, &mut storage_clone).await {
                Ok(_) => println!("Connection closed"),
                Err(e) => println!("Connection error: {}", e),
            };
        });
    }
}

async fn handle_connection(stream: &mut TcpStream, storage: &mut Storage) -> io::Result<()> {
    loop {
        let mut command = BytesMut::new();
        stream.read_buf(&mut command).await?;

        let mut response = handle_command(&command, storage);

        stream.write_buf(&mut response).await?;
    }
}
