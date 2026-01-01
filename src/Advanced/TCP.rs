use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").await.unwrap();

    let (mut socket, _) = listener.accept().await.unwrap();

    let mut buf = vec![0; 1024];
    let n = socket.read(&mut buf).await.unwrap();

    println!("Received = {}", String::from_utf8_lossy(&buf[..n]));
}
