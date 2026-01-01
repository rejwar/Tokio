async fn SayHello() {
    println!("Hello world async!");
}

#[tokio::main]
async fn main() {
    SayHello().await;
}
