use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error("connection closed")]
    ConnectionClosed,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
}
