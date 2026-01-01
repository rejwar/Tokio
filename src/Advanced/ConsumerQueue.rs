use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    for id in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            for j in 0..3 {
                tx.send((id, j)).await.unwrap();
            }
        });
    }

    drop(tx);

    while let Some(msg) = rx.recv().await {
        println!("Received {:?}", msg);
    }
}
