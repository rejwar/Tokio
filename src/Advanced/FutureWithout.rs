async fn Demo() {
    println!("Inside");
}

#[tokio::main]
async fn main() {
    let _f = Demo(); // nothing happens
    println!("Program finished");
}
