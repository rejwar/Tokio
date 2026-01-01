use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send("done").unwrap();
    });

    println!("{}", rx.await.unwrap());
}
