use futures::{Stream, StreamExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

struct WsStream {
    inner: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
}

impl Stream for WsStream {
    type Item = String;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match futures::ready!(Pin::new(&mut self.inner).poll_next(cx)) {
            Some(Ok(Message::Text(t))) => Poll::Ready(Some(t)),
            _ => Poll::Ready(None),
        }
    }
}

#[tokio::main]
async fn main() {
    let (ws, _) = connect_async("wss://echo.websocket.events").await.unwrap();

    let mut stream = WsStream { inner: ws };

    while let Some(msg) = stream.next().await {
        println!("Event = {}", msg);
    }
}
