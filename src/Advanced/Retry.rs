use reqwest::Client;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    let client = Client::new();

    for i in 0..5 {
        let attempt = timeout(
            Duration::from_secs(2),
            client.get("https://example.com").send(),
        )
        .await;

        match attempt {
            Ok(Ok(_)) => {
                println!("Success");
                break;
            }
            _ => {
                println!("Retry {}", i + 1);
                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        }
    }
}
