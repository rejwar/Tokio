use std::cmp::Reverse;
use std::collections::BinaryHeap;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((3, "Task-3")));
    heap.push(Reverse((1, "Task-1")));
    heap.push(Reverse((2, "Task-2")));

    while let Some(Reverse((delay, name))) = heap.pop() {
        sleep(Duration::from_secs(delay)).await;
        println!("Ran {}", name);
    }
}
