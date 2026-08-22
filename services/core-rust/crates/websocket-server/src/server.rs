use std::sync::Arc;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use ws_broadcaster::RadarBroadcaster;

use crate::{error::WebSocketError, handle_session};

pub struct WebSocketServer {
    broadcaster: Arc<RadarBroadcaster>,
}

impl WebSocketServer {
    pub fn new(broadcaster: Arc<RadarBroadcaster>) -> Self {
        Self { broadcaster }
    }

    pub fn broadcaster(&self) -> &Arc<RadarBroadcaster> {
        &self.broadcaster
    }

    pub async fn run(&self, address: &str) -> Result<(), WebSocketError> {
        let listener = TcpListener::bind(address).await?;

        tracing::info!("WebSocket server listening on {}", address);

        loop {
            let (stream, peer_address) = listener.accept().await?;

            tracing::info!("WebSocket connection from {}", peer_address);

            let websocket = accept_async(stream).await?;

            tracing::info!("WebSocket handshake completed for {}", peer_address);

            tokio::spawn(async move {
                if let Err(error) = handle_session(websocket).await {
                    tracing::error!("Session error: {}", error);
                }
            });
        }
    }

    pub async fn bind(&self, address: &str) -> Result<TcpListener, WebSocketError> {
        let listener = TcpListener::bind(address).await?;

        tracing::info!("WebSocket server listening on {}", address);

        Ok(listener)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use ws_broadcaster::RadarBroadcaster;

    #[test]
    fn creates_server() {
        let broadcaster = Arc::new(RadarBroadcaster::new());

        let server = WebSocketServer::new(Arc::clone(&broadcaster));

        assert!(Arc::ptr_eq(server.broadcaster(), &broadcaster));
    }
}
