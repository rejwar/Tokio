#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 1..=10 {
        handles.push(tokio::spawn(async move {
            println!("Task {}", i);
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    println!("Done");
}
