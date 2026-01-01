use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(5);

    tokio::spawn(async move {
        tx.send("Hello").await.unwrap();
    });

    let msg = rx.recv().await.unwrap();
    println!("Received: {}", msg);
}
