use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_blockNumber",
        "params": []
    });

    let res = client
        .post("https://rpc.ankr.com/eth")
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    println!("Block number hex = {}", res["result"]);
}
