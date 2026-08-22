use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error("connection closed")]
    ConnectionClosed,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
