use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let result: Vec<_> = stream::iter(v)
        .map(|x| async move { x * 2 })
        .buffer_unordered(2)
        .collect()
        .await;

    println!("{:?}", result);
}
