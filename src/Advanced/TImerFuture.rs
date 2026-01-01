use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

struct Delay {
    when: Instant,
    state: Arc<Mutex<Option<Waker>>>,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready("Timer finished")
        } else {
            let mut lock = self.state.lock().unwrap();
            *lock = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl Delay {
    fn new(dur: Duration) -> Self {
        let when = Instant::now() + dur;
        let state = Arc::new(Mutex::new(None));
        let thread_state = state.clone();

        thread::spawn(move || {
            let now = Instant::now();
            if when > now {
                thread::sleep(when - now);
            }

            if let Some(waker) = &*thread_state.lock().unwrap() {
                waker.wake_by_ref();
            }
        });

        Delay { when, state }
    }
}

fn main() {
    let future = Delay::new(Duration::from_secs(1));

    futures::executor::block_on(async {
        let result = future.await;
        println!("{}", result);
    });
}
