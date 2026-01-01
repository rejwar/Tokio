use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct MyFuture {
    done: bool,
}

impl Future for MyFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.done {
            Poll::Ready("Finished")
        } else {
            self.done = true;
            Poll::Pending
        }
    }
}
