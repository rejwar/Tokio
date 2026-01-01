use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let c = Arc::new(Mutex::new(0));
    let mut tasks = vec![];

    for _ in 0..100 {
        let cc = c.clone();
        tasks.push(tokio::spawn(async move {
            let mut n = cc.lock().await;
            *n += 1;
        }));
    }

    for t in tasks {
        t.await.unwrap();
    }

    println!("Result = {}", *c.lock().await);
}
