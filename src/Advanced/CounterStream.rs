use futures::Stream;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

struct Counter {
    n: usize,
}

impl Stream for Counter {
    type Item = usize;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.n < 5 {
            let v = self.n;
            self.n += 1;
            Poll::Ready(Some(v))
        } else {
            Poll::Ready(None)
        }
    }
}

#[tokio::main]
async fn main() {
    use futures::StreamExt;

    let mut counter = Counter { n: 0 };

    while let Some(v) = counter.next().await {
        println!("Got {}", v);
    }
}
