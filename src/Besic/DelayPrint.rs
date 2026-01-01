use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Waiting...");
    sleep(Duration::from_secs(1)).await;
    println!("Done");
}
