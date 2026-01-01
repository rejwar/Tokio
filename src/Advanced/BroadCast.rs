use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(10);

    let mut r1 = tx.subscribe();
    let mut r2 = tx.subscribe();

    tx.send(100).unwrap();

    println!("r1 = {}", r1.recv().await.unwrap());
    println!("r2 = {}", r2.recv().await.unwrap());
}
