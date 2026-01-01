use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(5);

    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(i).await.unwrap();
        }
    });

    while let Some(v) = rx.recv().await {
        println!("Got {}", v);
    }
}
