use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let address = "0x0000000000000000000000000000000000000000";

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_getBalance",
        "params": [address, "latest"]
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

    println!("Balance (wei hex) = {}", res["result"]);
}
