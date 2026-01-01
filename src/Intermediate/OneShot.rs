use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("Single message").unwrap();
    });

    let msg = rx.await.unwrap();
    println!("{}", msg);
}
