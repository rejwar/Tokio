use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    stream.write_all(input.as_bytes()).await.unwrap();
}
