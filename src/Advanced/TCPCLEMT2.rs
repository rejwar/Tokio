use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut s = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    s.write_all(b"hello").await.unwrap();

    let mut buf = [0; 5];
    s.read_exact(&mut buf).await.unwrap();

    println!("{:?}", String::from_utf8_lossy(&buf));
}
