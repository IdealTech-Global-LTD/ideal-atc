use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

use crate::error::WebSocketError;

pub async fn handle_session(
    mut websocket: WebSocketStream<TcpStream>,
) -> Result<(), WebSocketError> {
    websocket
        .send(Message::Text(
            r#"{"type":"connected","version":"1.0"}"#.into(),
        ))
        .await?;

    while let Some(message) = websocket.next().await {
        let message = message?;

        if message.is_close() {
            break;
        }
    }

    Ok(())
}
