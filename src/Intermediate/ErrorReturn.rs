use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    let result = timeout(Duration::from_secs(1), sleep(Duration::from_secs(3))).await;

    match result {
        Ok(_) => println!("Completed"),
        Err(_) => println!("Timeout happened"),
    }
}
