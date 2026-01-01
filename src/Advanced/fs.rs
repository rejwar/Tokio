use tokio::fs;

#[tokio::main]
async fn main() {
    let data = fs::read_to_string("test.txt").await.unwrap();
    println!("{}", data);
}
