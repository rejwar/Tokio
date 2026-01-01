use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let c = counter.clone();
        let h = tokio::spawn(async move {
            let mut n = c.lock().await;
            *n += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.await.unwrap();
    }

    println!("Result = {}", *counter.lock().await);
}
