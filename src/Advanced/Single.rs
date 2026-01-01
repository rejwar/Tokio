use tokio::signal;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        loop {
            println!("Server running...");
            sleep(Duration::from_secs(1)).await;
        }
    });

    signal::ctrl_c().await.unwrap();

    println!("Shutting down gracefully");
}
