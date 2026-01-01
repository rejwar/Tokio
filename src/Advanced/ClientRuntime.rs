use futures::StreamExt;
use serde_json::json;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::connect_async;

/// =======================
/// JSON-RPC async requester
/// =======================
async fn rpc_call(method: &str) {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": []
    });

    let client = reqwest::Client::new();

    let res = timeout(
        Duration::from_secs(3),
        client.post("https://rpc.ankr.com/eth").json(&body).send(),
    )
    .await;

    match res {
        Ok(Ok(resp)) => {
            let value: serde_json::Value = resp.json().await.unwrap();
            println!("RPC {} => {}", method, value);
        }
        _ => println!("Timeout or network error"),
    }
}

/// =======================
/// WebSocket subscription
/// =======================
async fn subscribe_new_blocks() {
    let (mut ws, _) = connect_async("wss://eth.llamarpc.com")
        .await
        .expect("connect failed");

    let sub = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method":"eth_subscribe",
        "params":["newHeads"]
    });

    ws.send(tokio_tungstenite::tungstenite::Message::Text(
        sub.to_string(),
    ))
    .await
    .unwrap();

    println!("Subscribed. Listening...");

    while let Some(msg) = ws.next().await {
        let data = msg.unwrap().into_text().unwrap();
        println!("Block event => {}", data);
    }
}

/// =======================
/// MAIN
/// =======================
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        loop {
            rpc_call("eth_blockNumber").await;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    subscribe_new_blocks().await;
}
