use tokio::sync::mpsc::{self, error::TrySendError};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<u32>(2);

    tokio::spawn(async move {
        for i in 0..10 {
            loop {
                match tx.try_send(i) {
                    Ok(_) => break,
                    Err(TrySendError::Full(_)) => {
                        tokio::task::yield_now().await;
                    }
                    Err(_) => return,
                }
            }
        }
    });

    while let Some(v) = rx.recv().await {
        println!("Got {}", v);
    }
}
