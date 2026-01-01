use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut f = File::open("big.txt").await.unwrap();
    let mut buf = vec![0; 1024];
    let mut total = 0usize;

    loop {
        let n = f.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }
        total += n;
    }

    println!("Total bytes = {}", total);
}
