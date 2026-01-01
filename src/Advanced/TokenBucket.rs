use tokio::time::{sleep, Duration, Instant};

#[tokio::main]
async fn main() {
    let mut last = Instant::now();

    for i in 1..=5 {
        let elapsed = last.elapsed();

        if elapsed < Duration::from_millis(500) {
            sleep(Duration::from_millis(500) - elapsed).await;
        }

        println!("Request {}", i);
        last = Instant::now();
    }
}
