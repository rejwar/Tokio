#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(async {
        println!("runs but no extra threads");
    })
    .await
    .unwrap();
}
