use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Inside spawned task");
    });

    println!("Main continues...");
    handle.await.unwrap();
}
