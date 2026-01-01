#[tokio::main]
async fn main() {
    let result = tokio::task::spawn_blocking(|| {
        // heavy CPU work
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    })
    .await
    .unwrap();

    println!("Sum = {}", result);
}
