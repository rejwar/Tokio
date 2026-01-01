use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut f = File::create("log.txt").await.unwrap();
    f.write_all(b"Tokio async write example").await.unwrap();
}
