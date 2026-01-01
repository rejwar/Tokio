use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();

    stream.write_all(b"Hello").await.unwrap();

    let mut buf = vec![0; 5];
    stream.read_exact(&mut buf).await.unwrap();

    println!("Received {:?}", String::from_utf8_lossy(&buf));
}
