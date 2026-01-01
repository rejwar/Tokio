use tokio::task;

#[tokio::main]
async fn main() {
    let mut tasks = vec![];

    for i in 1..=5 {
        tasks.push(task::spawn(async move {
            println!("Task {} running", i);
        }));
    }

    for t in tasks {
        t.await.unwrap();
    }
}
