use tokio::time::{sleep, timeout, Duration};

async fn Unreliable() {
    sleep(Duration::from_secs(2)).await;
}

#[tokio::main]
async fn main() {
    for _ in 0..3 {
        let result = timeout(Duration::from_secs(1), Unreliable()).await;

        if result.is_ok() {
            println!("Success");
            return;
        } else {
            println!("Retrying...");
        }
    }

    println!("Failed after retries");
}
