use tokio::select;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    select! {
        _ = sleep(Duration::from_secs(1)) => println!("Fast task finished"),
        _ = sleep(Duration::from_secs(3)) => println!("Slow task finished"),
    }
}
