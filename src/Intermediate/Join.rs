use tokio::join;

async fn Fast() {
    println!("Fast");
}

async fn Slow() {
    println!("Slow");
}

#[tokio::main]
async fn main() {
    join!(Fast(), Slow());
}
