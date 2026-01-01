suse tokio::sync::Notify;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let n2 = notify.clone();

    tokio::spawn(async move {
        println!("Waiting...");
        n2.notified().await;
        println!("Got signal");
    });

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    notify.notify_one();
}
