use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut file = File::open("input.txt").await.unwrap();
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();

    socket.write_all(&buf).await.unwrap();
}
