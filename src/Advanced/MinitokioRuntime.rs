use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::{Duration, Instant},
};

use futures::task::{waker_ref, ArcWake};

/// ===============
/// Delay Future
/// ===============
struct Delay {
    when: Instant,
    waker: Arc<Mutex<Option<std::task::Waker>>>,
}

impl Delay {
    fn new(dur: Duration) -> Self {
        let when = Instant::now() + dur;
        let waker = Arc::new(Mutex::new(None));
        let waker_thread = waker.clone();

        thread::spawn(move || {
            thread::sleep(dur);
            if let Some(w) = &*waker_thread.lock().unwrap() {
                w.wake_by_ref();
            }
        });

        Delay { when, waker }
    }
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

/// ===============
/// Task
/// ===============
struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    queue: Arc<TaskQueue>,
}

struct TaskQueue {
    tasks: Mutex<VecDeque<Arc<Task>>>,
}

/// wake => push to queue
impl ArcWake for Task {
    fn wake_by_ref(task: &Arc<Self>) {
        task.queue.tasks.lock().unwrap().push_back(task.clone());
    }
}

/// ===============
/// Runtime (Mini Tokio)
/// ===============
struct Runtime {
    queue: Arc<TaskQueue>,
}

impl Runtime {
    fn new(worker_threads: usize) -> Self {
        let queue = Arc::new(TaskQueue {
            tasks: Mutex::new(VecDeque::new()),
        });

        for _ in 0..worker_threads {
            let q = queue.clone();
            thread::spawn(move || loop {
                let opt = q.tasks.lock().unwrap().pop_front();

                if let Some(task) = opt {
                    let waker = waker_ref(&task);
                    let mut cx = Context::from_waker(&*waker);

                    let mut slot = task.future.lock().unwrap();

                    if let Some(mut fut) = slot.take() {
                        if let Poll::Pending = fut.as_mut().poll(&mut cx) {
                            *slot = Some(fut);
                        }
                    }
                } else {
                    thread::yield_now();
                }
            });
        }

        Runtime { queue }
    }

    fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(fut))),
            queue: self.queue.clone(),
        });

        self.queue.tasks.lock().unwrap().push_back(task);
    }
}

/// ===============
/// DEMO MAIN
/// ===============
fn main() {
    let rt = Runtime::new(4);

    for i in 1..=5 {
        rt.spawn(async move {
            println!("Task {} started", i);
            Delay::new(Duration::from_millis(500)).await;
            println!("Task {} finished", i);
        });
    }

    thread::sleep(Duration::from_secs(3));
}
