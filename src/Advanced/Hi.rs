use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let limit = Arc::new(Semaphore::new(3));

    let mut handles = vec![];

    for i in 1..=10 {
        let permit = limit.clone().acquire_owned().await.unwrap();

        handles.push(tokio::spawn(async move {
            println!("Running task {}", i);
            drop(permit);
        }));
    }

    for h in handles {
        h.await.unwrap();
    }
}
