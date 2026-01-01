use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    for i in 0..20 {
        tx.send(i).await.unwrap();
    }
    drop(tx);

    let mut workers = Vec::new();

    for id in 0..4 {
        let mut rx = rx.clone();
        workers.push(tokio::spawn(async move {
            while let Some(v) = rx.recv().await {
                println!("Worker {} got {}", id, v);
            }
        }));
    }

    for w in workers {
        w.await.unwrap();
    }
}
