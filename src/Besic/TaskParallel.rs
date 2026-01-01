use tokio::task;

#[tokio::main]
async fn main() {
    let t1 = task::spawn(async {
        println!("Task 1 running");
    });

    let t2 = task::spawn(async {
        println!("Task 2 running");
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
