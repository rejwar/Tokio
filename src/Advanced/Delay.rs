#[tokio::main]
async fn main() {
    let h = tokio::spawn(async { 10 });

    let v = h.await.unwrap();
    println!("Value = {}", v);
}
