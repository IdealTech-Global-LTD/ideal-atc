use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

use ws_broadcaster::{RadarBroadcaster, RadarMessage};

use crate::error::WebSocketError;

pub async fn handle_session(
    mut websocket: WebSocketStream<TcpStream>,
    broadcaster: Arc<RadarBroadcaster>,
) -> Result<(), WebSocketError> {
    websocket
        .send(Message::text(r#"{"type":"connected","version":"1.0"}"#))
        .await?;

    let mut receiver = broadcaster.subscribe();

    loop {
        tokio::select! {

            Some(incoming) = websocket.next() => {
                let message = incoming?;

                if message.is_close() {
                    break;
                }
            }

            Ok(radar_message) = receiver.recv() => {
                let json = serde_json::to_string(&radar_message)?;

                websocket.send(Message::text(json)).await?;
            }
        }
    }

    Ok(())
}
