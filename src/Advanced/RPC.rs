use futures::future::join_all;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let addresses = vec![
        "0x0000000000000000000000000000000000000000",
        "0x1111111111111111111111111111111111111111",
    ];

    let futures = addresses.into_iter().map(|addr| {
        let client = client.clone();

        async move {
            let body = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_getBalance",
                "params": [addr, "latest"]
            });

            let res = client
                .post("https://rpc.ankr.com/eth")
                .json(&body)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            println!("{} => {}", addr, res);
        }
    });

    join_all(futures).await;
}
