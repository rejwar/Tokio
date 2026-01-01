use tokio::time::{sleep, Duration};

async fn Fast() {
    println!("Fast done");
}

async fn Slow() {
    sleep(Duration::from_secs(1)).await;
    println!("Slow done");
}

#[tokio::main]
async fn main() {
    tokio::join!(Fast(), Slow());
}
