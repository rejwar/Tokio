#[tokio::main]
async fn main() {
    let h = tokio::spawn(async {
        panic!("task failed");
    });

    let res = h.await;

    println!("Result = {:?}", res);
}
