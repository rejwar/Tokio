use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    let res = timeout(Duration::from_secs(1), sleep(Duration::from_secs(5))).await;

    if res.is_err() {
        println!("Timed out");
    }
}
