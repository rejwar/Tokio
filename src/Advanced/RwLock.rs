use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let data = Arc::new(RwLock::new(5));

    // reader
    {
        let d = data.read().await;
        println!("Read {}", *d);
    }

    // writer
    {
        let mut d = data.write().await;
        *d = 10;
    }
}
