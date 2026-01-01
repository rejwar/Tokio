use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let child_token = token.child_token();

    let handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = child_token.cancelled() => {
                    println!("Task cancelled");
                    break;
                }
                _ = sleep(Duration::from_millis(500)) => {
                    println!("Working...");
                }
            }
        }
    });

    sleep(Duration::from_secs(2)).await;
    token.cancel();

    handle.await.unwrap();
}
