use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(5);

    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    println!("{}", rx.recv().await.unwrap());
}
