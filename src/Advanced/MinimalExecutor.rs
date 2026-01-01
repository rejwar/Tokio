use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

struct Executor {
    tasks: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    waker_cache: Mutex<Option<Waker>>,
    queue: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(fut))),
            waker_cache: Mutex::new(None),
            queue: self.tasks.clone(),
        });

        self.tasks.lock().unwrap().push_back(task);
    }

    fn run(&self) {
        while let Some(task) = self.tasks.lock().unwrap().pop_front() {
            let waker = futures::task::waker(task.clone());
            let mut cx = Context::from_waker(&waker);

            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                match future.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {}
                    Poll::Pending => {
                        *future_slot = Some(future);
                    }
                }
            }
        }
    }
}

impl futures::task::ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.queue.lock().unwrap().push_back(arc_self.clone());
    }
}

async fn ExampleTask(i: u32) {
    println!("Task {} running", i);
}

fn main() {
    let exec = Executor::new();

    exec.spawn(ExampleTask(1));
    exec.spawn(ExampleTask(2));

    exec.run();
}
