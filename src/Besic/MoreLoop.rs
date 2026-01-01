use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    for i in 1..=3 {
        sleep(Duration::from_millis(500)).await;
        println!("Step {}", i);
    }
}
