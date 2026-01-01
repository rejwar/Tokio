use futures::task::{waker_ref, ArcWake};
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::{Duration, Instant},
};

struct Delay {
    when: Instant,
    waker: Arc<Mutex<Option<std::task::Waker>>>,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            *self.waker.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl Delay {
    fn new(dur: Duration) -> Self {
        let when = Instant::now() + dur;
        let waker = Arc::new(Mutex::new(None));
        let thread_waker = waker.clone();

        thread::spawn(move || {
            thread::sleep(dur);
            if let Some(w) = &*thread_waker.lock().unwrap() {
                w.wake_by_ref();
            }
        });

        Delay { when, waker }
    }
}

struct Executor;
impl Executor {
    fn block_on<F: Future<Output = ()>>(mut f: F) {
        use futures::task::noop_waker_ref;
        let waker = noop_waker_ref();
        let mut cx = Context::from_waker(waker);

        let mut f = unsafe { Pin::new_unchecked(&mut f) };

        loop {
            match f.as_mut().poll(&mut cx) {
                Poll::Ready(()) => break,
                Poll::Pending => thread::yield_now(),
            }
        }
    }
}

fn main() {
    Executor::block_on(async {
        println!("Waiting 1s…");
        Delay::new(Duration::from_secs(1)).await;
        println!("Done");
    });
}
