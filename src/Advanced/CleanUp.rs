use tokio::time::{sleep, Duration};

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        println!("Cleanup executed before task exit");
    }
}

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        let _guard = Cleanup;
        sleep(Duration::from_secs(5)).await;
    });

    sleep(Duration::from_millis(500)).await;
    handle.abort();
}
