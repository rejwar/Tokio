use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let tasks = stream::iter(1..=5).map(|i| async move {
        println!("Processing {}", i);
        i * 2
    });

    let results: Vec<_> = tasks.buffer_unordered(2).collect().await;

    println!("Results = {:?}", results);
}
