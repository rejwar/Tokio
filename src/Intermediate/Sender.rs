use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(10);

    let mut r1 = tx.subscribe();
    let mut r2 = tx.subscribe();

    tokio::spawn(async move {
        tx.send("Hello world").unwrap();
    });

    println!("Receiver1: {}", r1.recv().await.unwrap());
    println!("Receiver2: {}", r2.recv().await.unwrap());
}
